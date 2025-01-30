use mmd2pdf::*;

fn main() {
    let config = parse_args::get_args();
    let (graph_type, nodes) = mmdparser::parse_file(&config.filename).unwrap();
    println!("config: {:#?}", config);
    println!("graph_type: {}\nnodes: {:#?}", graph_type, nodes);
    pdf_generator::makepdf();
    coordinate_determiner::corrdinate_determine(&nodes);
}
