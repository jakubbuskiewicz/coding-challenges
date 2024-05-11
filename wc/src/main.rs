fn main() {
    let option: String = std::env::args().nth(1).expect("no option given");
    let path: String = std::env::args().nth(2).expect("no path given");

    if option == "c" {
        let number_of_bytes: usize =
            count_number_of_bytes_in_file(&path).expect("could not count bytes in file");
        return println!("{:?} {:?}", number_of_bytes, path);
    } else if option == "l" {
        let number_of_lines: usize =
            count_number_of_lines_in_file(&path).expect("could not count lines in file");
        return println!("{:?} {:?}", number_of_lines, path);
    } else if option == "w" {
        let number_of_words: usize =
            count_number_of_words_in_file(&path).expect("could not count words in file");
        return println!("{:?} {:?}", number_of_words, path);
    } else if option == "m" {
        let number_of_characters: usize =
            count_number_of_characters_in_file(&path).expect("could not count characters in file");
        return println!("{:?} {:?}", number_of_characters, path);
    } else {
        panic!("unknown option {:?}", option);
    }
}

fn count_number_of_bytes_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let mut file: File = File::open(path)?;
    let mut buffer: Vec<u8> = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer.len())
}

fn count_number_of_lines_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut number_of_lines: usize = 0;

    for _line in reader.lines() {
        number_of_lines += 1;
    }

    Ok(number_of_lines)
}

fn count_number_of_words_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut number_of_words: usize = 0;
    for line in reader.lines() {
        let line = line?;
        number_of_words += line.split_whitespace().count();
    }

    Ok(number_of_words)
}

fn count_number_of_characters_in_file(path: &str) -> Result<usize, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut number_of_characters: usize = 0;
    for line in reader.lines() {
        let line = line?;
        number_of_characters += line.char_indices().count();
    }

    // number of characters will differ from wc command as Rust counts CRLF as LF
    // TODO: test file for CRLF / LF before reading
    Ok(number_of_characters)
}
