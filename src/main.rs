mod board;
mod block;

use crate::board::{BOARD_SIZE, Board};
use crate::block::Block;
fn main() {
  print_test_board();
}

fn print_test_board() {
	let mut board = Board::new();
  let mut block = Block::new(56, 56);
  board.add_block_at(block, 0, 0);
  board.remove_block_at_current_index();
  board.print();

  println!("Press Enter to continue...");
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).unwrap();

  block = Block::new(29, 15);
  for y in 0..BOARD_SIZE / 2 / 14 {
    for x in 0..BOARD_SIZE / 28 {
      let success = board.add_block_at(block.clone(), x * block.width(), y * block.height());
      if !success {
        println!("Failed to add block at ({}, {})", x * block.width(), y * block.height());
      }
    }
  }
	block = Block::new(8, 14);
	for y in 0..BOARD_SIZE / 4 / block.height() {
    for x in 0..BOARD_SIZE / block.width() {
      let success = board.add_block_at(block.clone(), x * block.width(), y * block.height() + BOARD_SIZE / 2);
      if !success {
        println!("Failed to add block at ({}, {})", x * block.width(), y * block.height() + BOARD_SIZE / 2);
      }
    }
  }
	block = Block::new(7, 4);
	block.rotate();
	for y in 0..BOARD_SIZE / 4 / block.height() {
    for x in 0..BOARD_SIZE / block.width() {
      let success = board.add_block_at(block.clone(), x * block.width(), y * block.height() + 3 *BOARD_SIZE / 4);
      if !success {
        println!("Failed to add block at ({}, {})", x * block.width(), y * block.height() + 3 *BOARD_SIZE / 4);
      }
    }
  }
  board.print();
  println!("\n");
}
