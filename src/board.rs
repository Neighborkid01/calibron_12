use crate::Block;

pub const BOARD_SIZE: usize = 56;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
  x: usize,
  y: usize,
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Point { x, y }
  }

  pub fn parts(&self) -> (usize, usize) {
    (self.x, self.y)
  }
}

pub struct Board {
  cells: [bool; BOARD_SIZE * BOARD_SIZE],
  blocks: Vec<Block>,
  log_additions: bool,
}

impl Board {
  pub fn new(log_additions: bool) -> Self {
    Board {
      cells: [false; BOARD_SIZE * BOARD_SIZE],
      blocks: Vec::new(),
      log_additions,
    }
  }

  pub fn print(&self) {
    self.debug_print(None, None);
  }

  fn debug_print(&self, current_position: Option<Point>, current_block: Option<&Block>) {
    use crossterm::{cursor, execute, terminal};
    use std::io::stdout;

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();


    let double_board_size = BOARD_SIZE * 2;
    println!("┌{}┐", "─".repeat(double_board_size));

    if let Some(position) = current_position {
      execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
      let (x, y) = position.parts();
      if let Some(block) = current_block {
        println!("Current index: ({}, {}) Current block: {}x{} ", x, y, block.width(), block.height());
      } else {
        println!("Current index: ({}, {})", x, y);
      }
    }

    for _ in 0..BOARD_SIZE {
      println!("│{}│", " ".repeat(double_board_size));
    }
    println!("└{}┘", "─".repeat(double_board_size));

    for block in &self.blocks {
      block.draw(&mut stdout);
    }

    execute!(stdout, cursor::MoveTo(0, (BOARD_SIZE + 2) as u16)).unwrap();
  }

  pub fn add_block_at(&mut self, mut block: Block, x: usize, y: usize) -> bool {
    let (width, height) = (block.width(), block.height());

    if self.log_additions {
      self.debug_print(Some(Point::new(x, y)), Some(&block));
      println!("Press Enter to continue...");
      let mut input = String::new();
      std::io::stdin().read_line(&mut input).unwrap();
    }

    if x + width > BOARD_SIZE || y + height > BOARD_SIZE {
      return false;
    }

    for dy in 0..height {
      let row_start = self.xy_to_index(x, y + dy);
      let any_occupied = self.cells[row_start..row_start + width].iter().any(|&cell| cell);
      if any_occupied { return false; }
    }

    block.set_position(Point::new(x, y));
    self.mark_block_occupied(&block, x, y);
    self.blocks.push(block);
    true
  }

  pub fn remove_block_at(&mut self, x: usize, y: usize) {
    self.mark_block_unoccupied(x, y);
    self.blocks.retain(|b| b.position() != Some(Point::new(x, y)));
  }

  fn xy_to_index(&self, x: usize, y: usize) -> usize {
    y * BOARD_SIZE + x
  }

  fn block_at(&self, x: usize, y: usize) -> Option<&Block> {
    self.blocks.iter().find(|b| b.position() == Some(Point::new(x, y)))
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

  pub fn cell_is_occupied(&self, x: usize, y: usize) -> bool {
    self.cells[self.xy_to_index(x, y)]
  }
}
