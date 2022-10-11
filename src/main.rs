use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let string_file = read_file("input.txt")?;
    let matrix_file = process_file(string_file);
    let matrix = count_mines(matrix_file);
    Ok(())
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_file(string_file: String) -> Vec<Vec<char>> {
    let mut matrix_file = Vec::new();
    for fila_str in string_file.as_str().split("\n") {
        let fila = fila_str.chars().collect::<Vec<char>>();
        matrix_file.push(fila);
    }
    matrix_file.pop();
    matrix_file
}

fn count_mines(board_matrix: Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut count_matrix = Vec::<Vec<usize>>::new();
    for (i, row) in board_matrix.iter().enumerate() {
        let mut counted_row = Vec::new();
        for (j, col) in row.iter().enumerate() {
            counted_row.push(count_neighbour_mines(&board_matrix, i, j, col));
        }
        count_matrix.push(counted_row);
    }
    println!("{:?}", count_matrix);
    count_matrix
}

fn count_neighbour_mines(board_matrix: &Vec<Vec<char>>, i: usize, j: usize, col: &char) -> usize {
    let mut count = 0_usize;
    if *col == '*' {
        return 0_usize;
    } else {
        for off_i in [-1_isize, 0, 1] {
            for off_j in [-1_isize, 0, 1] {
                match (
                    usize::try_from((i as isize) + off_i),
                    usize::try_from((j as isize) + off_j),
                ) {
                    (Ok(idx_i), Ok(idx_j)) => match board_matrix.get(idx_i) {
                        Some(visited_row) => match visited_row.get(idx_j) {
                            Some(char) => {
                                if *char == '*' {
                                    count += 1;
                                }
                            }
                            None => {}
                        },
                        None => {}
                    },
                    (_,_) => {},
                }

            }
        }
    }
    count
}
