
#[derive(Serialize, Deserialize, Debug)]
struct RenderCfg {
    width: u16,
    height: u16,
    fullscreen: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Cfg {
    render: RenderCfg
}

// TODO: parse config or return errors
// TODO: write config or return errors
// TODO: apply config (probably not here)