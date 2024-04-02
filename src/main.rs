use eframe::egui;
use chrono::{Duration, Local};
use rand::{rngs::StdRng, Rng, SeedableRng};

struct GameOfLife {
    grid: Vec<Vec<bool>>,
    last_update: chrono::DateTime<Local>,
    running: bool,
    seed: u64,
    seed_input: String, // use this as the input and convert it to the actual seed
    black_cells: bool,
    show_explanation: bool
}

impl GameOfLife {
    fn new(size: usize, seed: u64) -> Self {
        let game = Self {
            grid: vec![vec![false; size]; size],
            last_update: Local::now(),
            running: false,
            seed,
            seed_input: seed.to_string(),
            black_cells: true,
            show_explanation: false,
        };
        game
    }

    fn reset_grid(&mut self, seed: u64) {
        let size = self.grid.len();
        let mut rng = StdRng::seed_from_u64(seed);
        for i in 0..size {
            for j in 0..size {
                self.grid[i][j] = rng.gen_bool(0.5); // 50% chance of being alive
            }
        }
    }

    fn update_game_logic(&mut self) {
        let mut new_grid = self.grid.clone();
        let size = self.grid.len();

        for i in 0..size {
            for j in 0..size {
                let live_neighbors = self.live_neighbor_count(i, j);
                new_grid[i][j] = matches!(
                    (self.grid[i][j], live_neighbors),
                    (true, 2) | (_, 3)
                );
            }
        }

        self.grid = new_grid;
    }

    fn live_neighbor_count(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        for i in row.saturating_sub(1)..=row + 1 {
            if i >= self.grid.len() {
                continue;
            }
            for j in col.saturating_sub(1)..=col + 1 {
                if j >= self.grid[i].len() || (i == row && j == col) {
                    continue;
                }
                if self.grid[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    fn update(&mut self, ctx: &egui::Context) {
        let window_size = ctx.screen_rect().size();
        let cell_size = 10.0; 
        let new_grid_width = (window_size.x / cell_size).floor() as usize;
        let new_grid_height = (window_size.y / cell_size).floor() as usize;
        if new_grid_width != self.grid.len() || new_grid_height != self.grid[0].len() {
            self.resize_grid(new_grid_width, new_grid_height);
        }

        if self.running && Local::now().signed_duration_since(self.last_update) >= Duration::milliseconds(100) {
            self.update_game_logic();
            self.last_update = Local::now();
        }
    }

    fn resize_grid(&mut self, new_width: usize, new_height: usize) {
        let mut new_grid = vec![vec![false; new_width]; new_height];
        for i in 0..std::cmp::min(new_height, self.grid.len()) {
            for j in 0..std::cmp::min(new_width, self.grid[i].len()) {
                new_grid[i][j] = self.grid[i][j];
            }
        }
        self.grid = new_grid;
    }

    fn render(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Conway's Game of Life");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Seed:");
                    ui.text_edit_singleline(&mut self.seed_input);
                    if ui.button("Start").clicked() {
                        if let Ok(new_seed) = self.seed_input.parse::<u64>() {
                            self.seed = new_seed;
                            self.reset_grid(self.seed);
                            self.running = true;
                        }
                    }
                    if ui.button(if self.running { "Stop" } else { "Resume" }).clicked() {
                        self.running = !self.running;
                    }
                });
            });
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                if ui.button("Toggle Cell Color").clicked() {
                    self.black_cells = !self.black_cells;
                }
                
                ui.add_space(20.0);

                if ui.button("What is this?").clicked() {
                    self.show_explanation = true;
                }
            });
            ui.add_space(5.0);
            ui.separator();

            if self.show_explanation {
                egui::Window::new("Conway's Game of Life")
                    .open(&mut self.show_explanation)
                    .show(ctx, |ui| {
                        ui.label("Conway's Game of Life is a cellular automaton invented by John Horton Conway in 1970.
                        \nIt consists of a grid of cells which, based on a few mathematical rules, can live, die or multiply. 
                        \nThe game is a zero-player game, meaning that its evolution is determined by its initial state, requiring no further input. 
                        \nPlayers interact with the Game of Life by creating an initial configuration and observing how it evolves.
                        \nAny live cell with fewer than two live neighbors dies, as if by underpopulation.
                        \nAny live cell with two or three live neighbors lives on to the next generation.
                        \nAny live cell with more than three live neighbors dies, as if by overpopulation.
                        \nAny dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.");
                    });
            }

            let (_response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::click());

            let cell_size = egui::Vec2::splat(10.0);

            for (i, row) in self.grid.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let x = j as f32 * cell_size.x;
                    let y = i as f32 * cell_size.y;
                    let rect = egui::Rect::from_min_size(egui::Pos2::new(x, y), cell_size);
                    let color = if cell {
                        if self.black_cells { egui::Color32::BLACK } else { egui::Color32::WHITE }
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    painter.rect_filled(rect, 0.0, color);
                }
            }
        });
    }  
}

impl eframe::App for GameOfLife {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Conway's Game of Life");
                self.update(ctx);
                self.render(ctx);
            });
            ui.separator();});}}

// native app
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Conway's Game of Life",
        options,
        Box::new(|_cc| Box::new(GameOfLife::new(20, 42))),
    );
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    let options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                options,
                Box::new(|_cc| Box::new(GameOfLife::new(20, 42))),
            )
            .await
            .expect("failed to start eframe");
    });
}
