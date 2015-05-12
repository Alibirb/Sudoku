use std::rc::Rc;
use std::sync::{Arc,Mutex};
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;

use std::isize;
use std::usize;

mod utils;


/*
Strategies:
	Use "pencil marks" (list all possible values for each space).

TODO:
	fn solve_row( puzzle : &mut [[usize; 9]; 9], row:usize)
	
	Use mutexes so that multiple routines can run in parallel.


*/




fn main() {
	
	let mut puzzle = load_puzzle("easy.puz");
	let solution = load_puzzle("easy_solution.puz");
	
	print_puzzle(puzzle);
	
	for i in 0..9 {
		for j in 0..9 {
			solve_single_value(&mut puzzle, i, j);
		}
	}
	// TODO: keep working until we're no longer making any progress.
	
	print_puzzle(puzzle);
	
	if puzzles_equal(puzzle, solution) {
		println!("Puzzle matches the given solution\n");
	} else {
		println!("Puzzle does not match the given solution\n");
		if has_mistake(puzzle, solution) {
			println!("And it filled in the wrong number.");
		} else {
			println!("But everything it filled in is correct.\n");
		}
	}
	
}




fn print_puzzle(puzzle : [[usize; 9]; 9]) {
	for &row in puzzle.iter() {
		print!("\t");
		for &i in row.iter() {
			if i != 0 {
				print!("{} ", i);
			} else {
				print!("  ");
			}
		}
		println!("");
	}
	println!("");
}


fn puzzles_equal(x : [[usize; 9]; 9], y : [[usize; 9]; 9]) -> bool {
	for i in 0..9 {
		for j in 0..9 {
			if x[i][j] != y[i][j] {
				return false;
			}
		}
	}
	return true;
}


fn has_mistake(puz : [[usize; 9]; 9], solution : [[usize; 9]; 9]) -> bool {
	for i in 0..9 {
		for j in 0..9 {
			if puz[i][j] != 0 && puz[i][j] != solution[i][j] {
				println!("mistake found in row {}, column {}", i, j);
				return true;
			}
		}
	}
	return false;
}


fn load_puzzle(filename : &str) -> [[usize; 9]; 9] {
	let mut puzzle : [[usize; 9]; 9] =
	[
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0],
		[0, 0, 0, 0, 0, 0, 0, 0, 0]
	];
	
	let file = File::open(&Path::new(filename));
	let reader = BufReader::new(file.unwrap());
	let mut row = 0usize;
	let mut column = 0usize;
	for line in reader.lines() {
		for number in line.unwrap().split_whitespace() {
			//puzzle[row][column] = from_str_radix(number, 10).unwrap();
			puzzle[row][column] = usize::from_str_radix(number, 10).unwrap();
			column += 1;
		}
		row += 1;
		column = 0;
	}
	
	return puzzle;
}







fn get_other_numbers_in_box(puzzle: & [[usize; 9]; 9], row:usize, column:usize) -> [usize; 8]
{
	let mut numbers : [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
	let mut index = 0;
	for i in (row-(row%3))..(row-(row%3)+3) {
		for j in (column-(column%3))..(column-(column%3)+3) {
			if i!=row || j!=column {
				numbers[index] = puzzle[i][j];
				index += 1;
			}
		}
	}
	//println!("for row {}, column {}, found {},{},{},{},{},{},{},{}", row, column, numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7]);
	
	return numbers;
}


fn solve_single_value( puzzle : &mut [[usize; 9]; 9], row:usize, column:usize) {
	
	if(puzzle[row][column] != 0) {
		return;
	}

	let mut value_is_taken : [bool; 10] = [false,false,false,false,false,false,false,false,false,false];
	
	for i in 0..9 {
		if puzzle[row][i] != 0 {
			let value = puzzle[row][i];
			value_is_taken[value] = true;
		}
		if puzzle[i][column] != 0 {
			let value = puzzle[i][column];
			value_is_taken[value] = true;
		}
	}
	// Check the 3x3 box as well.
	for &i in get_other_numbers_in_box(puzzle, row, column).iter() {
		if i != 0 {
			value_is_taken[i] = true;
		}
	}
	
	let mut count = 0isize;
	for &i in value_is_taken.iter() {
		if i {
			count += 1;
		}
	}
	
	//println!("row {}, column {}: count is {}.", row, column, count);
	
	if count == 8 {
		// Only one value is left, so we can fill it in.
		for i in 1..10 {
			if !value_is_taken[i] {
				//println!("row {}, column {}: available value is {}.", row, column, i);
				puzzle[row][column] = i;
			}
		}
	}
}




