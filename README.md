# Chip 8 Emulator in Rust
This is a personal project to learn programming in Rust. It is a multithreaded Chip 8 emulator that can run the standard Chip 8 programs and programs that use the 10 Super Chip 8 codes. The emulator has a built in debug mode. 
  
Work in progress.

----

## Install

### Rust
To build this program you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

### SDL2
Additionally you need to install `SDL2`.

### Windows 
On Windows `SDL2` is installed with the `build.rs` script that is run automatically when the project is build.

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
-----

## Build the project
When everything is installed, you can open a terminal in the project folder and type `cargo build` in the console and the project will be build and is ready to be run. 


----
## Run the program
For the emulator to work you need to pass the path to a Chip 8 game. In the folder Games you can find a lot of them. The Super Chip 8 games are in the folder Games/Super.   
To run a game use:  
```
cargo run Games/{game_file}
```

-----

## Controls

Most controls are shown in the panel on the left below the game display. There are some additional keybinds that are not listed:

| Key or action | Function |
| ----------- | ----------- |
| `K`      | Changes how the program handles key inputs. Either the keys are reset after they are read once, or they reset after the key goes back up. Some programs don't work properly with the second option, so you can switch it as you like     |
| `Drag and Drop`   | You can drag and drop a Chip 8 file into the program and it will load the new game        |

   

There are also some commands that are not implemented or do not work as intended right now.