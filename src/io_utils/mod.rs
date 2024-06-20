use std::fs::File;
use std::io::{BufWriter, Write};

pub fn save_matrix_i32(matrix: &Vec<Vec<i32>>, file_path: &str) {
    let mut writer =
        BufWriter::new(File::create(file_path).expect("Failed to create the output file"));

    for row in matrix {
        for elem in &row[..(row.len()-1)] {
            writer.write(format!("{},", elem).as_bytes()).expect("Failed to write data.");
        }
        writer.write(format!("{}", row.last().unwrap()).as_bytes()).expect("Failed to write data");
        writer.write("\n".as_bytes()).expect("Failed to write data");
    }
}

// pub fn save_matrix_i32(matrix: &Vec<[i32]>, file_path: &str) {
//     let mut writer =
//         BufWriter::new(File::create(file_path).expect("Failed to create the output file"));
//
//     for row in matrix {
//         for elem in row {
//             writer.write(format!("{}", elem).as_bytes()).expect("Failed to write data.");
//         }
//         writer.write("\n".as_bytes())
//     }
// }