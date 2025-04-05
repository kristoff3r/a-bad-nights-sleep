use std::borrow::Cow;

use bevy::{color::palettes::tailwind, prelude::*};

use crate::{player::PlayerStats, GameState};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DayTime), spawn_menus);
        app.add_systems(Update, update_stats);
    }
}

#[derive(Component, Clone)]
pub struct Upgrade {
    pub name: &'static str,
    pub description: &'static [&'static str],
    pub cost: u32,
    pub effect: fn(&mut PlayerStats),
}

pub const UPGRADES: &[Upgrade] = &[
    Upgrade {
        name: "melatonin",
        description: &["+5 sleep intensity", "+5 sleep duration"],
        cost: 50,
        effect: |player_stats| {
            player_stats.sleep_intensity += 5.0;
            player_stats.sleep_duration += 5.0;
        },
    },
    Upgrade {
        name: "extra blanket",
        description: &["+10 comfy"],
        cost: 70,
        effect: |player_stats| {
            player_stats.comfort += 10.0;
        },
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
    Upgrade {
        name: "placeholder",
        description: &["no effect"],
        cost: 1,
        effect: |player_stats| {},
    },
];

#[derive(Component)]
pub struct StatsField;

fn spawn_menus(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    player_stats: Res<PlayerStats>,
) {
    let mut menu = commands.spawn((
        StateScoped(GameState::DayTime),
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundColor(tailwind::BLUE_300.into()),
        PickingBehavior::IGNORE,
    ));

    menu.observe(|mut trigger: Trigger<Pointer<Click>>| {
        trigger.propagate(false);
    });

    menu.with_children(|menu| {
        menu.spawn((
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Text::new("A bad night's sleep"),
            TextFont {
                font_size: 32.0,
                ..default()
            },
        ));
        menu.spawn((
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Text::new(day_string(&player_stats)),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ));

        menu.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(tailwind::GREEN_500.into()),
        ))
        .with_children(|stats| {
            stats.spawn((
                Node {
                    margin: UiRect::axes(Val::Px(10.0), Val::Px(10.0)),
                    ..default()
                },
                Text::new("Sleep prep"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ));
            for name in ["Comfort", "Snug", "Warmth", "Hydration", "Rest"] {
                let text = create_stat_string(&player_stats, name);
                stats.spawn((
                    Node {
                        margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                        ..default()
                    },
                    Text::new(text),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                    StatsField,
                ));
            }
        });

        menu.spawn((
            Node {
                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::px(200.0),
                    GridTrack::px(200.0),
                    GridTrack::px(200.0),
                    GridTrack::px(200.0),
                ],
                grid_template_rows: vec![
                    GridTrack::px(80.0),
                    GridTrack::px(80.0),
                    GridTrack::px(80.0),
                    GridTrack::px(80.0),
                ],
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(tailwind::PURPLE_500.into()),
        ))
        .with_children(|upgrades| {
            for upgrade @ Upgrade {
                name,
                description,
                cost,
                ..
            } in UPGRADES.iter()
            {
                let mut upgrade = upgrades.spawn((
                    Node {
                        margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(tailwind::PINK_500.into()),
                    upgrade.clone(),
                ));

                upgrade.with_children(|upgrade| {
                    upgrade.spawn((
                        Node {
                            margin: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                            ..default()
                        },
                        Text::new(format!("{name} ({cost})")),
                        TextFont {
                            font_size: 12.0,
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ));
                    for line in *description {
                        upgrade.spawn((
                            Node {
                                margin: UiRect::axes(Val::Px(10.0), Val::Px(0.0)),
                                ..default()
                            },
                            Text::new(format!("{line}")),
                            TextFont {
                                font_size: 8.0,
                                ..default()
                            },
                            BackgroundColor(Color::NONE),
                        ));
                    }
                });

                upgrade
                    // .observe(button_hover_effect_over)
                    // .observe(button_hover_effect_out)
                    .observe(buy_upgrade);
            }
        });

        menu.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            },
            Text::new("Try to sleep"),
            BackgroundColor(Color::NONE),
        ))
        .observe(button_hover_effect_over)
        .observe(button_hover_effect_out)
        .observe(start_level);

        menu.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                bottom: Val::Px(10.0),
                ..default()
            },
            Text::new("Quit"),
            BackgroundColor(Color::NONE),
        ))
        .observe(button_hover_effect_over)
        .observe(button_hover_effect_out)
        .observe(quit);
    });
}

fn update_stats(
    mut query: Query<(&mut Text, &Node), With<StatsField>>,
    player_stats: Res<PlayerStats>,
    mut last_player_stats: Local<PlayerStats>,
) {
    if *last_player_stats != *player_stats {
        *last_player_stats = player_stats.clone();
    } else {
        return;
    }

    for (mut text, node) in query.iter_mut() {
        let name = text.0.split(' ').next().unwrap_or("");
        text.0 = create_stat_string(&player_stats, name);
    }
}

fn create_stat_string(player_stats: &PlayerStats, name: &str) -> String {
    let stat = match name {
        "Comfort" => player_stats.comfort,
        "Snug" => player_stats.snug,
        "Warmth" => player_stats.warmth,
        "Hydration" => player_stats.hydration,
        "Rest" => player_stats.rest as f32,
        _ => panic!("Unknown stat name: {name}"),
    };

    format!("{name:12} {stat}")
}

fn day_string(player_stats: &PlayerStats) -> &'static str {
    match player_stats.day {
        0 => "It's the first day, better get to sleep",
        1 => "Day 2",
        2 => "Day 3",
        _ => "Day ???",
    }
}

fn start_level(mut trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    if trigger.button == PointerButton::Primary {
        next_state.set(GameState::NightTime);
    }
}

fn quit(mut trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    if trigger.button == PointerButton::Primary {
        app_exit.send(AppExit::Success);
    }
}

fn buy_upgrade(
    trigger: Trigger<Pointer<Click>>,
    mut player_stats: ResMut<PlayerStats>,
    mut upgrades: Query<(&Upgrade, &mut BackgroundColor)>,
) {
    if trigger.button == PointerButton::Primary {
        if let Ok((upgrade, mut bg_color)) = upgrades.get_mut(trigger.entity()) {
            if player_stats.rest >= upgrade.cost {
                (upgrade.effect)(&mut player_stats);
                player_stats.rest -= upgrade.cost;
            } else {
                bg_color.0 = tailwind::RED_500.into();
            }
        }
    }
}

fn button_hover_effect_over(
    trigger: Trigger<Pointer<Over>>,
    mut node_query: Query<(&mut Node, &mut BackgroundColor)>,
) {
    let Ok((node, mut bg_color)) = node_query.get_mut(trigger.entity()) else {
        return;
    };

    bg_color.0 = tailwind::BLUE_500.into();
}

fn button_hover_effect_out(
    trigger: Trigger<Pointer<Out>>,
    mut node_query: Query<(&mut Node, &mut BackgroundColor)>,
) {
    let Ok((node, mut bg_color)) = node_query.get_mut(trigger.entity()) else {
        return;
    };

    bg_color.0 = Color::NONE;
}
