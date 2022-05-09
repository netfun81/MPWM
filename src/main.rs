#[macro_use]
extern crate penrose;

use penrose::{
    contrib::{
        extensions::Scratchpad,
        hooks::{
            LayoutSymbolAsRootName,
        },
        layouts::paper,
    },
    core::{
        config::Config,
        helpers::index_selectors,
        hooks::Hook,
        layout::{
            bottom_stack,
            side_stack,
            Layout,
            LayoutConf,
        },
        manager::WindowManager,
        ring::Selector,
        xconnection::{
            XConn,
        },
    },
    xcb::{
        XcbConnection,
        XcbHooks,
    },
    logging_error_handler,
    Backward,
    Forward,
    Less,
    More,
    Result,
};

use simplelog::{LevelFilter, SimpleLogger};
use std::collections::HashMap;

struct StartupScript {
    path: String,
}

impl<X: XConn> Hook<X> for StartupScript {
    fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
        spawn!(&self.path)
    }
}

fn main() -> Result<()> {
    SimpleLogger::init(LevelFilter::Debug, simplelog::Config::default())
        .expect("failed to init logging");

    let mut config_builder = Config::default().builder();
    config_builder
        .floating_classes(vec!["dmenu", "dunst", "polybar", "rofi"])
        .focused_border("#0000ff")?
        .unfocused_border("#0c1014")?
        .border_px(2)
        .gap_px(2)
        .show_bar(false)
        .top_bar(false)
        .bar_height(0);

    let follow_focus_conf = LayoutConf {
        floating: false,
        gapless: true,
        follow_focus: true,
        allow_wrapping: false,
    };

    let n_main = 1;
    let ratio = 0.5;

    config_builder.layouts(vec![
        Layout::new("[side]", LayoutConf::default(), side_stack, n_main, ratio),
        Layout::new("[botm]", LayoutConf::default(), bottom_stack, n_main, ratio),
        Layout::new("[papr]", follow_focus_conf, paper, n_main, ratio),
        Layout::floating("[----]"),
    ]);

    let config = config_builder.build().unwrap();
    
    let program_launcher = "rofi -show run";
    let file_manager     = "thunar";
    let terminal         = "alacritty";
    let browser          = "chromium --incognito";
    let mixer            = "pavucontrol";
    let locker           = "slock";

    let sp = Scratchpad::new(terminal, 0.8, 0.8);
    let hooks: XcbHooks = vec![
       LayoutSymbolAsRootName::new(),
        sp.get_hook(),
    ];

    let key_bindings = gen_keybindings! {
        "M-r"   => run_external!(program_launcher);
        "M-t"   => run_external!(terminal);
        "M-e"   => run_external!(file_manager);
        "M-w"   => run_external!(browser);
        "M-a"   => run_external!(mixer);
        "M-s"   => run_external!(locker);

        "M-Tab"    => run_internal!(cycle_client, Forward);
        "M-S-Tab"  => run_internal!(cycle_client, Backward);
        "M-j"      => run_internal!(drag_client, Forward);
        "M-k"      => run_internal!(drag_client, Backward);
        "M-d"      => run_internal!(kill_client);
        "M-f"      => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-Return" => sp.toggle(); // popup terminal
        "M-n"      => run_internal!(toggle_workspace);
        
        "M-grave"    => run_internal!(cycle_layout, Forward);
        "M-S-grave"  => run_internal!(cycle_layout, Backward);
        "M-Up"       => run_internal!(update_max_main, More);
        "M-Down"     => run_internal!(update_max_main, Less);
        "M-Right"    => run_internal!(update_main_ratio, More);
        "M-Left"     => run_internal!(update_main_ratio, Less);
        "M-S-s"      => run_internal!(detect_screens);
        "M-S-Escape" => run_internal!(exit);

        map: { "z", "x", "c", "v", "b" } to index_selectors(5) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let conn = XcbConnection::new()?;

    let mut wm = WindowManager::new(config, conn, hooks, logging_error_handler());
    wm.init()?;
    
    wm.grab_keys_and_run(key_bindings, HashMap::new())?;

    Ok(())
}
