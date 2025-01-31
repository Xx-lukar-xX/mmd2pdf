// コマンドライン引数をパースする

use clap::{App, Arg};
use regex::Regex;

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub output: Option<String>,
    pub font: Option<String>,
    pub node_colour: String,
    pub linebreak: Option<usize>,
}

pub fn get_args() -> Config {
    let matches = App::new("mmd2pdf")
        .version("0.1.0")
        .author("Xx-lukar-xX")
        .about(
            "Generates a PDF file from a mmd file(only for graph TD or LR) LRは日本語の縦書き対応！",
        )
        .arg(
            Arg::with_name("filename")
                .index(1)
                .value_name("FILE")
                .help("Sets the mmd file to use")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .index(2)
                .value_name("OUTPUT")
                .help("Sets the output PDF file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("font")
                .short("f")
                .long("font")
                .value_name("FONT")
                .help("Sets the font file to use(ttf supported)")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("node_colour")
                .short("c")
                .long("colour")
                .value_name("COLOUR")
                .help("Sets the colour of the node")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("linebreak")
                .short("b")
                .long("linebreak")
                .value_name("LINEBREAK")
                .help("Sets the number of characters per line")
                .takes_value(true)
        )
        .get_matches();

    Config {
        filename: matches.value_of("filename").unwrap().to_string(),
        // outputが指定されていない場合は、filenameの拡張子をpdfに変更して出力する
        output: match matches.value_of("output") {
            Some(output) if !output.is_empty() => Some(output.to_string()),
            _ => {
                let mut output = matches.value_of("filename").unwrap().to_string();
                if let Some(pos) = output.rfind('.') {
                    output.replace_range(pos.., ".pdf");
                } else {
                    output.push_str(".pdf");
                }
                Some(output)
            }
        },
        font: matches.value_of("font").map(|s| s.to_string()),
        node_colour: node_color(matches.value_of("node_colour").map(|s| s.to_string())),
        linebreak: matches.value_of("linebreak").map(|s| s.parse().unwrap()),
    }
}

fn node_color(non_hex_colour: Option<String>) -> String {
    /*
    アルファベットの入力をカラーコードに変換する
    できなかったら、#000000(白)を返す
     */
    if let Some(colour) = non_hex_colour.as_deref() {
        let colour = colour.to_lowercase();
        if let Some(hex) = match colour.as_str() {
            "black" => Some("#000000"),
            "white" => Some("#ffffff"),
            "red" => Some("#ff0000"),
            "green" => Some("#00ff00"),
            "blue" => Some("#0000ff"),
            "yellow" => Some("#ffff00"),
            "cyan" => Some("#00ffff"),
            "magenta" => Some("#ff00ff"),
            "purple" => Some("#800080"),
            "orange" => Some("#ffa500"),
            "pink" => Some("#ffc0cb"),
            "brown" => Some("#a52a2a"),
            hex if Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap().is_match(hex) => Some(hex),
            _ => None,
        } {
            hex.to_string();
        }
    }
    "#FFFFFF".to_string()
}
