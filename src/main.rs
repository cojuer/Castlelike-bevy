mod app_state;
mod game;
mod menu;

use bevy::prelude::*;
use bevy::window::WindowMode;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct RenderCfg {
    width: u16,
    height: u16,
    fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cfg {
    render: RenderCfg,
}

fn main() {
    let cfg_path = env::args()
        .skip(1)
        .next()
        .expect("configuration path not provided");
    let cfg_path = Path::new(&cfg_path);

    let cfg_reader = File::open(cfg_path).expect("failed to open configuration file");
    let mut cfg_reader = BufReader::new(cfg_reader);

    let mut cfg_contents = String::new();
    cfg_reader
        .read_to_string(&mut cfg_contents)
        .expect("failed to read configuration");

    let cfg: Cfg = serde_yaml::from_str(&cfg_contents).expect("failed to parse configuration");

    println!(
        "{}x{} fscreen {}",
        cfg.render.width, cfg.render.height, cfg.render.fullscreen
    );

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: cfg.render.width.into(),
        height: cfg.render.height.into(),
        vsync: true,
        mode: if cfg.render.fullscreen {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        },
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);

    app.add_startup_system(|mut commands: Commands| {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
        commands.spawn_bundle(UiCameraBundle::default());
    });

    app.add_system(bevy::input::system::exit_on_esc_system);

    use menu::MainMenuPlugin;
    app.add_plugin(MainMenuPlugin);

    use game::plugin::GamePlugin;
    app.add_plugin(GamePlugin);

    app.run();
}
