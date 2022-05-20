# MPWM
A tiling Window Manager for X11, using the Penrose library written in Rust language.

TO USE:
Install cargo and rust libraries from your package manager if needed.

Make any desired changes to keybinds in /src/main.rs

In a terminal type:
cargo build --release

Edit .xinitrc and add path to executable:
--
geany ~/.xinitrc
--
~/MPWM-main/target/release/mpwm
