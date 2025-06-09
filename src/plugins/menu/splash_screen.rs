use bevy::prelude::*;

use crate::plugins::menu::MainMenuState;

use super::{GameState, SplashScreen, SplashTimer};

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
