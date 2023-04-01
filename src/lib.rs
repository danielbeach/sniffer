use std::{fs, io::BufReader};

pub fn get_file_size_in_mb(file_path: &str) -> f64 {
    let metadata: fs::Metadata = fs::metadata(file_path).expect("Error reading file metadata");
    let file_size: f64 = metadata.len() as f64;
    let mb_size: f64 = file_size / (1024.0 * 1024.0);
    mb_size
}
pub fn check_all_column_for_nulls(file_path: &str, delimiter: &str, &quote: &u32) {
    let file: fs::File = std::fs::File::open(file_path).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if delimiter == "," { b',' } else { b'\t' })
        .double_quote(match quote == 1 {
            true => true,
            false => false,
        })
        .from_reader(bf);
    let mut columns_with_nulls: Vec<String> = Vec::new();
    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        for field in record.iter() {
            if field.is_empty() {
                columns_with_nulls.push(String::from(field));
            }
        }
    }
    if !columns_with_nulls.is_empty() {
        println!("Found columns with NULL values: {:?}", columns_with_nulls);
    } else {
        println!("No columns with nulls");
    }
}

pub fn print_headers_few_lines_and_line_count(file_path: &str, delimiter: &str, &quote: &u32) {
    let file: fs::File = std::fs::File::open(file_path).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if delimiter == "," { b',' } else { b'\t' })
        .double_quote(match quote == 1 {
            true => true,
            false => false,
        })
        .from_reader(bf);
    let headers: &csv::StringRecord = rdr.headers().expect("Error reading headers");
    println!("Headers: {:?}", headers);
    println!(" ");
    let mut count: u32 = 0;
    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        if count < 3 {
            println!("'Row: {:?}", record);
            println!(" ");
        }
        count += 1;
    }
    println!("number of lines: {}", count);
}
