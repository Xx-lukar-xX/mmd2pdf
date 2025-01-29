use clap::{App, Arg};

pub struct Config {
    pub filename: String,
    pub output: String,
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
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    Config {
        filename: matches.value_of("filename").unwrap().to_string(),
        output: matches.value_of("output").unwrap().to_string(),
    }
}
