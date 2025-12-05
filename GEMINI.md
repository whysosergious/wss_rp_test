# Gemini Project: wss_rp_test

## Project Overview

This is a hex-based war game built using the Bevy game engine in Rust. The application features a setup screen (`SetupState`) with UI elements to configure game parameters such as board size (5x5 to 20x20), number of players (2-6), initial armies per starting tile (1-5), and reinforcement rate (1-10). Upon generating the board, the game transitions to `GameState`, where a correctly generated hexagonal grid (horizontal and with outlines) is rendered. Initial territories are visibly assigned to factions with distinct colors, and armies are placed. Each tile also displays its army count using a 2D UI overlay. An on-screen UI indicator shows whose turn it is. The camera provides an overview of the map with intuitive controls. All previous panics and issues have been resolved, ensuring smooth functionality of the camera, UI, state transitions, and map generation. Pressing the `Escape` key returns to the `SetupState`, allowing the user to regenerate the board with new parameters.

### Font Requirement
For the army count display on tiles to work, a font file named `FiraSans-Bold.ttf` must be present in the `assets/fonts/` directory relative to the project root.

## Controls

*   **W, A, S, D:** Move the camera's focus point across the map.
*   **Right Mouse Button + Mouse Movement:** Rotate and tilt the camera around its focus point.
*   **Mouse Wheel:** Zoom the camera in and out.
*   **Free Cursor:** The cursor is free by default, allowing interaction with UI elements.
*   **Escape:** Return to the Setup State to regenerate the board.

## Building and Running

To build and run the project, use the following command:

```bash
cargo run
```

This will compile the necessary dependencies and launch the application window.

## Development Conventions

The project follows a modular structure, with different functionalities separated into their own files. Key modules include:
*   `src/camera.rs`: Handles camera controls (movement, rotation, zoom).
*   `src/ui.rs`: Contains UI elements and logic for the game setup screen.
*   `src/game.rs`: Defines game-specific enums (like `Faction`) and systems for game logic (e.g., assigning initial territories).
*   `src/map.rs`: Manages hexagonal grid generation and tile-related components.
The application utilizes Bevy's `AppState` system for managing different game states (SetupState, GameState).
