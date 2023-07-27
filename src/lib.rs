use std::{fs, io::BufReader, error::Error};
use clap::Parser;
// import CreateBuilder
use deltalake::operations::create::CreateBuilder;
use deltalake::DeltaTable;
use deltalake::schema::SchemaField;
use deltalake::schema::SchemaDataType;
use deltalake::writer::json::JsonWriter;
use deltalake::writer::DeltaWriter;
use serde_json::json;
use csv;
use serde_json::Value;


#[derive(Parser, Debug)]
#[command(name = "sniffer")]
#[command(author = "Daniel Beach")]
#[command(version = "1.0")]
#[command(about = "sniffs flat files", long_about = None)]
pub struct Args {
    #[arg(long, short='t', default_value_t = 0)]
    delta: u16,

    #[arg(long, short='p', default_value_t =  String::from("./delta"))]
    delta_path: String,

    #[arg(long, short='f')]
    file_path: String,

    #[arg(long, short='d', default_value_t = String::from(","))]
    delimiter: String,

    #[arg(long,short='q', default_value_t = 0)]
    quote: u32,

    #[arg(long, short='n', default_value_t = 1)]
    check_nulls: u32,

    #[arg(long, short='w', default_value_t = 1)]
    check_whitespace: u32,
}

impl Args {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn file_path(&self) -> &str {
        &self.file_path
    }

    pub fn delta(&self) -> &u16 {
        &self.delta
    }

    pub fn delta_path(&self) -> &str {
        &self.delta_path
    }

    pub fn delimiter(&self) -> &str {
        &self.delimiter
    }

    pub fn quote(&self) -> bool {
        if &self.quote == &1 {
            return true
        } 

        false
    }

    pub fn check_nulls(&self) -> &u32 {
        &self.check_nulls
    }

    pub fn check_whitespace(&self) -> &u32 {
        &self.check_whitespace
    }
    
}
    




/// Get a file size in MB
/// 
/// # Arguments
/// 
/// * `file_path` - A string slice that holds the path to the file
/// 
/// # Returns
/// * Result<f64, Box<dyn Error>> - A result that is either a file size  or an error message
/// 
pub fn get_file_size_in_mb(file_path: &str) -> Result<f64, Box<dyn Error>> {
    let metadata: fs::Metadata = fs::metadata(file_path)?;
    let file_size: f64 = metadata.len() as f64;
    let mb_size: f64 = file_size / (1024.0 * 1024.0);
    Ok(mb_size)
}

/// check whitespace at beginning or end of string
/// 
/// # Arguments
///
///  * `s` - A string slice that we want to check for whitespace at beginning or end
///
///  # Returns
///
/// * `Result<bool, &'static str>` - A result that is either a boolean or an error message
fn has_whitespace_at_beginning_or_end(s: &str) -> Result<bool,&'static str> {

    if s.len() == 0 {
        return Ok(false);
    }

    let c = s.chars().take(1).last().expect("Error getting first character");
    if c.is_whitespace()  {
        return Ok(true);
    }
    let c = s.chars().rev().take(1).last().expect("Error getting last character");
    if c.is_whitespace() {
        return Ok(true);
    }
    
    Ok(false)
}

pub fn check_all_column_for_nulls_and_whitespace(args:&Args) {
    let file: fs::File = get_file_handler(args.file_path()).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);

    // get delimiter byte
    // Allow support for future delimiters 
    let delimiter_byte = match args.delimiter() {
        "," => b',',
        "\t" => b'\t',
        _ => b',',
    };
    
    let mut rdr: csv::Reader<BufReader<fs::File>> = csv::ReaderBuilder::new()
        .delimiter( delimiter_byte )
        .double_quote(args.quote())
        .from_reader(bf);
    let mut columns_with_nulls: Vec<String> = Vec::new();
    let mut has_whitespace: Vec<bool> = Vec::new();
    for result in rdr.records() {
        let record: csv::StringRecord = result.unwrap();
        for field in record.iter() {
            if field.is_empty() {
                columns_with_nulls.push(String::from(field));
            }
            if args.check_whitespace() == &1 {
                if has_whitespace_at_beginning_or_end(field).unwrap() {
                    has_whitespace.push(true);
                }
            }
        }
    }
    if !columns_with_nulls.is_empty() {
        println!("Found columns with NULL values: {:?}", columns_with_nulls);
    } else {
        println!("No columns with nulls");
    }
    if args.check_whitespace() == &1 {
   
        if has_whitespace.len() > 0 {
            println!("Found columns with whitespace at beginning or end");
        } else {
            println!("No columns with whitespace at beginning or end");
        }
    }
}

pub fn print_headers_few_lines_and_line_count(args:&Args) {
    let file = get_file_handler(args.file_path()).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if args.delimiter() == "," { b',' } else { b'\t' })
        .double_quote(args.quote())
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

// get file handler
//
// # Arguments
//
// * `file_path` - A string slice that is the path to the file
//
// # Returns
//
// * `Result<fs::File,Box<dyn Error>>` - A result that is either a file handler or an error message
fn get_file_handler(file_path: &str) -> Result<fs::File,Box<dyn Error>> {
    let file: fs::File = fs::File::open(file_path)?;
    Ok(file)
}

pub async fn convert_csv_to_delta_lake(args:&Args) {
    let file = get_file_handler(args.file_path()).unwrap();
    let bf: BufReader<fs::File> = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(if args.delimiter() == "," { b',' } else { b'\t' })
        .double_quote(args.quote())
        .from_reader(bf);
    let mut schema_fields: Vec<SchemaField> = Vec::new();

    let headers: &csv::StringRecord = rdr.headers().expect("Error reading headers");
    for header in headers.iter() {
        schema_fields.push(SchemaField::new(header.to_string(), 
        SchemaDataType::primitive("string".to_string()), true, Default::default()));
    }
    let mut table: DeltaTable = CreateBuilder::new().with_location(args.delta_path()).with_columns(schema_fields).await.unwrap();
    let mut wrtr = JsonWriter::for_table(&table).unwrap();
    let mut reader = csv::Reader::from_path(args.file_path()).unwrap();
    let mut map: serde_json::Map<String,Value> = serde_json::Map::new();

    for result in reader.records() {
        let record: csv::StringRecord = result.unwrap();
        let mut values = Vec::new();
        for (i, field) in record.iter().enumerate() {
            map.insert(headers[i].to_string(), json!(field));
        }
        values.push(json!(map));
        println!("values: {:?}", values);
        wrtr.write(values).await.unwrap();
    }

    wrtr.flush_and_commit(&mut table).await.unwrap();
    println!("Done writing CSV to Delta Lake at {}", args.delta_path());
    
}
    





#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE_NAME: &str = "test.csv";

    fn create_csv_file_for_testing() -> String {
        let file_path: String = String::from(TEST_FILE_NAME);
        let mut wtr = csv::Writer::from_path(file_path.clone()).unwrap();
        wtr.write_record(&["a", "b", "c"]).unwrap();
        wtr.write_record(&["1", "2", "3"]).unwrap();
        wtr.write_record(&["4", "5", "6"]).unwrap();
        wtr.flush().unwrap();
        file_path
    }

    fn remove_file_after_testing() -> ()
    {
        let file_path: String = String::from(TEST_FILE_NAME);
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_leading_space() {
        let s: &str = "  hello";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_trailing_space() {
        let s: &str = "hello  ";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_without_spaces() {
        let s: &str = "hello";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_both_leading_and_trailing_spaces() {
        let s: &str = "  hello  ";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_whitespace_at_beginning_or_end_with_empty_string() {
        let s: &str = "";
        let result: bool = has_whitespace_at_beginning_or_end(s).unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_file_handler() {
        let file_path: String = create_csv_file_for_testing();
        let result: fs::File = get_file_handler(&file_path).unwrap();
        assert_eq!(result.metadata().unwrap().len(), 18);
        remove_file_after_testing();
    }

    #[test]
    fn test_get_file_handler_with_non_existent_file() {
        let file_path: &str = "non_existent_file.csv";
        let result: Result<fs::File,Box<dyn Error>> = get_file_handler(file_path);
        assert!(result.is_err());
    }

    #[test]
    #[should_panic]
    fn test_get_file_handler_with_empty_string() {
        let file_path: &str = "";
        let _result: fs::File = get_file_handler(file_path).unwrap();
    }

    #[test]
    fn test_delimiter_without_default_is_tab() {
        let args: Args = Args::parse_from(&["sniffer", "-f", "test.csv", "-d", "\t"]);
        let result: &str = args.delimiter();
        assert_eq!(result, "\t");
    }

    #[test]
    fn test_delimiter_with_default_is_comma() {
        let args: Args = Args::parse_from(&["sniffer", "-f", "test.csv"]);
        let result: &str = args.delimiter();
        assert_eq!(result, ",");
    }

    #[test]
    fn test_quote_with_default_is_false() {
        let args: Args = Args::parse_from(&["sniffer", "-f", "test.csv"]);
        let result: bool = args.quote();
        assert_eq!(result, false);
    }

    #[test]
    fn test_quote_with_true_is_true() {
        let args: Args = Args::parse_from(&["sniffer", "-f", "test.csv", "-q", "1"]);
        let result: bool = args.quote();
        assert_eq!(result, true);
    }

}