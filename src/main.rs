mod utils;
use utils::print_puzzle;
use utils::load_puzzle;
use utils::puzzles_equal;
use utils::has_mistake;
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
	
	let mut puzzle = load_puzzle("easy.puz");
	let solution = load_puzzle("easy_solution.puz");
	
	print_puzzle(puzzle);
	
	let mut made_progress = true;
	while(made_progress)
	{
		made_progress = false;
		
		for i in 0..9 {
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



// Tries to solve a single cell. Returns true if it managed to solve the cell.
fn solve_cell(puzzle : &mut [[usize; 9]; 9], row:usize, column:usize) -> bool {
	
	if(puzzle[row][column] != 0) {
		return false;	// cell is already solved
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
				return true;
			}
		}
	}
	
	return false;
}




