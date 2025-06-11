use bevy::prelude::*;

use crate::plugins::menu::{despawn_screen, main_menu::MainMenuState};

use super::GameState;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), setup_splash_screen)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<SplashScreen>);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(Timer);

#[derive(Component)]
struct SplashScreen;

pub fn setup_splash_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("app-icon.png");

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        SplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                width: Val::Px(200.0),
                ..default()
            }
        )],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(0.5, TimerMode::Once)));
}

pub fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
        main_menu_state.set(MainMenuState::Menu);
    }
}
