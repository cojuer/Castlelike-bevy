use crate::app_state::AppState;
use bevy::prelude::*;

pub enum MainMenuButton {
    Continue,
    Load,
    Options,
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

pub struct MainMenuCanvas;

pub fn register_main_menu(app: &mut AppBuilder) {
    app.add_state(AppState::MainMenu)
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(main_menu_create.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::MainMenu).with_system(main_menu_handle.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(main_menu_cleanup.system()),
        )
        .add_system(button_system.system());
}

fn main_menu_create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/Lato-Regular.ttf"),
        font_size: 25.0,
        color: Color::BLACK,
    };
    let button_style = Style::default();
    let button_material = materials.add(Color::rgba(1., 1., 1., 0.).into());

    let texture_handle = asset_server.load("images/menu_texture.png");
    let material_handle = materials.add(ColorMaterial {
        texture: Some(texture_handle.clone()),
        ..Default::default()
    });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: material_handle,
            ..Default::default()
        })
        .insert(MainMenuCanvas)
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: button_style.clone(),
                    material: button_material.clone(),
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
                    material: button_material.clone(),
                    ..Default::default()
                })
                .insert(MainMenuButton::Continue)
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
                    material: button_material.clone(),
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
                    material: button_material.clone(),
                    ..Default::default()
                })
                .insert(MainMenuButton::Continue)
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
                    material: button_material.clone(),
                    ..Default::default()
                })
                .insert(MainMenuButton::Continue)
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
                    material: button_material.clone(),
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

pub fn main_menu_handle() {}

pub fn main_menu_cleanup() {}
