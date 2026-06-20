# NumpadDriver

A Linux driver for Asus laptop touchpads with an integrated numpad.

This project started as an attempt to make a touchpad numpad work outside Windows. The device exposed only normal touchpad functionality under Linux, so the communication protocol had to be reverse engineered and reimplemented.

## Features

* Enables the integrated numpad on supported devices
* Reads numpad key presses directly from the hardware
* Emits standard Linux input events
* Lightweight and written in Rust

## Motivation

Asus laptops ship with touchpads that include a numpad feature. In some cases, the numpad only works through vendor software on Windows and has no official Linux support.

This project aims to change that by providing an open-source implementation based on reverse engineering and experimentation.

## Running

```bash
make install
sudo systemctl start numpaddriver.service
sudo systemctl enable numpaddriver.service
```

## License

MIT
