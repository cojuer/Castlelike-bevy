use crate::app_state::AppState;
use bevy::prelude::*;

#[derive(Component)]
pub enum MainMenuButton {
    Continue,
    Load,
    NewGame,
    Settings,
    Credits,
    Quit,
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let text_style = &mut text_query.get_mut(children[0]).unwrap().sections[0].style;
        match *interaction {
            Interaction::Clicked => {
                text_style.color = Color::rgb(0.8, 0.8, 0.8);
            }
            Interaction::Hovered => {
                text_style.color = Color::rgb(0.6, 0.6, 0.6);
            }
            Interaction::None => {
                text_style.color = Color::rgb(0.4, 0.4, 0.4);
            }
        }
    }
}

#[derive(Component)]
pub struct MainMenuCanvas;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(AppState::MainMenu)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(handle))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup))
            .add_system(button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/Lato-Regular.ttf"),
        font_size: 25.0,
        color: Color::BLACK,
    };
    let button_style = Style::default();
    let button_color = UiColor(Color::rgba(1., 1., 1., 0.));

    let bg_image = UiImage(asset_server.load("images/menu_texture.png"));

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            image: bg_image,
            ..Default::default()
        })
        .insert(MainMenuCanvas)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::Continue)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Continue",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::NewGame)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "New game",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::Continue)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Load game",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::Settings)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Settings",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::Credits)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Credits",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: button_color,
                    ..Default::default()
                })
                .insert(MainMenuButton::Quit)
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Quit",
                            button_text_style.clone(),
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });
        });
}

pub fn handle(
    interaction_query: Query<(&Interaction, &MainMenuButton)>,
    mut app_state: ResMut<State<AppState>>,
) {
    for (i, b) in interaction_query.iter() {
        match (i, b) {
            (
                Interaction::Clicked,
                MainMenuButton::Continue | MainMenuButton::Load | MainMenuButton::NewGame,
            ) => {
                app_state.set(AppState::Game).unwrap();
            }
            _ => {}
        }
    }
}

pub fn cleanup(mut commands: Commands, q: Query<Entity, With<MainMenuCanvas>>) {
    commands.entity(q.single()).despawn_recursive();
}
