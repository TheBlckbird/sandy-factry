use bevy::prelude::*;

use crate::game_save_types::LoadedGameSave;

mod main_menu;
mod pause_menu;
mod splash_screen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

const MAIN_TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const MENU_BACKGROUND: Color = Color::hsl(15.0, 0.31, 0.5);

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Game,
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PauseMenuState {
    #[default]
    Hidden,
    Shown,
}

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component)]
enum MainMenuButtonAction {
    Play,
    Quit,
}

#[derive(Component)]
enum GameMenuButtonAction {
    BackToGame,
    BackToMainMenu,
    Quit,
}

#[derive(Component)]
struct MainMenuScreen;

#[derive(Component)]
struct GameMenuScreen;

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
struct SplashScreen;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .init_state::<PauseMenuState>()
            .init_resource::<LoadedGameSave>()
            // Splash Screen
            .add_systems(
                OnEnter(GameState::Splash),
                splash_screen::setup_splash_screen,
            )
            .add_systems(
                Update,
                splash_screen::countdown.run_if(in_state(GameState::Splash)),
            )
            .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashScreen>)
            // Main Menu
            .add_systems(OnEnter(GameState::MainMenu), main_menu::setup_menu)
            .add_systems(
                Update,
                (main_menu::update_menu, button_system).run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                despawn_screen::<MainMenuScreen>,
            )
            // Game Menu
            .add_systems(OnEnter(PauseMenuState::Shown), pause_menu::setup_menu)
            .add_systems(
                Update,
                (pause_menu::update_menu, button_system).run_if(in_state(PauseMenuState::Shown)),
            )
            .add_systems(
                OnExit(PauseMenuState::Shown),
                despawn_screen::<GameMenuScreen>,
            )
            .add_systems(
                Update,
                pause_menu::show_game_menu.run_if(in_state(GameState::Game)),
            );
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
