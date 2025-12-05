use bevy::prelude::*;

#[derive(Component)]
pub struct TurnIndicator;

pub fn spawn_turn_indicator(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "Player 1's Turn",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        TurnIndicator,
    ));
}

pub fn update_turn_indicator(_query: Query<&mut Text, With<TurnIndicator>>) {
    // This will be updated later with actual turn logic
    //if let Ok(mut text) = query.get_single_mut() {
    //    text.sections[0].value = "Player X's Turn".to_string(); // Placeholder
    //}
}
