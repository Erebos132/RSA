use std::fs;
use std::io::Write;

pub fn export_data(data: String, path: &str) {
    let mut file = fs::File::create(path).unwrap();
    write!(file, "{}", data);
}

pub fn make_data(matrix: Vec<Vec<u128>>) -> String {
    let mut output_string = String::new();
    for row in matrix {
        for column in row {
            output_string += &format!("{},", column);
        }
        output_string += "\n";
    }
    return output_string;
}
