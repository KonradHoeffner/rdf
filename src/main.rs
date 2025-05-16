use clap::{Parser, Subcommand};
use color_eyre::config::HookBuilder;
use color_eyre::eyre::{Report, WrapErr};
//use hdt::{Hdt, HdtGraph};
//use log::info;
//use sophia::api::prelude::{Stringifier, TripleSerializer};
//use sophia::turtle::serializer::nt::NtSerializer;
//use sophia::turtle::serializer::turtle::{TurtleConfig, TurtleSerializer};
//use sophia::turtle::parser::{nt, turtle};
use sophia::api::graph::Graph;
use sophia::api::prelude::{Triple, TripleSource};
use sophia::inmem::graph::FastGraph;
use sophia::turtle::parser::turtle;
use std::fs::File;
use std::io::BufReader;
//use std::io::{BufReader, stdin};

/*enum Format {
    NTriples,
    RdfXml,
    Turtle,
}*/

//#[derive(Parser, Debug)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    //#[arg(short, long)]
    //count: u8,
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

#[derive(Subcommand)]
enum Command {
    Info {
        //#[arg(short, long)]
        //count: u8,
        filename: String,
    },
    Panic,
}

//fn main() -> Result<(), Box<dyn std::error::Error>> {
fn main() -> Result<(), Report> {
    use Command::*;
    let cli = Cli::parse();
    HookBuilder::default()
        .panic_section("consider reporting the bug at https://github.com/KonradHoeffner/rdf/issues")
        .display_env_section(false)
        .install()?;
    match cli.command {
        Panic => panic!("I'm panicking"),
        Info { filename } => {
            let file = File::open(filename.clone())
                .wrap_err_with(|| format!("Error opening input file {}", filename))?;
            let br = BufReader::new(file);
            let triples = turtle::parse_bufread(br).collect_triples();
            let graph: FastGraph = triples?;
            println!("~ {} triples", graph.triples().size_hint().0);
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
