#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::MouseEvent, config::Config, helpers::index_selectors, manager::WindowManager,
    },
    logging_error_handler,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Result, Selector
};

fn main() -> Result<()> {
    let hooks = vec![];
    
    let mut config_builder = Config::default().builder();
    config_builder
		.workspaces(vec!["1", "2", "3", "4", "5"])
        .floating_classes(vec!["dmenu", "dunst", "polybar", "rofi"])
        .focused_border("#0000ff")?
        .unfocused_border("#0c1014")?
        .border_px(2)
        .gap_px(2)
        .show_bar(false)
        .top_bar(false)
        .bar_height(0);
    let config = config_builder.build().unwrap();

    let key_bindings = gen_keybindings! {
        "M-Tab" => run_internal!(cycle_client, Forward);
        "M-S-Tab" => run_internal!(cycle_client, Backward);
        "M-j" => run_internal!(drag_client, Forward);
        "M-k" => run_internal!(drag_client, Backward);
        "M-d" => run_internal!(kill_client);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-n" => run_internal!(toggle_workspace);
        "M-x" => run_internal!(cycle_workspace, Forward);
        "M-z" => run_internal!(cycle_workspace, Backward);
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-Up" => run_internal!(update_max_main, More);
        "M-Down" => run_internal!(update_max_main, Less);
        "M-Right" => run_internal!(update_main_ratio, More);
        "M-Left" => run_internal!(update_main_ratio, Less);
        "M-S-Escape" => run_internal!(exit);
        "M-w" => run_external!("chromium --incognito");
        "M-e" => run_external!("thunar");
        "M-r" => run_external!("rofi -show run");
        "M-t" => run_external!("alacritty");
        "M-a" => run_external!("pavucontrol");
        "M-s" => run_external!("slock");

        map: { "1", "2", "3", "4", "5" } to index_selectors(5) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mouse_bindings = gen_mousebindings! {
        Press Right + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Forward),
        Press Left + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Backward)
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, mouse_bindings)?;

    Ok(())
}
