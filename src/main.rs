mod utils;
use utils::print_puzzle;
use utils::load_puzzle;
use utils::puzzles_equal;
use utils::has_mistake;
use utils::get_other_numbers_in_row;
use utils::get_other_numbers_in_column;
use utils::get_other_numbers_in_box;

/*
Strategies:
	Use "pencil marks" (list all possible values for each space).

TODO:
	fn solve_row( puzzle : &mut [[usize; 9]; 9], row:usize)
	
	Use mutexes so that multiple routines can run in parallel.

terminology: (see https://en.wikipedia.org/wiki/Glossary_of_Sudoku)
	cell: where a single number goes
	box: 3x3 arrangement of cells
	stack: 3 vertically-stacked boxes
	band: 3 horizontally-connected boxes
	chute: either a band or a stack

*/




fn main() {
	
	let mut puzzle = load_puzzle("medium.puz");
	let solution = load_puzzle("medium_solution.puz");
	
	print_puzzle(puzzle);
	
	let mut made_progress = true;
	while made_progress
	{
		made_progress = false;
		
		for i in 0..9 {
			if solve_row(&mut puzzle, i) {
				made_progress = true;
			}
			if solve_column(&mut puzzle, i) {
				made_progress = true;
			}
			for j in 0..9 {
				if solve_cell(&mut puzzle, i, j) {
					made_progress = true;
				}
			}
		}
		
		print_puzzle(puzzle);
	}
	
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

// Attempts to solve the given row. Returns true if it solved at least one cell.
fn solve_row(puzzle : &mut [[usize; 9]; 9], row:usize) -> bool
{
	let mut cells_filled = 0;	// To keep track of how many cells we managed to fill.
	
	// determine what values are already taken.
	let mut values_taken: Vec<usize> = Vec::new();
	for i in 0..9 {
		if puzzle[row][i] != 0 {
			values_taken.push(puzzle[row][i]);
		}
	}
	
	// Determine which values are still needed.
	let mut values_needed: Vec<usize> = Vec::new();
	for value in 1usize..10usize {
		let mut taken = false;
		for &i in values_taken.iter() {
			if i == value {
				taken = true;
				break;
			}
		}
		if !taken {
			values_needed.push(value);
		}
	}
	
	if values_needed.len() == 0 {
		return false;	// no blank spaces left to fill in
	}
	if values_needed.len() == 1 {
		// only one space is blank. Find it, and fill it in.
		for i in 0..9 {
			if puzzle[row][i] == 0 {
				puzzle[row][i] = values_needed[0];
				//println!("Filling in {} in row {}, column {}", values_needed[0], row, i);
				return true;
			}
		}
	}
	
	// Go through the values we need, and check each available spot to see if that value could go there. If there's only spot that can accomodate that value, then it must go there.
	for &value in values_needed.iter() {
		let mut possible_cells: Vec<usize> = Vec::new();
		for col in 0..9 {
			if puzzle[row][col] != 0 {
				continue;	// cell already taken.
			}
			
			let mut taken = false;
			for &i in get_other_numbers_in_column(puzzle, row, col).iter() {
				if i == value {
					taken = true;	// value already exists in column.
					break;
				}
			}
			for &i in get_other_numbers_in_box(puzzle, row, col).iter() {
				if i == value {
					taken = true;	// value already exists in box
					break;
				}
			}
			if !taken {
				possible_cells.push(col);	
			}
		}
		
		if possible_cells.len() == 1 {
			// Only one place this value can go.
			puzzle[row][possible_cells[0]] = value;
			cells_filled += 1;
			//println!("Filling in a cell");
		}
	}
	
	return cells_filled != 0;
}

// Attempts to solve the given column. Returns true if it solved at least one cell.
fn solve_column(puzzle : &mut [[usize; 9]; 9], column:usize) -> bool
{
	let mut cells_filled = 0;	// To keep track of how many cells we managed to fill.
	
	// determine what values are already taken.
	let mut values_taken: Vec<usize> = Vec::new();
	for i in 0..9 {
		if puzzle[i][column] != 0 {
			values_taken.push(puzzle[i][column]);
		}
	}
	
	// Determine which values are still needed.
	let mut values_needed: Vec<usize> = Vec::new();
	for value in 1usize..10usize {
		let mut taken = false;
		for &i in values_taken.iter() {
			if i == value {
				taken = true;
				break;
			}
		}
		if !taken {
			values_needed.push(value);
		}
	}
	
	if values_needed.len() == 0 {
		return false;	// no blank spaces left to fill in
	}
	if values_needed.len() == 1 {
		// only one space is blank. Find it, and fill it in.
		for i in 0..9 {
			if puzzle[i][column] == 0 {
				puzzle[i][column] = values_needed[0];
				//println!("Filling in {} in row {}, column {}", values_needed[0], row, i);
				return true;
			}
		}
	}
	
	// Go through the values we need, and check each available spot to see if that value could go there. If there's only spot that can accomodate that value, then it must go there.
	for &value in values_needed.iter() {
		let mut possible_cells: Vec<usize> = Vec::new();
		for row in 0..9 {
			if puzzle[row][column] != 0 {
				continue;	// cell already taken.
			}
			
			let mut taken = false;
			for &i in get_other_numbers_in_row(puzzle, row, column).iter() {
				if i == value {
					taken = true;	// value already exists in row.
					break;
				}
			}
			for &i in get_other_numbers_in_box(puzzle, row, column).iter() {
				if i == value {
					taken = true;	// value already exists in box
					break;
				}
			}
			if !taken {
				possible_cells.push(row);	
			}
		}
		
		if possible_cells.len() == 1 {
			// Only one place this value can go.
			puzzle[possible_cells[0]][column] = value;
			cells_filled += 1;
			//println!("Filling in a cell");
		}
	}
	
	return cells_filled != 0;
}




// Tries to solve a single cell. Returns true if it managed to solve the cell.
fn solve_cell(puzzle : &mut [[usize; 9]; 9], row:usize, column:usize) -> bool {
	
	if puzzle[row][column] != 0 {
		return false;	// cell is already solved
	}

	let mut value_is_taken : [bool; 10] = [false,false,false,false,false,false,false,false,false,false];
	
	// Check what numbers are in the cell's row, column, and box.
	for &i in get_other_numbers_in_row(puzzle, row, column).iter() {
		if i != 0 {
			value_is_taken[i] = true;
		}
	}
	for &i in get_other_numbers_in_column(puzzle, row, column).iter() {
		if i != 0 {
			value_is_taken[i] = true;
		}
	}
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
				return true;
			}
		}
	}
	
	return false;
}




