// 文字、座標などからpdfファイルを生成する

use crate::mmdparser::Element;
use crate::parse_args::Config;
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

const MAX_WIDTH: f32 = 100.0;
const MAX_HEIGHT: f32 = 20.0;
const LINE_MAX_CHAR_NUM: usize = 10;
const MARGIN: f32 = 10.0;

pub fn makepdf(nodes: &Vec<Element>, config: &Config, graph_type: &str) {
    let x_size = nodes
        .iter()
        .map(|node| node.x.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        + if graph_type == "TD" {
            MAX_HEIGHT
        } else {
            MAX_WIDTH
        } / 2.0
        + MARGIN;
    let y_size = nodes
        .iter()
        .map(|node| node.y.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        + if graph_type == "TD" {
            MAX_WIDTH
        } else {
            MAX_HEIGHT
        } / 2.0
        + MARGIN;
    let (doc, page1, layer1) = PdfDocument::new("mmd2pdf", Mm(x_size), Mm(y_size), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc
        .add_external_font(File::open("assets/fonts/GenShinGothic-Regular.ttf").unwrap())
        .unwrap();

    let rect = Rect::new(Mm(92.0), Mm(1.0), Mm(x_size), Mm(y_size))
        .with_mode(PaintMode::Stroke)
        .with_winding(WindingOrder::EvenOdd);
    current_layer.add_rect(rect);

    // ノードの描画
    for node in nodes {
        let mut text = node.text.clone();
        if text.chars().count() > LINE_MAX_CHAR_NUM {
            let mut char_indices = text.char_indices();
            let mut insert_pos = 0;
            for _ in 0..LINE_MAX_CHAR_NUM {
                if let Some((pos, _)) = char_indices.next() {
                    insert_pos = pos;
                }
            }
            text.insert(insert_pos, '\n');
        }
        current_layer.use_text(&node.name, 10.0, Mm(node.x), Mm(node.y), &font);
        current_layer.use_text(
            &text,
            10.0,
            Mm(node.x + x_size / 2.0),
            Mm(node.y + y_size / 2.0),
            &font,
        );
    }

    // ファイル出力
    let output = File::create(config.output.as_ref().unwrap()).unwrap();
    println!("Output file: {}", config.output.as_ref().unwrap());
    let mut writer = BufWriter::new(output);
    doc.save(&mut writer).unwrap();
}
