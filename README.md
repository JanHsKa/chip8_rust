# Chip 8 Emulator in Rust
This is a personal project to learn programming in Rust. Work in progress.



## Install

### Rust
To build this program you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

### SDL2
Additionally you need to install `SDL2`.

### Windows 
On Windows everything is installed with the `build.rs` script that is run automatically when the project is build.

### Linux
```  
    #Fedora  
    sudo dnf install SDL2 SDL2-devel 
    sudo dnf install SDL2_image SDL2_image-devel SDL2_ttf SDL2_ttf-devel
    
    #Ubuntu
    sudo apt-get install libsdl2-dev
    sudo apt-get install libsdl2-ttf-dev
    sudo apt-get install libsdl2-image-dev
    
```
### MacOs
```  
    sudo brew install sdl2
    sudo brew install sdl_ttf
    sudo brew install sdl2_image
```

## Build the project
When everything is installed, you can open a terminal in the project folder and type `cargo build` in the console and the project will be build. 

## Run the program
For the emulator to work you need to pass the path to a Chip 8 game. In the folder Games you can find a lot of them.   
To run use:  
```
cargo run Games/{game_file}
```
