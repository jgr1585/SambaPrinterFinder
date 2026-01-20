# Samba Printer Finder
A simple GTK4 application written in Rust that discovers and lists Samba (SMB) printers available on the local network.

## Features
- Discovers printers shared via Samba on the local network.
- Displays printer names and their network addresses in a user-friendly GTK4 interface.
- Allows the user to install selected printers.

## Installing
TODO: Provide pre-built binaries or package instructions. Currently, you need to build from source.


## Build requirements
You need to have the following dependencies installed on your system:
- Rust (latest stable version)
- GTK4 development libraries (https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html)
- CUPS development libraries
- Samba development libraries

## Building on Ubuntu/Debian
```bash
sudo apt install libgtk-4-dev libcups2-dev libsamba-dev build-essential
```

### Building on Fedora
```bash
sudo dnf install gtk4-devel cups-devel samba-devel gcc
```

### Arch Linux
```bash
sudo pacman -S gtk4 cups samba base-devel
```

### MacOS (with Homebrew)
```bash
brew install gtk4 cups samba
```

#### Windows
Building under Windows untested, undocumented and not officially supported.
You can try it, but you will need to figure out how to install the required dependencies yourself.
It is recommended to use a native Linux environment for best results.