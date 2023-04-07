use clap::Parser;
use sniffer::Args;


fn main() {
    let args = Args::parse();

    sniffer::print_headers_few_lines_and_line_count(&args.file_path, &args.delimiter, &args.quote);
    if args.check_nulls == 1 {
        sniffer::check_all_column_for_nulls_and_whitespace(&args.file_path, &args.delimiter, &args.quote, &args.check_whitespace);
    }
    let size_in_mb = sniffer::get_file_size_in_mb(&args.file_path);
    println!("File size in MB: {}", size_in_mb);
}
