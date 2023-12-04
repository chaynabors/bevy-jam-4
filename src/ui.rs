use bevy::prelude::*;

use crate::{
    enemy::SpawnGeneration,
    player::{NetPlayer, Player},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, update);
    }
}

#[derive(Component)]
struct UiText;

fn startup(mut commands: Commands, server: Res<AssetServer>) {
    let font = server.load("fonts/Roboto-Regular.ttf");

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Wave: ",
                TextStyle {
                    font_size: 50.0,
                    font: font.clone(),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                font: font.clone(),
                ..default()
            }),
            TextSection::new(
                "\nHealth: ",
                TextStyle {
                    font_size: 50.0,
                    font: font.clone(),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 50.0,
                font: font.clone(),
                ..default()
            }),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        UiText,
    ));
}

fn update(
    mut ui_text: Query<&mut Text, With<UiText>>,
    spawn_generation: Res<SpawnGeneration>,
    player: Query<&Player, Without<NetPlayer>>,
) {
    let player = player.single();
    for mut text in ui_text.iter_mut() {
        text.sections[1].value = spawn_generation.0.to_string();
        text.sections[3].value = format!("{:.0}", player.health);
    }
}
