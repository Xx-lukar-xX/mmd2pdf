use crate::mmdparser::Element;
use std::collections::HashSet;

// graph_typeがTDの場合は横書き、LRの場合は縦書き(LRの場合はWIDTHとHEIGHTを入れ替える)
const MAX_WIDTH: f32 = 100.0;
const MAX_HEIGHT: f32 = 20.0;
#[allow(dead_code)]
const LINE_MAX_CHAR_NUM: usize = 10;
const MARGIN: f32 = 10.0;

pub fn corrdinate_determine(nodes: Vec<Element>, graph_type: &str) -> Vec<Element> {
    /*
    入力: ノード名、親ノード名、子ノード名
    出力はそのノードの座標
    */
    let priority = priority_determine(&nodes);
    let mut result_nodes: Vec<Element> = Vec::new();
    for (i, group) in priority.iter().enumerate() {
        let (mut x, y) = (
            0.0 - (group.len() - 1) as f32 * (MAX_WIDTH + MARGIN),
            0.0 - i as f32 * (MAX_HEIGHT + MARGIN),
        );
        let (height, width) = (MAX_HEIGHT, MAX_WIDTH / group.len() as f32);
        for node in group.iter() {
            if graph_type == "TD" {
                result_nodes.push(Element {
                    name: node.name.clone(),
                    text: node.text.clone(),
                    shape: node.shape.clone(),
                    to: node.to.clone(),
                    from: node.from.clone(),
                    x,
                    y,
                    width,
                    height,
                });
            } else if graph_type == "LR" {
                result_nodes.push(Element {
                    name: node.name.clone(),
                    text: node.text.clone(),
                    shape: node.shape.clone(),
                    to: node.to.clone(),
                    from: node.from.clone(),
                    x: y,
                    y: x,
                    width: height,
                    height: width,
                });
            }
            x += MAX_WIDTH + MARGIN;
        }
    }
    result_nodes
}

fn priority_determine(nodes: &Vec<Element>) -> Vec<Vec<Element>> {
    // 最も上位の親ノードをから優先順位を決定
    // 返り値の構造: インデックスが優先順位、中身がその優先順位のノード名
    let mut priority: Vec<Vec<Element>> = Vec::new();
    for node in nodes {
        if node.from.is_none() {
            priority.push(Vec::new());
            priority[0].push(node.clone());
        }
    }
    let mut i = 0;
    loop {
        priority.push(Vec::new());
        let (current, next) = priority.split_at_mut(i + 1); // split_at_mut関数はi番目の要素を含む前後に分割する
        for node in &mut current[i] {
            if let Some(to_node) = &node.to {
                for (to, _) in to_node {
                    let to_node = nodes.iter().find(|n| n.name == *to).unwrap();
                    next[0].push(to_node.clone());
                }
            }
        }
        i += 1;
        if next[0].is_empty() {
            // 最後の要素に終点(子ノードがないノード)を追加
            priority[i].push(nodes.iter().find(|n| n.to.is_none()).unwrap().clone());
            break;
        }
    }
    remove_duplicates(&mut priority);
    priority
        .iter()
        .filter(|p| !p.is_empty())
        .map(|p| p.to_vec())
        .collect()
}

fn remove_duplicates(priority: &mut Vec<Vec<Element>>) {
    // priority内に同じ要素がある場合は、インデックスの小さいものを全て削除
    // 普通にHashSetを使うと、小さいインデックスのものが残るので、逆順からかける
    let mut seen = HashSet::new();
    priority.iter_mut().rev().for_each(|group| {
        group.retain(|element| seen.insert(element.name.clone()));
    });
}
