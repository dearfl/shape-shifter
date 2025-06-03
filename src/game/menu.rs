use bevy::{
    ecs::{relationship::RelatedSpawner, spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
};

use super::{GameState, Selected};

// a marker component used for cleanup
#[derive(Debug, Clone, Copy, Component)]
pub(crate) struct Menu;

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub(crate) struct NewGameButton;

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub(crate) struct SettingsButton;

#[derive(Debug, Clone, Copy, Component)]
#[require(Button)]
pub(crate) struct ExitButton;

const COLOR_NORMAL: Color = Color::srgb(0.8, 0.8, 0.8);
const COLOR_HOVERED: Color = Color::WHITE;

#[derive(Debug, Clone, Resource)]
pub(crate) struct MenuResources {
    color_normal: Color,
    color_selected: Color,
    font_title: TextFont,
    font_button: TextFont,
    node_root: Node,
    node_title: Node,
    node_buttons: Node,
    node_basic: Node,
}

impl MenuResources {
    fn title(&self, text: impl Into<String>) -> impl Bundle {
        (
            self.node_basic.clone(),
            Text::new(text),
            self.font_title.clone(),
            TextColor(self.color_normal),
        )
    }

    fn button(&self, marker: impl Component, text: impl Into<String>) -> impl Bundle {
        (
            marker,
            Button,
            self.node_basic.clone(),
            Text::new(text),
            self.font_button.clone(),
            TextColor(self.color_normal),
        )
    }
}

impl Default for MenuResources {
    fn default() -> Self {
        let color_normal = COLOR_NORMAL;
        let color_selected = COLOR_HOVERED;
        let font_title = TextFont::from_font_size(144.0); // .with_font_smoothing(FontSmoothing::None),
        let font_button = TextFont::from_font_size(50.0); // .with_font_smoothing(FontSmoothing::None),
        let node_root = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        let node_title = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(40.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        let node_buttons = Node {
            width: Val::Percent(50.0),
            height: Val::Percent(60.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        let node_basic = Node {
            width: Val::Auto,
            height: Val::Auto,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        };
        Self {
            color_normal,
            color_selected,
            font_title,
            font_button,
            node_root,
            node_title,
            node_buttons,
            node_basic,
        }
    }
}

fn setup(mut command: Commands, resources: Res<MenuResources>) {
    info!("enter GameState::Menu");
    let node_root = (Menu, resources.node_root.clone());
    let title = resources.title("Shape Shifter");
    let button_new_game = resources.button(NewGameButton, "New Game");
    let button_settings = resources.button(SettingsButton, "Settings");
    let button_exit = resources.button(ExitButton, "Exit");

    fn spawn_button<E: 'static + Event, B: Bundle, M>(
        parent: &mut RelatedSpawner<ChildOf>,
        button: impl Bundle,
        observer: impl IntoObserverSystem<E, B, M>,
    ) {
        parent
            .spawn(button)
            .observe(hover)
            .observe(unhover)
            .observe(selected)
            .observe(deselected)
            .observe(observer);
    }

    fn transition(
        state: GameState,
    ) -> impl Fn(Trigger<'_, Pointer<Click>>, ResMut<'_, NextState<GameState>>) {
        move |_: Trigger<Pointer<Click>>, mut next: ResMut<NextState<GameState>>| {
            next.set(state);
        }
    }

    // everything is children of node_root, so on
    command.spawn((
        node_root,
        children![
            (resources.node_title.clone(), children![title]),
            (
                resources.node_buttons.clone(),
                Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>| {
                    // these buttons should have up/down relations for gamepads to work
                    spawn_button(parent, button_new_game, transition(GameState::Ingame));
                    // sometimes here need a resume
                    spawn_button(parent, button_settings, transition(GameState::Settings));
                    spawn_button(parent, button_exit, transition(GameState::Exit));
                })),
            ),
        ],
    ));
}

fn cleanup(mut command: Commands, menu: Query<Entity, With<Menu>>) {
    info!("exit GameState::Menu");
    for entity in &menu {
        command.entity(entity).despawn();
    }
}

fn hover(trigger: Trigger<Pointer<Over>>, mut cmd: Commands) {
    cmd.entity(trigger.target()).insert(Selected);
}

fn unhover(trigger: Trigger<Pointer<Out>>, mut cmd: Commands) {
    cmd.entity(trigger.target()).remove::<Selected>();
}

fn selected(
    trigger: Trigger<OnAdd, Selected>,
    resources: Res<MenuResources>,
    mut query: Query<&mut TextColor>,
) {
    if let Ok(mut item) = query.get_mut(trigger.target()) {
        item.0 = resources.color_selected;
    }
}

fn deselected(
    trigger: Trigger<OnRemove, Selected>,
    resources: Res<MenuResources>,
    mut query: Query<&mut TextColor>,
) {
    if let Ok(mut item) = query.get_mut(trigger.target()) {
        item.0 = resources.color_normal;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuResources>()
            .add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnExit(GameState::Menu), cleanup);
    }
}
