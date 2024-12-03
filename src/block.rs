use crate::board::{BOARD_SIZE, Board};

#[derive(Clone)]
pub enum Orientation {
  Horizontal,
  Vertical,
}

#[derive(Clone)]
pub struct Block {
	width: usize,
  height: usize,
  orientation: Orientation,
  index: Option<usize>,
}

impl Block {
  pub fn new(width: usize, height: usize) -> Self {
    Block { width, height, orientation: Orientation::Horizontal, index: None }
  }

	pub fn width(&self) -> usize {
		match self.orientation {
			Orientation::Horizontal => self.width,
			Orientation::Vertical => self.height,
		}
	}

	pub fn height(&self) -> usize {
		match self.orientation {
			Orientation::Horizontal => self.height,
			Orientation::Vertical => self.width,
		}
	}

  pub fn index(&self) -> Option<usize> {
    self.index
  }

  pub fn set_index(&mut self, index: usize) {
    self.index = Some(index);
  }

  pub fn rotate(&mut self) {
    self.orientation = match self.orientation {
      Orientation::Horizontal => Orientation::Vertical,
      Orientation::Vertical => Orientation::Horizontal,
    };
  }

  pub fn draw(&self, stdout: &mut std::io::Stdout, board: &Board) {
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
