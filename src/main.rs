use clap::Parser;
use std::collections::HashSet;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

mod ast;
mod codegen;
mod parser;

/// Simple graph parser with visualization options and export to dot format
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file
    input: String,
    /// Directed graph
    #[clap(short, long)]
    directed: bool,
    /// Use node id as default label
    #[clap(short, long)]
    autolabel: bool,
    /// Default node shape
    #[clap(short, long, default_value = "circle")]
    shape: String,
    /// Output file (dot, png, jpeg, svg, pdf...)
    #[clap(short, long)]
    output: Option<String>,
    /// Set layout engine
    #[clap(short, long, default_value = "neato")]
    layout: String,
}

fn main() {
    let args = Args::parse();
    let src: String = fs::read_to_string(args.input).expect("Cannot read input file");
    let ast = parser::parse(&src).expect("Failed to parse graph").1;
    let dot = codegen::graph_to_str(&ast, args.directed, args.autolabel, args.shape);
    match args.output {
        None => println!("{}", dot),
        Some(output_file) => {
            let path = Path::new(&output_file);
            match path.extension() {
                None => fs::write(path, dot.as_bytes()).expect("Failed to write output"),
                Some(ext) => {
                    let ext = ext.to_str().unwrap();
                    if !HashSet::from([
                        "eps", "gif", "jpg", "jpeg", "json", "pdf", "png", "ps", "svg", "webp",
                    ])
                    .contains(ext)
                    {
                        fs::write(path, dot.as_bytes()).expect("Failed to write output")
                    } else {
                        assert!(
                            HashSet::from([
                                "dot",
                                "neato",
                                "fdp",
                                "sfdp",
                                "circo",
                                "twopi",
                                "osage",
                                "patchwork"
                            ])
                            .contains(args.layout.as_str()),
                            "Invalid layout engine: {}",
                            args.layout
                        );
                        let mut cmd = Command::new("dot")
                            .arg(format!("-K{}", args.layout))
                            .arg(format!("-T{}", ext))
                            .arg(format!("-o{}", output_file))
                            .stdin(Stdio::piped())
                            .spawn()
                            .expect("Failed to execute `dot`");
                        cmd.stdin
                            .take()
                            .expect("Failed to get stdin")
                            .write_all(dot.as_bytes())
                            .expect("Failed to write to stdin");
                        cmd.wait().expect("Failed to wait on `dot`");
                    }
                }
            }
        }
    }
}
