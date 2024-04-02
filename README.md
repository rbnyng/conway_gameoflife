# Conway's Game of Life 

This project is an implementation of Conway's Game of Life, a cellular simulation devised by British mathematician John Conway in 1970. It simulates the life and death of cells on a rectangular grid based on a set of rules. This version is built in Rust.

![Screenshot of game](img/game.png?raw=true "Title")

## Features

- Configurable seed input for grid state initialization
- Control buttons to start, stop, and reset the simulation
- Ability to toggle cell colors between black and white
- Dynamic grid resizing based on the window size

## Installation

To build this from source, you need to have Rust and Cargo installed on your computer. If you don't have Rust installed, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

### Cloning the Repository

1. First, clone the repository to your local machine:

    ```sh
    git clone https://github.com/rbnyng/conway_gameoflife
    ```
2. Navigate to the project directory:
    ```sh
    cd conway_gameoflife
    ```

### Running the Simulation

To compile and run the simulation, use the following command from the project directory:
    ```sh
    cargo run
    ```

Or you can build it into an executable with:
    ```sh
    cargo build --release
    ```
	
### WebAssembly Deployment

To deploy the game as a WebAssembly application and see it in a web browser, use the following commands:

1. Install `trunk`:

    ```sh
    cargo install trunk
    ```

2. Install the required wasm target with:
    ```sh
    rustup target add wasm32-unknown-unknown
    ```

#### Web Local Testing

1. Build and serve the game locally on `http://127.0.0.1:8080` with:
    ```sh
    trunk serve
    ```

#### Web Deploy

1. Build the dist with:
    ```sh
    trunk build --release --public-url .
    ```

This generates a `dist` folder as the static html to deploy.

Alternatively, a workflow is included to automatically build and deploy to GitHub Pages.

## Usage

Upon running the game, you will be presented with a graphical interface displaying Conway's Game of Life.

- To start the simulation with a random seed, enter a number in the seed input field and click `Start`.
- Use `Stop` to pause the simulation and `Resume` to continue.
- Click `Toggle Cell Color` to switch between black and white cells.
- To learn more about the game and its rules, click `What is this?`.

## License

This project is open source and available under the [MIT License](LICENSE).

