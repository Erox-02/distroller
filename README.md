# Distroller

Display management cli for Hyprland, written in Rust. Distroller use hyprctl to manage your monitors directly from the terminal.

## Demo video 

<video src="./dis.mp4" controls="controls" style="max-width: 100%;"></video>

## Requirements

- Arch (btw) or another Linux system running Hyprland
- hyprland (0.55 or higher / at least lua support)

## Installation

```sh
wget https://github.com/Erox-02/distroller/releases/download/v0.1.0/PKGBUILD
makepkg -si
```
---

## Commands

### List 

```sh
distroller list
```

Shows monitors, their resolutions, and current refresh rates. Monitor numbers are used by the other commands.

### change scale

```sh
distroller scale 2 1.33
```

Sets the scale of monitor 2 to `1.33`.

### change refresh rate

```sh
distroller refresh 2 100
```

Sets monitor 2 to 100 Hz, if that refresh rate is supported at its current resolution.

### enable monitors

```sh
distroller enable 1,2
```

Enables monitors 1 and 2.

### disable a monitor

```sh
distroller disable 2
```

Disables monitor 2. Only one monitor can be disabled per command.

### arrange monitors

```sh
distroller use 2,1 --gap 10
```

Places monitor 2 on the left and monitor 1 on the right, with a 10-pixel gap.

You can choose a different gap:

```sh
distroller use 1,2 --gap 20
```

If `--gap' is ignored , the gap will use 10 pixel as default , the gap string is passed as the distance between two windows , it heavily depends on moniter resolution and 
scaling. 

## Build from source aka route for tinkerer's

```sh
git clone https://github.com/Erox-02/distroller
cd distroller
cargo build --release
```

The executable will be at "target/release/distroller".

## License
 
> GPL-3.0