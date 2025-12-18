use bevy::prelude::*;
use crate::{
    Score,
    nanonaut::{Hp, MAX_HP, Nanonaut}
};


pub fn hud_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui)
        .add_systems(Update, update_hp_bar)
        .add_systems(Update, update_score);
}

fn spawn_ui(mut commands: Commands, init_score: Res<Score>) {
    let container = Node {
        display: Display::Flex,
        width: Val::Percent(100.0),
        height: Val::Percent(12.5),
        justify_content: JustifyContent::Center,
        ..default()
    };

    commands.spawn((
        container, children![
            hp_bar(),
            score_text(&init_score)
        ]
    ));
}

#[derive(Component)]
struct HpBar;
fn hp_bar() -> impl Bundle {
    let full_bar = (
        // #201537 (32, 21, 55)
        BackgroundColor(Color::srgb_u8(32, 21, 55)),
        BorderColor::all(Color::srgb_u8(253, 40, 40)),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(70.0),
            margin: UiRect::vertical(Val::Auto).with_left(Val::Percent(2.0)),
            border: UiRect::all(Val::Px(4.0)),
            border_radius: BorderRadius::all(Val::Percent(30.0)),
            ..default()
        }
    );
    let current_hp_bar = (
        HpBar,
        // #FD2828 (253, 40, 40)
        BackgroundColor(Color::srgb_u8(253, 40, 40)),
        Node::DEFAULT,
    );

    (full_bar, children![current_hp_bar])
}

fn score_text(init_score: &Score) -> impl Bundle {
    let right_margin = 6.0;
    let total_width = 100.0 - right_margin;
    let score_node = Node {
        width: Val::Percent(total_width),
        height: Val::Percent(80.0),
        margin: UiRect::vertical(Val::Auto).with_right(Val::Percent(right_margin)),
        ..default()
    };

    (
        Text::new(format!("{}", init_score.0)),
        TextColor::BLACK,
        TextFont::from_font_size(52.0),
        TextLayout::new_with_justify(Justify::Right),
        score_node,
    )
}
fn update_score(
    mut score_text: Single<&mut Text>,
    score: Res<Score>
) {
    // Single -> Mut Text -> Text -> String
    ***score_text = format!("{}", score.0);
}

fn update_hp_bar(
    mut current_hp_bar: Single<&mut Node, With<HpBar>>,
    nanonaut_hp: Single<&Hp, With<Nanonaut>>
) {
    let hp_percentage = nanonaut_hp.value() / MAX_HP * 100.0;
    current_hp_bar.width = Val::Percent(hp_percentage);
}