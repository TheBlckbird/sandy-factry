use bevy::prelude::*;

use crate::plugins::menu::{despawn_screen, main_menu::MainMenuState};

use super::GameState;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Splash),
            (setup_splash_timer, splash_screen.spawn()),
        )
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashScreen>);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component, Clone, Copy, Default)]
struct SplashScreen;

fn setup_splash_timer(mut commands: Commands) {
    commands.insert_resource(SplashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
}

fn splash_screen() -> impl Scene {
    bsn! {
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: percent(100),
            height: percent(100),
        }
        SplashScreen
        Children [
            ImageNode {
                image: "app-icon.png"
            }
            Node {
                width: px(200),
            }
        ]
    }
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).is_finished() {
        game_state.set(GameState::MainMenu);
        main_menu_state.set(MainMenuState::Menu);
    }
}
