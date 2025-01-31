use mmd2pdf::*;
use std::process::Command;

fn main() {
    let config = parse_args::get_args();
    let (graph_type, nodes) = mmdparser::parse_file(&config.filename).unwrap();
    let nodes = coordinate_determiner::corrdinate_determine(nodes, &graph_type);
    pdf_generator::makepdf(&nodes, &config, &graph_type);
    Command::new("open")
        .arg(config.output.unwrap())
        .spawn()
        .expect("Failed to open the file");
}
