use bevy::{color::palettes::tailwind, prelude::*};

use crate::{player::PlayerStats, GameState};

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DayTime), (new_day, spawn_menus).chain());
        app.add_systems(OnEnter(GameState::GameOver), spawn_over);
        app.add_systems(OnEnter(GameState::GameWon), spawn_won);
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
        name: "Melatonin",
        description: &["+5 sleep duration"],
        cost: 50,
        effect: |player_stats| {
            player_stats.sleep_duration += 5.0;
        },
    },
    Upgrade {
        name: "Extra blanket",
        description: &["+1 warmth"],
        cost: 150,
        effect: |player_stats| {
            player_stats.warmth += 1.0;
        },
    },
    Upgrade {
        name: "Milk and cookies",
        description: &["+2 hydration", "+3 comfort"],
        cost: 40,
        effect: |player_stats| {
            player_stats.hydration += 5.0;
            player_stats.comfort += 5.0;
        },
    },
    Upgrade {
        name: "Booze",
        description: &["-5 hydration", "-2 comfort", "+10 sleep duration"],
        cost: 40,
        effect: |player_stats| {
            player_stats.hydration -= 5.0;
            player_stats.comfort -= 2.0;
            player_stats.sleep_duration += 10.0;
        },
    },
];

#[derive(Component)]
pub struct StatsField;

fn spawn_menus(mut commands: Commands, player_stats: Res<PlayerStats>) {
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
        let mut day = menu.spawn((
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Text::new(format!(
                "{} (day {})",
                day_string(&player_stats),
                player_stats.day
            )),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ));

        if player_stats.day == 6 {
            day.insert(TextColor(tailwind::RED_500.into()));
        }

        menu.spawn((
            Node {
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Text::new("Sleep 60 seconds before day 7 to win"),
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
            for name in [
                "Comfort",
                "Warmth",
                "Hydration",
                "Sleep duration",
                "Rest gained",
                "Rest",
            ] {
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
                        Text::new(format!("{name} ({cost} rest)")),
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

    for (mut text, _node) in query.iter_mut() {
        let name = text.0.split("    ").next().unwrap_or("");
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
        "Sleep duration" => player_stats.sleep_duration,
        "Rest gained" => player_stats.unsafe_rest as f32,
        _ => panic!("Unknown stat name: {name}"),
    };

    format!("{name:25} {stat}")
}

fn day_string(player_stats: &PlayerStats) -> &'static str {
    match player_stats.day {
        1 => "It's the first day, better get to sleep",
        2 => "You slept a while, but not enough. Guess you should try again",
        3 => "That was refreshing, but you still feel tired",
        4 => "The bed fills you with dread, are you prepared for the night?",
        5 => "Surely you can sleep a full night right?",
        6 => "You can't really keep going, this is your final chance",
        _ => "Day ???",
    }
}

fn start_level(trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>) {
    if trigger.button == PointerButton::Primary {
        next_state.set(GameState::NightTime);
    }
}

fn quit(trigger: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
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
    let Ok((_node, mut bg_color)) = node_query.get_mut(trigger.entity()) else {
        return;
    };

    bg_color.0 = tailwind::BLUE_500.into();
}

fn button_hover_effect_out(
    trigger: Trigger<Pointer<Out>>,
    mut node_query: Query<(&mut Node, &mut BackgroundColor)>,
) {
    let Ok((_node, mut bg_color)) = node_query.get_mut(trigger.entity()) else {
        return;
    };

    bg_color.0 = Color::NONE;
}

fn new_day(mut player_stats: ResMut<PlayerStats>, mut next_state: ResMut<NextState<GameState>>) {
    player_stats.rest += player_stats.unsafe_rest;
    player_stats.day += 1;

    if player_stats.sleep_duration >= 59.0 && !player_stats.died {
        next_state.set(GameState::GameWon);
    } else if player_stats.day > 6 {
        next_state.set(GameState::GameOver);
    }
}

fn spawn_over(mut commands: Commands) {
    let mut menu = commands.spawn((
        StateScoped(GameState::GameOver),
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundColor(tailwind::RED_300.into()),
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
            Text::new("You failed to sleep through the night"),
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
            Text::new("Try again?"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ))
        .observe(new_game);
    });
}

fn spawn_won(mut commands: Commands) {
    let mut menu = commands.spawn((
        StateScoped(GameState::GameWon),
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        BackgroundColor(tailwind::GREEN_300.into()),
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
            Text::new("You survived the night"),
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
            Text::new("Try again?"),
            TextFont {
                font_size: 12.0,
                ..default()
            },
        ))
        .observe(new_game);
    });
}

fn new_game(
    trigger: Trigger<Pointer<Click>>,
    mut player_stats: ResMut<PlayerStats>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if trigger.button == PointerButton::Primary {
        *player_stats = PlayerStats::default();
        next_state.set(GameState::DayTime);
    }
}
