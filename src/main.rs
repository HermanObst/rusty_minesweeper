use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

enum Input {
    Mine,
    Empty,
}

fn main() -> Result<(), std::io::Error> {
    let string_file = read_file("input.txt")?;
    let matrix_file = process_file(&string_file);
    let matrix_counted = count_mines(&matrix_file);
    println!("{:?}", matrix_counted);
    Ok(())
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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

fn count_mines(board_matrix: &[Vec<Input>]) -> Vec<Vec<usize>> {
    let mut count_matrix = Vec::<Vec<usize>>::new();
    for (i, row) in board_matrix.iter().enumerate() {
        let mut counted_row = Vec::new();
        for (j, col) in row.iter().enumerate() {
            counted_row.push(count_neighbour_mines(board_matrix, i, j, col));
        }
        count_matrix.push(counted_row);
    }
    count_matrix
}

fn count_neighbour_mines(board_matrix: &[Vec<Input>], i: usize, j: usize, col: &Input) -> usize {
    let i = isize::try_from(i).unwrap_or(std::isize::MAX);
    let j = isize::try_from(j).unwrap_or(std::isize::MAX);
    let mut count = 0;
    if let Input::Mine = col {
        return 0;
    }
    for off_i in [-1_isize, 0, 1] {
        for off_j in [-1_isize, 0, 1] {
            if let (Ok(idx_i), Ok(idx_j)) = (usize::try_from(i + off_i), usize::try_from(j + off_j))
            {
                if let Some(visited_row) = board_matrix.get(idx_i) {
                    if let Some(char) = visited_row.get(idx_j) {
                        {
                            if let Input::Mine = char {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
