const BOARD_SIZE: usize = 56;

struct Board {
  cells: [bool; BOARD_SIZE * BOARD_SIZE],
  current_index: usize,
  blocks: Vec<Block>,
}

impl Board {
  fn new() -> Self {
    Board {
      cells: [false; BOARD_SIZE * BOARD_SIZE],
      current_index: 0,
      blocks: Vec::new(),
    }
  }

  fn print(&self) {
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

    println!("\n");
  }

  fn xy_to_index(&self, x: usize, y: usize) -> usize {
    y * BOARD_SIZE + x
  }

	fn index_to_xy(&self, index: usize) -> (usize, usize) {
		(index % BOARD_SIZE, index / BOARD_SIZE)
	}
}

#[derive(Clone)]
enum Orientation {
  Horizontal,
  Vertical,
}

#[derive(Clone)]
struct Block {
	width: usize,
  height: usize,
  orientation: Orientation,
  index: Option<usize>,
}

impl Block {
  fn new(width: usize, height: usize) -> Self {
    Block { width, height, orientation: Orientation::Horizontal, index: None }
  }

	fn width(&self) -> usize {
		match self.orientation {
			Orientation::Horizontal => self.width,
			Orientation::Vertical => self.height,
		}
	}

	fn height(&self) -> usize {
		match self.orientation {
			Orientation::Horizontal => self.height,
			Orientation::Vertical => self.width,
		}
	}

  fn set_index(&mut self, index: usize) {
    self.index = Some(index);
  }

  fn draw(&self, stdout: &mut std::io::Stdout, board: &Board) {
    use crossterm::{cursor, execute, style::{self}};

    if let Some(index) = self.index {
      let (block_x, block_y) = board.index_to_xy(index);
      let (width, height) = (self.width(), self.height());

      if block_x + width <= BOARD_SIZE && block_y + height <= BOARD_SIZE {
        // Draw top border
        execute!(
          stdout,
          cursor::MoveTo((1 + block_x * 2) as u16, (1 + block_y) as u16),
          style::Print("┌"),
          style::Print("─".repeat(width * 2 - 2)),
          style::Print("┐")
        ).unwrap();

        // Draw dimensions in top row
        let dimensions = format!("{}x{}", self.width, self.height);
        let dimensions_len = dimensions.len();
        execute!(
          stdout,
          cursor::MoveTo((1 + block_x * 2) as u16, (1 + block_y + 0) as u16 + 1),
          style::Print("│"),
          style::Print(dimensions),
          style::Print(" ".repeat(width * 2 - 2 - dimensions_len)),
          style::Print("│")
        ).unwrap();

        // Draw middle rows
        for y in 2..height-1 {
          execute!(
            stdout,
            cursor::MoveTo((1 + block_x * 2) as u16, (1 + block_y + y) as u16),
            style::Print("│"),
            style::Print(" ".repeat(width * 2 - 2)),
            style::Print("│")
          ).unwrap();
        }

        // Draw bottom border
        execute!(
          stdout,
          cursor::MoveTo((1 + block_x * 2) as u16, (1 + block_y + height - 1) as u16),
          style::Print("└"),
          style::Print("─".repeat(width * 2 - 2)),
          style::Print("┘")
        ).unwrap();
      }
    }
  }
}

fn main() {
  print_test_board();
}

fn print_test_board() {
	let mut board = Board::new();
  let mut block = Block::new(28, 14);
  for y in 0..BOARD_SIZE / 2 / block.height {
    for x in 0..BOARD_SIZE / block.width {
      board.blocks.push(block.clone());
      let i = board.blocks.len() - 1;
      let cell_index = board.xy_to_index(x * block.width, y * block.height);
      board.blocks[i].set_index(cell_index);
    }
  }
	block = Block::new(7, 4);
	block.orientation = Orientation::Vertical;
	for y in 0..BOARD_SIZE / 2 / block.height() {
    for x in 0..BOARD_SIZE / block.width() {
      board.blocks.push(block.clone());
      let i = board.blocks.len() - 1;
      let cell_index = board.xy_to_index(x * block.width(), y * block.height() + BOARD_SIZE / 2);
      board.blocks[i].set_index(cell_index);
    }
  }
  board.print();
}
