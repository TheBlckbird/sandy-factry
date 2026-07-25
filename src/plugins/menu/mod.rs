use bevy::{prelude::*, text::FontSourceTemplate};

use crate::{
    game_save_types::LoadedGameSave,
    plugins::menu::{
        game_menus::GameMenusPlugin, main_menu::MainMenuPlugin, popup::PopupPlugin,
        splash_screen::SplashScreenPlugin,
    },
};

pub mod game_menus;
mod main_menu;
pub mod popup;
mod splash_screen;

// MARK: Constants

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

const MAIN_TEXT_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const MENU_BACKGROUND: Color = Color::hsl(15.0, 0.31, 0.5);

fn button(title: &'static str) -> impl Scene {
    bsn! {
        UiButton
        Node {
            width: px(400),
            height: px(65),
            margin: px(20),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
        }
        BackgroundColor(NORMAL_BUTTON)
        Children [
            Text::new(title)
            TextFont {
                font_size: px(33),
            }
            TextColor(TEXT_COLOR)
        ]
    }
}

fn get_continuous_text_font(font_size: Option<i32>) -> impl Scene {
    bsn! {
        TextFont {
            font: FontSourceTemplate::Handle("fonts/Rubik-Regular.ttf"),
            font_size: px(font_size.unwrap_or(25)),
        }
    }
}

// MARK: Plugin
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SplashScreenPlugin,
            MainMenuPlugin,
            GameMenusPlugin,
            PopupPlugin,
        ))
        .init_state::<GameState>()
        .init_resource::<LoadedGameSave>()
        .add_systems(Update, button_system);
    }
}

// MARK: State

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Game,
}

// MARK: Components

#[derive(Component)]
struct SelectedOption;

/// A button for the main UI.
///
/// It automatically gets the normal, hover and clicked colors assigned.
#[derive(Component, Clone, Copy, Default)]
#[require(Button)]
struct UiButton;

// MARK: Systems

/// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

/// System to update button hover states
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<UiButton>),
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
