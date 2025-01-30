// markdown記法のmermaidをパースするためのモジュール
// これから本気でやるなら、nomを使う

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Element {
    pub name: String,
    pub text: String,
    pub shape: String,
    pub to: Option<Vec<(String, Option<String>)>>, // 一つ目がノード接続先、二つ目がが中間にはさむテキスト(条件分岐とかに使う)
    pub from: Option<Vec<String>>, // toだけじゃきつかったので、fromも追加
}

pub fn parse_file(filename: &str) -> Result<(String, Vec<Element>), String> {
    /*
    ファイルをパースする
    出力はグラフの種類(TDまたはLR)と、ノード情報のリスト
     */
    let space_toka_ignored = apply_ignore(open(filename));
    let mut elements: Vec<Element> = Vec::new();
    let mut graph_type: Option<&str> = None;

    for line in &space_toka_ignored {
        if let Some(pos) = line.find("graph") {
            graph_type = Some(&line[pos + 5..pos + 7]);
        } else if let Some(pos) = line.find("--") {
            if line[pos + 2..].contains("--") {
                let re = Regex::new(r"(\w+)--(\w+)-->(\w+)").unwrap();
                if let Some(caps) = re.captures(&line) {
                    let from = caps.get(1).unwrap().as_str();
                    let middle = caps.get(2).unwrap().as_str();
                    let to = caps.get(3).unwrap().as_str();
                    gen_vars(from, &mut elements);
                    gen_vars(to, &mut elements);
                    pusher(from, to, Some(middle), &mut elements);
                }
            } else if line.contains("-->") {
                let re = Regex::new(r"(\w+)-->(\w+)").unwrap();
                if let Some(caps) = re.captures(&line) {
                    let from = caps.get(1).unwrap().as_str();
                    let to = caps.get(2).unwrap().as_str();
                    gen_vars(from, &mut elements);
                    gen_vars(to, &mut elements);
                    pusher(from, to, None, &mut elements);
                } else {
                    // pass
                }
            }
        } else {
            gen_vars(&line, &mut elements);
        }
    }
    Ok((graph_type.unwrap().to_string(), elements))
}

fn gen_vars(input: &str, elements: &mut Vec<Element>) {
    /*
    入力はノード名、と現在のelementたちのリスト
    入力をパースし、elementを生成する
     */

    let name;
    let mut text;
    let mut shape = "rect".to_string();
    if let Some(pos) = input.find("[") {
        name = input[..pos].to_string();
        text = input[pos + 1..input.len() - 1].to_string();
    } else if let Some(pos) = input.find("{{") {
        name = input[..pos].to_string();
        text = input[pos + 2..input.len() - 2].to_string();
        shape = "diamond".to_string();
    } else if let Some(pos) = input.find("{") {
        name = input[..pos].to_string();
        text = input[pos + 1..input.len() - 1].to_string();
        shape = "diamond".to_string();
    } else if let Some(pos) = input.find("(") {
        name = input[..pos].to_string();
        text = input[pos + 1..input.len() - 1].to_string();
        shape = "circle".to_string();
    } else {
        name = input.to_string();
        text = input.to_string();
    }
    if (text.starts_with('"') && text.ends_with('"')) || (text.starts_with('\'') && text.ends_with('\'')) {
        text.remove(0);
        text.pop();
    }
    if elements.iter_mut().find(|e| e.name == name).is_some() {
        // pass
    } else {
        elements.push(Element {
            name,
            text,
            shape,
            to: None,
            from: None,
        });
    }
}

fn pusher(from: &str, to: &str, text: Option<&str>, elements: &mut Vec<Element>) {
    to_pusher(from, to, text, elements);
    from_pusher(from, to, elements);
}

fn to_pusher(from: &str, to: &str, text: Option<&str>, elements: &mut Vec<Element>) {
    /*
    入力はfromノード名、toノード名、現在のelementたちのリスト
    fromノードにtoノードを接続する(fromのtoにtoを追加する。もし、textがあればそれも追加する)
     */
    let from = elements.iter_mut().find(|e| e.name == from).unwrap();
    if from.to.is_none() {
        from.to = Some(Vec::new());
    }
    from.to
        .as_mut()
        .unwrap()
        .push((to.to_string(), text.map(|t| t.to_string())));
}

fn from_pusher(from: &str, to: &str, elements: &mut Vec<Element>) {
    /*
    ほぼto_pusherと同じ
    fromにtoを追加する
     */
    let to = elements.iter_mut().find(|e| e.name == to).unwrap();
    if to.from.is_none() {
        to.from = Some(Vec::new());
    }
    to.from.as_mut().unwrap().push(from.to_string());
}

fn apply_ignore(lines: Vec<String>) -> Vec<String> {
    /*
    ignore_whitespace_outside_quotesを適用し、空白行を削除する
     */
    lines
        .into_iter()
        .map(|line| ignore_whitespace_outside_quotes(&line))
        .filter(|line| !line.trim().is_empty())
        .collect()
}

fn ignore_whitespace_outside_quotes(input: &str) -> String {
    /*
    入力された文字列の中の、クォーテーションの外側のスペースを無視する(必要なスペースもあるから検討は必要)
     */
    let mut result = String::new();
    let mut in_quotes = false;

    for c in input.chars() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
                result.push(c);
            }
            ' ' | '\t' if !in_quotes => {
                // Ignore whitespace outside quotes
            }
            _ => {
                result.push(c);
            }
        }
    }
    result
}

fn open(filename: &str) -> Vec<String> {
    /*
    ファイルを開いて、中身を返す(行またはセパレータ(;)で分割)
     */
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        for part in line.split(';') {
            lines.push(part.to_string());
        }
    }
    lines
}

#[allow(dead_code)]
fn serialize(elements: Vec<Element>) -> String {
    /*
    入力はelementのリスト
    出力は隣接リストの形にした文字列
    ノード間のテキストがあれば!--テキスト--!として出力する(もっといい方法あればいいな)
    */
    let mut result = String::new();
    for element in elements {
        result.push_str(&format!("{}[", element.name));
        if let Some(to) = element.to {
            for (to_name, text) in to {
                if let Some(text) = text {
                    result.push_str(&format!("!--{}--!{},", text, to_name));
                } else {
                    result.push_str(&format!("{},", to_name));
                }
            }
        } else {
            result.push_str(",");
        }
        result.pop();
        result.push_str("]");
    }
    println!("{}", result);
    result
}
