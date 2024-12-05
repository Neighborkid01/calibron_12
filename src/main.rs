mod board;
mod block;

use crate::board::{BOARD_SIZE, Board};
use crate::block::Block;
use std::time::Instant;
use std::collections::HashSet;
fn main() {
  solve_calibron_12();
}

fn solve_calibron_12() {
  let all_blocks = vec![
    Block::new(32, 11),
    Block::new(32, 10),
    Block::new(28, 14),
    Block::new(28, 7),
    Block::new(28, 6),
    Block::new(21, 18),
    Block::new(21, 18),
    Block::new(21, 14),
    Block::new(21, 14),
    Block::new(17, 14),
    Block::new(14, 4),
    Block::new(10, 7),
  ];

  let mut board = Board::new(false);
  let start = Instant::now();
  solve_step(&mut board, &all_blocks, false);
  let duration = start.elapsed();
  println!("Biggest first");
  board.print();
  println!("Time elapsed: {:?}", duration);
  println!();

  let mut board = Board::new(false);
  let all_blocks = all_blocks.into_iter().rev().collect::<Vec<_>>();
  let start = Instant::now();
  solve_step(&mut board, &all_blocks, false);
  let duration = start.elapsed();
  println!("Smallest first");
  println!("Time elapsed: {:?}", duration);
  println!();

  println!("Random");
  let mut durations = Vec::new();
  let all_blocks = all_blocks.into_iter().rev().collect::<Vec<_>>();
  for _ in 0..20 {
    let mut board = Board::new(false);
    let start = Instant::now();
    solve_step(&mut board, &all_blocks, true);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    durations.push(duration);
  }

  let min = durations.iter().min().unwrap();
  let max = durations.iter().max().unwrap();
  let mean = durations.iter().sum::<std::time::Duration>() / durations.len() as u32;

  println!("Min time: {:?}", min);
  println!("Max time: {:?}", max);
  println!("Mean time: {:?}", mean);
}

fn solve_step(board: &mut Board, all_blocks: &Vec<Block>, randomize: bool) -> bool {
  if all_blocks.is_empty() { return true; }
  let mut blocks_to_try = Vec::new();
  if randomize {
    blocks_to_try = all_blocks
      .into_iter()
      .collect::<HashSet<_>>()
      .into_iter()
      .collect::<Vec<_>>();
  } else {
    for block in all_blocks {
      if !blocks_to_try.contains(&block) { blocks_to_try.push(block); }
    }
  }

  for y in 0..BOARD_SIZE {
    for x in 0..BOARD_SIZE {
      if board.cell_is_occupied(x, y) { continue; }
      for block in blocks_to_try {
        let mut block = block.clone();
        let mut blocks = all_blocks.clone();
        if let Some(idx) = blocks.iter().position(|b| *b == block) {
          blocks.remove(idx);
        }

        if add_block_at(board, block, &blocks, x, y, randomize) { return true; }
        block.rotate();
        if add_block_at(board, block, &blocks, x, y, randomize) { return true; }
      }
      return false;
    }
  }
  false
}

fn add_block_at(board: &mut Board, block: Block, blocks: &Vec<Block>, x: usize, y: usize, randomize: bool) -> bool {
  if board.add_block_at(block, x, y) {
    if solve_step(board, &blocks, randomize) {
      return true;
    } else {
      board.remove_block_at(x, y);
    }
  }
  false
}

#[allow(dead_code)]
fn print_test_board() {
  let mut board = Board::new(false);
  let mut block = Block::new(56, 56);
  board.add_block_at(block, 0, 0);
  board.remove_block_at(0, 0);

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
