use bevy::prelude::*;

use crate::plugins::menu::{
    MAIN_TEXT_COLOR, MENU_BACKGROUND, button, get_continuous_text_font,
    main_menu::{HowToPlayMenu, MainMenuState},
};

pub fn how_to_play_menu() -> impl Scene {
    bsn! {
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        HowToPlayMenu
        Children [
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            }
            BackgroundColor(MENU_BACKGROUND)

            Children [
                Text::new("Sandy Fact'ry")
                TextFont {
                    font_size: px(67),
                }
                TextColor(MAIN_TEXT_COLOR)
                Node {
                    margin: px(50),
                },

                Text::new(include_str!("./how-to-play.txt"))
                get_continuous_text_font(Some(20))
                TextColor(MAIN_TEXT_COLOR)
                Node {
                    max_width: px(600),
                },

                button("Back")
                on(|_event: On<Pointer<Press>>, mut main_menu_state: ResMut<NextState<MainMenuState>>| {
                    main_menu_state.set(MainMenuState::Menu);
                })
            ]
        ]
    }
}
