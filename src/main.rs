use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "sniffer")]
#[command(author = "Daniel Beach")]
#[command(version = "1.0")]
#[command(about = "sniffs flat files", long_about = None)]
struct Args {
    #[arg(long)]
    file_path: String,

    #[arg(long)]
    delimiter: String,

    #[arg(long, default_value_t = 0)]
    quote: u32,

    #[arg(long, default_value_t = 1)]
    check_nulls: u32,

    #[arg(long, default_value_t = 1)]
    check_whitespace: u32,
}

fn main() {
    let args = Args::parse();

    let file_path = args.file_path;
    let delimiter = args.delimiter;
    let quote = args.quote;
    let check_nulls = args.check_nulls;
    let check_whitespace = args.check_whitespace;

    sniffer::print_headers_few_lines_and_line_count(&file_path, &delimiter, &quote);
    if check_nulls == 1 {
        sniffer::check_all_column_for_nulls_and_whitespace(&file_path, &delimiter, &quote, &check_whitespace);
    }
    let size_in_mb = sniffer::get_file_size_in_mb(&file_path);
    println!("File size in MB: {}", size_in_mb);
}
