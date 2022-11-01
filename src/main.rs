mod input;
mod output;
use input::Input;
use output::Output;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let string_file = read_file("input.txt")?;
    let matrix_file = process_file(&string_file);
    let matrix_counted = count_mines(&matrix_file);
    print_board(&matrix_counted);
    Ok(())
}

// reads a file containing a board and convers it to a String.
fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Given a string containing the information of the board
// Construct a matix of type Input.
fn process_file(string_file: &str) -> Vec<Vec<Input>> {
    let mut matrix_file = Vec::new();
    for fila_str in string_file.split('\n') {
        let fila = fila_str
            .chars()
            .map(|c| match c {
                '*' => Input::Mine,
                _ => Input::Empty,
            })
            .collect::<Vec<Input>>();
        matrix_file.push(fila);
    }
    matrix_file.pop();
    matrix_file
}

// Given a matrix of Input, returns a matrix of Output
fn count_mines(board_matrix: &[Vec<Input>]) -> Vec<Vec<Output>> {
    let mut count_matrix = Vec::<Vec<Output>>::new();
    for (i, row) in board_matrix.iter().enumerate() {
        let mut counted_row = Vec::new();
        for (j, col) in row.iter().enumerate() {
            counted_row.push(count_neighbour_mines(board_matrix, i, j, col));
        }
        count_matrix.push(counted_row);
    }
    count_matrix
}

// Given a specific cell of the board matrix
// Iterates over all its neighbours and returns an Output type of:
// - The number of mines around that cell, if the cell is empty
// - The type Output. 
fn count_neighbour_mines(board_matrix: &[Vec<Input>], i: usize, j: usize, col: &Input) -> Output {
    let i = isize::try_from(i).unwrap_or(std::isize::MAX);
    let j = isize::try_from(j).unwrap_or(std::isize::MAX);
    let mut count = 0;
    if let Input::Mine = col {
        return Output::Mine;
    }
    for off_i in [-1_isize, 0, 1] {
        for off_j in [-1_isize, 0, 1] {
            if let (Ok(idx_i), Ok(idx_j)) = (usize::try_from(i + off_i), usize::try_from(j + off_j))
            {
                if let Some(visited_row) = board_matrix.get(idx_i) {
                    if let Some(Input::Mine) = visited_row.get(idx_j) {
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    Output::Empty(count)
}

// Prints the matrix of outputs
fn print_board(counted_board: &[Vec<Output>]) {
    for row in counted_board {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_reading() {
        let result = read_file("input.txt").unwrap();
        assert_eq!(result, "·*·*·\n··*··\n··*··\n·····\n");
    }

    #[test]
    fn test_process_file() {
        let processed_file = process_file("·*·*·\n··*··\n··*··\n·····\n");
        let result = [
            [
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Mine,
                Input::Empty,
            ],
            [
                Input::Empty,
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Empty,
            ],
            [
                Input::Empty,
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Empty,
            ],
            [
                Input::Empty,
                Input::Empty,
                Input::Empty,
                Input::Empty,
                Input::Empty,
            ],
        ];
        assert_eq!(processed_file, result);
    }

    #[test]
    fn test_count_mines() {
        let input = [
            vec![
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Mine,
                Input::Empty,
            ],
            vec![
                Input::Empty,
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Empty,
            ],
            vec![
                Input::Empty,
                Input::Empty,
                Input::Mine,
                Input::Empty,
                Input::Empty,
            ],
            vec![
                Input::Empty,
                Input::Empty,
                Input::Empty,
                Input::Empty,
                Input::Empty,
            ],
        ];
        let output = count_mines(&input);
        let expected_output = vec![
            vec![
                Output::Empty(1),
                Output::Mine,
                Output::Empty(3),
                Output::Mine,
                Output::Empty(1),
            ],
            vec![
                Output::Empty(1),
                Output::Empty(3),
                Output::Mine,
                Output::Empty(3),
                Output::Empty(1),
            ],
            vec![
                Output::Empty(0),
                Output::Empty(2),
                Output::Mine,
                Output::Empty(2),
                Output::Empty(0),
            ],
            vec![
                Output::Empty(0),
                Output::Empty(1),
                Output::Empty(1),
                Output::Empty(1),
                Output::Empty(0),
            ],
        ];
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_count_neighbour_mines() {
        let board_matrix = vec![
            vec![
                Input::Empty,
                Input::Mine,
                Input::Empty,
            ],
            vec![
                Input::Empty,
                Input::Mine,
                Input::Empty,
            ],
            vec![
                Input::Empty,
                Input::Empty,
                Input::Empty,
            ]
        ];
        let output_1 = count_neighbour_mines(&board_matrix, 1, 0, &board_matrix[1][0]);
        let output_2 = count_neighbour_mines(&board_matrix, 1, 1, &board_matrix[1][1]);
        assert_eq!(output_1, Output::Empty(2));
        assert_eq!(output_2, Output::Mine);

    }

    #[test]
    fn integration_test() {
        let string_file = read_file("input.txt").unwrap();
        let matrix_file = process_file(&string_file);
        let matrix_counted = count_mines(&matrix_file); 
        let expected_output = vec![
            vec![
                Output::Empty(1),
                Output::Mine,
                Output::Empty(3),
                Output::Mine,
                Output::Empty(1),
            ],
            vec![
                Output::Empty(1),
                Output::Empty(3),
                Output::Mine,
                Output::Empty(3),
                Output::Empty(1),
            ],
            vec![
                Output::Empty(0),
                Output::Empty(2),
                Output::Mine,
                Output::Empty(2),
                Output::Empty(0),
            ],
            vec![
                Output::Empty(0),
                Output::Empty(1),
                Output::Empty(1),
                Output::Empty(1),
                Output::Empty(0),
            ],
        ];
        assert_eq!(matrix_counted, expected_output); 
    }
}
