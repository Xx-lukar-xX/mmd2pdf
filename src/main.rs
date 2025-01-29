use mmd2pdf::{coordinate_determiner::*, mmdparser::*, parse_args::*, shape::*};
use printpdf::*;

fn main() {
    let config = get_args();
    let (graph_type, nodes) = parse_file(&config.filename);
    println!("graph_type: {}\nnodes: {:#?}", graph_type, nodes);
}
