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
}

fn main() {
    let args = Args::parse();

    let file_path = args.file_path;
    let delimiter = args.delimiter;
    let quote = args.quote;

    let lines = sniffer::read_number_lines_in_file(&file_path);
    println!("number of lines: {}", lines);
    sniffer::print_headers(&file_path, &delimiter);
    sniffer::print_a_few_lines(&file_path, &delimiter, 3);
}
