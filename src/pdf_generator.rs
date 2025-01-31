// 文字、座標などからpdfファイルを生成する
// 座標: 左下端が(0.0, 0.0)

use crate::mmdparser::Element;
use crate::parse_args::Config;
use printpdf::path::{PaintMode, WindingOrder};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

const MAX_WIDTH: f32 = 100.0;
const MAX_HEIGHT: f32 = 20.0;
const MARGIN: f32 = 10.0;
const FONT_SIZE: f32 = 10.0;

pub fn makepdf(nodes: &Vec<Element>, config: &Config, graph_type: &str) {
    let x_size = nodes
        .iter()
        .map(|node| node.x.abs())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
        * 2.0
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
        * 2.0
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

    // ノードの描画
    for node in nodes {
        let text = node.text.clone();
        let mut texts: Vec<String> = Vec::new();
        if let Some(linebreak) = config.linebreak {
            let line_num = text.chars().count() / linebreak;
            for i in 0..=line_num {
                texts.push(text.chars().skip(i * linebreak).take(linebreak).collect());
            }
        } else {
            texts.push(text);
        }
        for (i, text) in texts.iter().enumerate() {
            current_layer.use_text(
                text,
                FONT_SIZE,
                Mm(node.x + x_size / 2.0),
                Mm(node.y + y_size / 2.0 - (FONT_SIZE / 2.0) * i as f32),
                &font,
            );
        }
        let figure_size = if graph_type == "TD" {
            vec![
                Mm(node.x + x_size / 2.0 - 2.0),
                Mm(node.y + y_size / 2.0 - (texts.len() - 1) as f32 * FONT_SIZE / 2.0 - 2.0),
                Mm(node.x + x_size / 2.0 + (texts[0].len() - 1) as f32 * FONT_SIZE / 2.0 + 2.0),
                Mm(node.y + y_size / 2.0 + texts.len() as f32 * FONT_SIZE / 2.0 - 2.0),
            ]
        } else {
            vec![
                Mm(node.x + x_size / 2.0 - (texts[0].len() - 1) as f32 * FONT_SIZE / 2.0 - 2.0),
                Mm(node.y + y_size / 2.0 - 2.0),
                Mm(node.x + x_size / 2.0 + (texts[0].len() - 1) as f32 * FONT_SIZE / 2.0 + 2.0),
                Mm(node.y + y_size / 2.0 + texts.len() as f32 * FONT_SIZE / 2.0 - 2.0),
            ]
        };

        if node.shape == "rect" {
            current_layer.add_rect(
                Rect::new(
                    figure_size[0],
                    figure_size[1],
                    figure_size[2],
                    figure_size[3],
                )
                .with_mode(PaintMode::Stroke)
                .with_winding(WindingOrder::EvenOdd),
            );
        }
    }

    // ファイル出力
    let output = File::create(config.output.as_ref().unwrap()).unwrap();
    let mut writer = BufWriter::new(output);
    doc.save(&mut writer).unwrap();
}

fn to_tategaki(text: &str) -> Vec<String> {
    let mut result = Vec::new();
    for c in text.chars() {
        result.push(c.to_string());
    }
    result
}
