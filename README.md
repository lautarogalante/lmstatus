# **lmstatus**
**lmstatus** is a simple status monitor written in Rust for **dwm window manager**.

#### Content

- [**lmstatus**](#lmstatus)
      - [Content](#content)
    - [Clone](#clone)
  - [Compilation](#compilation)
  - [Config File](#config-file)
    - [Example of config file:](#example-of-config-file)
  - [Move the binary](#move-the-binary)
    - [Example :](#example-)
  - [Autostart](#autostart)
    - [Example :](#example--1)

### Clone

Clone the project: 
```bash
git clone https://github.com/lautarogalante/lmstatus.git

cd lmstatus
```
## Compilation

To compile the project use `cargo` : 

```bash
cargo build --release
```

## Config File
**lmstatus** can be configured with a `Config.toml` configuration file, which can be saved at the path: `~/.config/lmstatus/Config.toml` .

### Example of config file: 

```toml
[format]
date_format = "%Y-%m-%d"
time_format = "%H:%M:%S"

[icons]
date = ""
time = ""

[icons.battery]
battery_full = " "
battery_middle = " "
battery_quarter = " "

[icons.volume]
volume_high = " "
volume_low = ""
volume_mute = "󰝟"
```
## Move the binary
To have the binary `lmstatus` execute at system startup, you need to move it to the path: `/usr/bin` or `/usr/local/bin` wherever you prefer.

### Example :
```bash
 mv /lmstatus/target/release/lmstatus /usr/local/bin
```

## Autostart
To autostart lmstatus add `lmstatus &` in your `.xprofile` or `.xinitrc`

### Example :
```bash
# .xprofile

lmstatus &


```
