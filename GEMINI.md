# Gemini Project: wss_rp_test

## Project Overview

This is a simple 3D scene built using the Bevy game engine in Rust. The application renders a single cube on a textured floor. The cube can be controlled with the "WASD" keys and has inertia. The camera follows the cube, rotates around it when the left mouse button is pressed, and zooms in/out with the mouse wheel.

## Controls

*   **W, A, S, D:** Move the cube. The cube has inertia, so it will accelerate and decelerate.
*   **Left Mouse Button + Mouse Movement:** Rotate the camera around the cube.
*   **Mouse Wheel:** Zoom the camera in and out.

## Building and Running

To build and run the project, use the following command:

```bash
cargo run
```

This will compile the necessary dependencies and launch the application window.

## Development Conventions

The project follows a modular structure, with different functionalities separated into their own files. For instance, the camera control logic is encapsulated in `src/camera.rs`, the cube control logic is in `src/cube.rs`, and the floor generation is in `src/floor.rs`. This promotes code organization and reusability.
