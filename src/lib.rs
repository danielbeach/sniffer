use std::{io::{BufRead, BufReader}, fs};


pub fn read_number_lines_in_file(file_path: &str) -> u32 {
    let mut count = 0;
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    for _ in bf.lines() {
        count += 1;
    }
    count
}

fn remove_quotes(input: &str) -> String {
    input.replace("\"", "")
}

fn split_line<'a>(line: &'a str, delimiter: &'a str) -> Vec<&'a str> {
    let line = &line.split(delimiter).collect::<Vec<&str>>();
    return line.to_vec();
}

pub fn print_headers(file_path: &str, delimiter: &str, &quote: &u32) {
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    let mut count = 0;
    for line in bf.lines() {
        let line = line.unwrap();
        if quote == 1 {
            let line = remove_quotes(&line);
            println!("Headers: {:?}", split_line(&line, delimiter));
            println!("{}", "");
            break;
        }
        println!("Headers: {:?}", &line);
        break;
    }
}


pub fn print_a_few_lines(file_path: &str, delimiter: &str, &quote: &u32, number_of_lines: u32) {
    let file = std::fs::File::open(file_path).unwrap();
    let bf = BufReader::new(file);
    let mut count = 0;
    for line in bf.lines() {
        if count == 0 {
            // skip the first line
            count += 1;
            continue;
        }
        let line = line.unwrap();
        if quote == 1 {
            let line = remove_quotes(&line);
            println!("Row: {:?}", split_line(&line, delimiter));
            println!("{}", "");
            count += 1;
            if count == number_of_lines {
                break;
            }
            continue;
        }
        println!("Row: {:?}", line);
        println!("{}", "");
        count += 1;
        if count == number_of_lines {
            break;
        }
    }
}

pub fn get_file_size_in_mb(file_path: &str) ->f64 {
    let metadata = fs::metadata(file_path).expect("Error reading file metadata");
    let file_size = metadata.len() as f64;
    let mb_size = file_size / (1024.0 * 1024.0);
    mb_size
}