## sniffer

<img src="https://github.com/danielbeach/sniffer/blob/0ca48931cacf052ad3bce881f6e2847c58e0f97c/imgs/sniff.png" width="300">

`sniffer` is a tool to quickly inspect `csv` and `flat-file` files for basic information.

Need to see how many rows are in a `csv` file?
Want to see the first few rows printed out to your terminal?

Then `sniffer` is for you!

`sniffer` is built with `Rust` and is made for the average
Data Engineering or data person who frequently need to inspect
`csv` files quicky.

The follow data is displayed about a `flat file` by default.
- file size in `mb`.
- number of `lines` per file.
- `header` row is displayed.
- First few `rows` are printed.
- Option to indciate of flat-file is quoted.
- Option to check all columns for `NULL` values.

### Usage
```
Usage: sniffer [OPTIONS] --file-path <FILE_PATH> --delimiter <DELIMITER>

Options:
      --file-path <FILE_PATH>
      --delimiter <DELIMITER>
      --quote <QUOTE>          [default: 0]
      --check-nulls <CHECK_NULLS>  [default: 0]
  -h, --help                   Print help
  -V, --version                Print version
  ```

To use `sniffer` to inspect a `flat-file`
simply pass the `file-path` and `delimiter`.
`cargo run -- --file-path sample.csv  --delimiter , --quote 1 --check-nulls 1`
This will give you output something like ...
```
Headers: StringRecord(["ride_id", "rideable_type", "started_at", "ended_at", "start_station_name", "start_station_id", "end_station_name", "end_station_id", "start_lat", "start_lng", "end_lat", "end_lng", "member_casual"])

'Row: StringRecord(["CBCD0D7777F0E45F", "classic_bike", "2023-02-14 11:59:42", "2023-02-14 12:13:38", "Southport Ave & Clybourn Ave", "TA1309000030", "Clark St & Schiller St", "TA1309000024", "41.920771", "-87.663712", "41.907993", "-87.631501", "casual"])

'Row: StringRecord(["F3EC5FCE5FF39DE9", "electric_bike", "2023-02-15 13:53:48", "2023-02-15 13:59:08", "Clarendon Ave & Gordon Ter", "13379", "Sheridan Rd & Lawrence Ave", "TA1309000041", "41.957879424", "-87.649583697", "41.969517", "-87.654691", "casual"])

'Row: StringRecord(["E54C1F27FA9354FF", "classic_bike", "2023-02-19 11:10:57", "2023-02-19 11:35:01", "Southport Ave & Clybourn Ave", "TA1309000030", "Aberdeen St & Monroe St", "13156", "41.920771", "-87.663712", "41.880419", "-87.655519", "member"])

number of lines: 4
No columns with nulls
File size in MB: 0.001027107238769531
```

## Testing and CI, Building.
To run `pre-commit` checks ...
`pre-commit run --all-files`
