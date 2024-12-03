use crate::Block;

pub const BOARD_SIZE: usize = 56;

pub struct Board {
  cells: [bool; BOARD_SIZE * BOARD_SIZE],
  current_index: usize,
  blocks: Vec<Block>,
}

impl Board {
  pub fn new() -> Self {
    Board {
      cells: [false; BOARD_SIZE * BOARD_SIZE],
      current_index: 0,
      blocks: Vec::new(),
    }
  }

  pub fn print(&self) {
    use crossterm::{cursor, execute, terminal};
    use std::io::stdout;

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();

		let double_board_size = BOARD_SIZE * 2;
    println!("┌{}┐", "─".repeat(double_board_size));
    for _ in 0..BOARD_SIZE {
      println!("│{}│", " ".repeat(double_board_size));
    }
    println!("└{}┘", "─".repeat(double_board_size));

    for block in &self.blocks {
      block.draw(&mut stdout, &self);
    }
  }

  pub fn add_block_at(&mut self, mut block: Block, x: usize, y: usize) -> bool {
    let (width, height) = (block.width(), block.height());

    if x + width > BOARD_SIZE || y + height > BOARD_SIZE {
      return false;
    }

    for dy in 0..height {
      let row_start = self.xy_to_index(x, y + dy);
      let any_occupied = self.cells[row_start..row_start + width].iter().any(|&cell| cell);
      if any_occupied { return false; }
    }

    block.set_index(self.xy_to_index(x, y));
    self.mark_block_occupied(&block, x, y);
    self.blocks.push(block);
    true
  }

  pub fn remove_block_at_current_index(&mut self) {
    let (x, y) = self.index_to_xy(self.current_index);
    self.remove_block_at(x, y);
  }

  pub fn remove_block_at(&mut self, x: usize, y: usize) {
    let index = self.xy_to_index(x, y);
    self.mark_block_unoccupied(x, y);
    self.blocks.retain(|b| b.index() != Some(index));
  }

	pub fn index_to_xy(&self, index: usize) -> (usize, usize) {
    (index % BOARD_SIZE, index / BOARD_SIZE)
	}

  fn xy_to_index(&self, x: usize, y: usize) -> usize {
    y * BOARD_SIZE + x
  }

  fn block_at(&self, x: usize, y: usize) -> Option<&Block> {
    self.blocks.iter().find(|b| b.index() == Some(self.xy_to_index(x, y)))
  }

  fn mark_block_occupied(&mut self, block: &Block, x: usize, y: usize) {
    for dy in 0..block.height() {
      for dx in 0..block.width() {
        let cell_index = self.xy_to_index(x + dx, y + dy);
        self.cells[cell_index] = true;
      }
    }
  }

    fn mark_block_unoccupied(&mut self, x: usize, y: usize) {
    let some_block = self.block_at(x, y);
    if let None = some_block { return; }

    let block = some_block.unwrap();
    let (width, height) = (block.width(), block.height());
    for dy in 0..height {
      for dx in 0..width {
        let cell_index = self.xy_to_index(x + dx, y + dy);
        self.cells[cell_index] = false;
      }
    }
  }
}
