use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;



pub fn print_puzzle(puzzle : [[usize; 9]; 9]) {
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

pub fn puzzles_equal(x : [[usize; 9]; 9], y : [[usize; 9]; 9]) -> bool {
	for i in 0..9 {
		for j in 0..9 {
			if x[i][j] != y[i][j] {
				return false;
			}
		}
	}
	return true;
}


pub fn has_mistake(puz : [[usize; 9]; 9], solution : [[usize; 9]; 9]) -> bool {
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


pub fn load_puzzle(filename : &str) -> [[usize; 9]; 9] {
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

pub fn copy_puzzle(puzzle : [[usize; 9]; 9]) -> [[usize; 9]; 9] {
	let mut copy : [[usize; 9]; 9] = 
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
	
	for i in 0..9 {
		for j in 0..9 {
			copy[i][j] = puzzle[i][j];
		}
	}
	
	return puzzle;
}



pub fn get_other_numbers_in_row(puzzle: & [[usize; 9]; 9], row:usize, column:usize) -> [usize; 8]
{
	let mut numbers : [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
	let mut index = 0;
	for j in 0..9 {
		if j!=column {
			numbers[index] = puzzle[row][j];
			index += 1;
		}
	}
	//println!("for row {}, column {}, found {},{},{},{},{},{},{},{}", row, column, numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7]);
	
	return numbers;
}

pub fn get_other_numbers_in_column(puzzle: & [[usize; 9]; 9], row:usize, column:usize) -> [usize; 8]
{
	let mut numbers : [usize; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
	let mut index = 0;
	for i in 0..9 {
		if i!=row {
			numbers[index] = puzzle[i][column];
			index += 1;
		}
	}
	//println!("for row {}, column {}, found {},{},{},{},{},{},{},{}", row, column, numbers[0], numbers[1], numbers[2], numbers[3], numbers[4], numbers[5], numbers[6], numbers[7]);
	
	return numbers;
}

pub fn get_other_numbers_in_box(puzzle: & [[usize; 9]; 9], row:usize, column:usize) -> [usize; 8]
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