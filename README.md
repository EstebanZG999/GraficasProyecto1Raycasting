# Cuphead Maze Game

Welcome to the **Cuphead Maze Game**! This is a 3D maze game with a Cuphead theme, where you can navigate through a maze, avoid enemies, and enjoy the unique Cuphead-style graphics.


## Video Demo

[Check out a demo of the game on YouTube:](https://youtu.be/wFG6eMvoMhU)


## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Controls](#controls)
- [Project Structure](#project-structure)
- [Credits](#credits)

## Features

- **3D Maze Rendering**: Explore a 3D maze with textured walls, floors, and sky.
- **Animated Enemies**: Avoid animated enemies with different textures.
- **Minimap**: Navigate using a minimap that shows your position and the layout of the maze.
- **FPS Display**: Real-time FPS counter displayed on the screen.
- **Welcome Screen**: An introductory screen with animated frames before the game starts.
- **Background Music and Sound Effects**: Enjoy background music and sound effects for a more immersive experience.

## Installation

To run this project locally, follow these steps:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/yourusername/CupheadMazeGame.git
   cd CupheadMazeGame

2. Ensure you have Rust installed on your machine. If not, you can install it from rust-lang.org.

Then, install the required dependencies by running:
cargo build

3. **Run the Game:**
To run the game, use the following command:
**cargo run --release**


## Controls
WASD: Move the player around the maze.
M: Toggle between 2D and 3D views.
ESC: Exit the game.

CupheadMazeGame/
├── assets/
│   ├── FloralFury.mp3       # Background music
│   ├── footsteps.mp3        # Footsteps sound effect
│   ├── introframe1.jpeg     # Intro animation frame 1
│   ├── introframe2.jpeg     # Intro animation frame 2
│   ├── introframe3.jpeg     # Intro animation frame 3
│   ├── font.ttf             # Font for rendering text
│   └── sprites/             # Directory for texture files
│       ├── wall4.webp
│       ├── floor7.webp
│       ├── sky3.jpeg
│       ├── cagney.png
│       └── cagney2.png
├── src/
│   ├── main.rs              # Main game logic
│   ├── maze.rs              # Maze generation and loading
│   ├── player.rs            # Player movement and controls
│   ├── raycasting.rs        # Ray casting logic for 3D rendering
│   ├── controls.rs          # Input processing
│   ├── textures.rs          # Texture loading and management
│   ├── audio.rs             # Audio management for music and sound effects
└── Cargo.toml               # Rust project configuration


## Credits
Cuphead: All character designs and themes are inspired by the game Cuphead by Studio MDHR.
Rust Libraries: This project utilizes several Rust libraries including minifb, nalgebra, once_cell, rusttype, and image.
Music and Sound Effects: Background music and sound effects are used under fair use.

