use mmd2pdf::*;

fn main() {
    let config = parse_args::get_args();
    let (graph_type, nodes) = mmdparser::parse_file(&config.filename).unwrap();
    let nodes = coordinate_determiner::corrdinate_determine(nodes, &graph_type);
    pdf_generator::makepdf(&nodes, &config, &graph_type);
}
