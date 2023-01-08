use clap::Parser;
use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use surveyor::all_metadata;

/// Outputs a markdown document which contains information for all tests
#[derive(Debug, Parser)]
struct Args {
    #[arg(long,short)]
    output: Option<String>,
}

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
        write!(
            out,
            "
### {}

- Description: {}
- Tags: {}

Code:
```
{}
```
",
            metadata.title,
            metadata.description,
            metadata.tags,
            p.code_block()
        )
        .unwrap();
    }
}
