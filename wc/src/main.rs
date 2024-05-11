fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    if pattern == "c" {
        let number_of_bytes: usize =
            count_number_of_bytes_in_file(&path).expect("could not count bytes in file");
        return println!("{:?} {:?}", number_of_bytes, path);
    } else if pattern == "l" {
        let number_of_lines: usize =
            count_number_of_lines_in_file(&path).expect("could not count lines in file");
        return println!("{:?} {:?}", number_of_lines, path);
    }
}

fn count_number_of_bytes_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    Ok(buffer.len())
}

fn count_number_of_lines_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut number_of_lines = 0;
    for _line in reader.lines() {
        number_of_lines += 1;
    }

    Ok(number_of_lines)
}
