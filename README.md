## sniffer

<img src="https://github.com/danielbeach/sniffer/blob/0ca48931cacf052ad3bce881f6e2847c58e0f97c/imgs/sniff.png" width="300">

`sniffer` is a tool to quickly inspect `csv` and `flat-file` files.

Need to see how many rows are in a `csv` file?
Want to see the first few rows printed out to your terminal?

Then `sniffer` is for you!

`sniffer` is built with `Rust` and is made for the average 
Data Engineering or data person who frequently need to inspect
`csv` files quicky.

### Usage
To use `sniffer` to inspect a `flate-file` simply pass the `file-path` and `delimiter`.
`cargo run sample.csv ,`
This will give you output something like ...
```
number of lines: 3
Headers: ["header_1", "header_2"]
```
