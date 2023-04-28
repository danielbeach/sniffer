use std::process;

use sniffer::*;


fn main() {
    let args:Args = Args::new();

    sniffer::print_headers_few_lines_and_line_count(&args);
    
    if args.check_nulls() == &1 {
        sniffer::check_all_column_for_nulls_and_whitespace(&args);
    }
    
    let size_in_mb = sniffer::get_file_size_in_mb(&args.file_path()).unwrap_or_else(|err|
    {
        println!("Error getting file size: {}", err);
        process::exit(1);
    } );
    
    println!("File size in MB: {}", size_in_mb);
}
