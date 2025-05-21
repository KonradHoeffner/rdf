#![allow(unused_imports)]
use clap::{Parser, Subcommand};
use color_eyre::config::HookBuilder;
use color_eyre::eyre::{Report, WrapErr};
use hdt::{Hdt, HdtGraph};
//use sophia::api::prelude::{Triple, TripleSource};
use serde::Serialize;
//use sophia::turtle::parser::{nt, turtle};
use sophia::api::graph::Graph;
use sophia::api::prelude::TripleSource;
//use hdt::{Hdt, HdtGraph};
//use log::info;
use sophia::api::prelude::{Stringifier, TripleSerializer};
use sophia::api::term::matcher::{Any, TermMatcher};
// LightGraph loads 1M triples in 12s, LightGraph takes 18s to load but queries faster
use sophia::inmem::graph::LightGraph;
use sophia::turtle::parser::{nt, turtle};
use sophia::turtle::serializer::nt::NtSerializer;
use sophia::turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
use std::convert::Infallible;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
//use std::io::{BufReader, stdin};

//#[derive(Parser, Debug)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long)]
    quiet: bool,
    /*    // /// RDF Format of the output
        //format: Format,
        #[arg(short, long)]
        /// export as RDF Turtle, default is N-Triples
        turtle: bool,

        #[arg(short, long)]
        /// Count triples only, do not print them
        count: bool,

        // /// verbose output
        //verbose: bool,
        /// the HDT file to load from, if not given it is read from stdin
        hdt_input_file: Option<String>,
        /// the RDF file to create, if not given it is written to stdout
        rdf_output_file: Option<String>,
    */
}

#[derive(Debug, Clone, clap::ValueEnum, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Format {
    /// RDF/XML
    Rdfxml,
    /// Turtle Terse RDF Triple Language
    Turtle,
    Jsonld,
    Hdt,
    /// N-Triples
    #[default]
    Ntriples,
}

#[derive(Subcommand)]
enum Command {
    Convert {
        input_uri: PathBuf,
        /// Set the input format.
        #[arg(short, long)]
        input: Format,
        /// Set the output format.
        #[arg(short, long)]
        output: Format,
    },
    Info {
        //#[arg(short, long)]
        //count: u8,
        filename: PathBuf,
    },
    Test {
        filename: PathBuf,
    },
    Panic,
}

/// Load RDF graph from the RDF Turtle file specified in the config.
//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Report> {
    use Command::*;
    //use Format::*;
    let cli = Cli::parse();
    HookBuilder::default()
        .panic_section("consider reporting the bug at https://github.com/KonradHoeffner/rdf/issues")
        .display_env_section(false)
        .install()?;
    match cli.command {
        Panic => panic!("I'm panicking"),
        Info { filename } => {
            let file = File::open(filename.clone()).wrap_err_with(|| format!("Error opening input file {filename:?}"))?;
            let br = BufReader::new(file);
            let triples = turtle::parse_bufread(br).collect_triples();
            let graph: LightGraph = triples?;
            println!("~ {} triples", graph.triples().size_hint().0);
        }
        Test { filename } => {
            let file = File::open(filename.clone()).wrap_err_with(|| format!("Error opening input file {filename:?}"))?;
            Hdt::new(std::io::BufReader::new(file)).expect(&format!("Error loading HDT from {filename:?}"));
            //.wrap_err_with(|| format!("Error loading HDT from {filename:?}"))?
        }
        Convert { input_uri, input, output } => {
            if !cli.quiet {
                eprintln!("rdf: Parsing URI {input_uri:?} with parser {input:?}");
            }
            let file = File::open(input_uri.clone()).wrap_err_with(|| format!("Error opening input URI {input_uri:?}"))?;
            let br = BufReader::new(file);
            let triples = match input {
                Format::Turtle => turtle::parse_bufread(br).collect_triples(),
                Format::Ntriples => nt::parse_bufread(br).collect_triples(),
                //Hdt => nt::parse_bufread(br).collect_triples(),
                _ => todo!("parsing {:?} not implemented", input),
            };
            let graph: LightGraph = triples?;
            if !cli.quiet {
                eprintln!("rdf: Serializing with serializer {output:?}");
            }
            match output {
                Format::Ntriples => {
                    println!("{}", NtSerializer::new_stringifier().serialize_graph(&graph)?.to_string());
                }
                Format::Turtle => {
                    let config = TurtleConfig::new().with_pretty(false); // pretty printing is much slower

                    //.with_own_prefix_map(prefixes().clone());
                    println!(
                        "{}",
                        TurtleSerializer::new_stringifier_with_config(config)
                            .serialize_graph(&graph)
                            .wrap_err("error serializing graph as RDF Turtle")?
                            .to_string()
                    );
                }
                _ => {
                    panic!("unsupported serializer {output:?}");
                }
            }
            if !cli.quiet {
                eprintln!("rdf: Converting returned {} triples", graph.triples().size_hint().0);
            }
        }
    }
    //env_logger::init();
    //env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    //let args = Args::parse();
    /*let hdt = match args.hdt_input_file {
        Some(filename) => {
            let file =
                File::open(filename.clone()).wrap_err_with(|| format!("Error opening input file {}", filename))?;
            Hdt::new(std::io::BufReader::new(file))
                .wrap_err_with(|| format!("Error loading HDT from {}", filename))?
        }
        None => {
            let reader = BufReader::new(stdin());
            Hdt::new(reader).wrap_err("Error loading HDT from standard input")?
            //info!("Loaded from stdin {hdt:?}");
        }
    };
    let count = hdt.triples.len();
    let graph = HdtGraph::new(hdt);
    if args.count {
        println!("Parsing returned {} triples", count);
        return Ok(());
    }
    let s = match args.turtle {
        true => {
            let config = TurtleConfig::new().with_pretty(true);
            //.with_own_prefix_map(prefixes().clone());
            TurtleSerializer::new_stringifier_with_config(config)
                .serialize_graph(&graph)
                .wrap_err("error serializing graph as RDF Turtle")?
                .to_string()
        }
        false => {
            // Default: export the complete graph as N-Triples.
            NtSerializer::new_stringifier()
                .serialize_graph(&graph)
                .wrap_err("error serializing graph as N-Triples")?
                .to_string()
        }
    };
    println!("{s}");
    */
    Ok(())
}
