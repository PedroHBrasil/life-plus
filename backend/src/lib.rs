use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
    n_cells_x: usize,
    n_cells_y: usize,
    cells: Vec<bool>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(n_cells_x: usize, n_cells_y: usize) -> Self {
        Self {
            n_cells_x,
            n_cells_y,
            cells: vec![false; n_cells_y * n_cells_x],
        }
    }

    pub fn n_cells_x(&self) -> usize {
        self.n_cells_x
    }

    pub fn n_cells_y(&self) -> usize {
        self.n_cells_y
    }

    pub fn cells(&self) -> *const bool {
        self.cells.as_ptr()
    }

    pub fn update_population(&mut self) {
        let mut new_population = vec![false; self.n_cells_y * self.n_cells_x];
        for i in 0..self.n_cells_x {
            for j in 0..self.n_cells_y {
                let neighbor_count = self.count_alive_neighbors(i, j);
                let index = self.get_index(i, j);
                let lives = self.apply_rules(self.cells[index], neighbor_count);
                new_population[index] = lives;
            }
        }
        self.cells = new_population;
    }

    fn count_alive_neighbors(&self, i: usize, j: usize) -> u8 {
        // Determines next and previous cell indices assuming a full loop world
        let i_prev = if i == 0 { self.n_cells_x - 1 } else { i - 1 };
        let j_prev = if j == 0 { self.n_cells_y - 1 } else { j - 1 };
        let i_next = if i == self.n_cells_x - 1 { 0 } else { i + 1 };
        let j_next = if j == self.n_cells_y - 1 { 0 } else { j + 1 };
        // Counts alive neighbors
        let mut count = 0;
        for i_cur in [i_prev, i, i_next] {
            for j_cur in [j_prev, j, j_next] {
                if i_cur == i && j_cur == j { continue }
                if self.cells[self.get_index(i_cur, j_cur)] { count += 1 }
            }
        }

        count
    }

    fn get_index(&self, i: usize, j: usize) -> usize {
        i * (self.n_cells_x as usize) + j
    }

    fn apply_rules(&self, is_alive: bool, neighbor_count: u8) -> bool {
        // This line summarizes the original rules found at https://conwaylife.com/
        neighbor_count == 3 || (neighbor_count == 2 && is_alive)
    }
}
