use bevy::{color::palettes::tailwind, prelude::*};

use crate::GameState;

pub struct DayPlugin;

impl Plugin for DayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::DayTime), spawn_menus);
    }
}

fn spawn_menus(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut menu = commands.spawn((
        StateScoped(GameState::DayTime),
        Node {
            width: Val::Percent(75.0),
            height: Val::Percent(75.0),
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
                position_type: PositionType::Absolute,
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
                align_self: AlignSelf::End,
                margin: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            Text::new("Start level"),
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
