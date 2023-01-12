use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::{stdout, BufWriter, Write};
use std::process::Command;
use surveyor::all_metadata;

/// Outputs a markdown document which contains information for all tests
#[derive(Debug, Parser)]
struct Args {
    #[arg(long, short)]
    output: Option<String>,
}

const TEMP_FILENAME: &'static str = "temp.rs";

fn main() {
    let args = Args::parse();

    let mut out: Box<dyn Write> = if let Some(file) = args.output {
        let f = File::create(file).unwrap();
        Box::new(BufWriter::new(f))
    } else {
        Box::new(stdout())
    };

    for p in all_metadata() {
        let metadata = p.metadata();

        // easiest to save code block to a file
        // then format it, wish I could this more easily programmatically.
        let mut temp = File::create(TEMP_FILENAME).unwrap();
        write!(temp, "{}", p.code_block()).unwrap();
        Command::new("rustfmt").arg(TEMP_FILENAME).output().unwrap();
        let code_block = read_to_string(TEMP_FILENAME).unwrap();

        write!(
            out,
            "### {}

- Description: {}
- Version: {}
- Tags: {}

Code:
```rust
{}
```
",
            metadata.title,
            metadata.description,
            p.semver(),
            metadata.tags,
            code_block,
        )
        .unwrap();
    }
}
