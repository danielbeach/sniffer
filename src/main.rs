use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let delimiter = &args[2];
    let lines = sniffer::read_number_lines_in_file(&file_path);
    println!("number of lines: {}", lines);
    sniffer::print_headers(&file_path, &delimiter);
}
