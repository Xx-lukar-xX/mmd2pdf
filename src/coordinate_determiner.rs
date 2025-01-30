use crate::mmdparser::Element;
use std::collections::HashMap;
use taffy::prelude::*;


pub fn corrdinate_determine(nodes: &Vec<Element>) -> Vec<(String, (f64, f64))> {
    /*
    入力: ノード名、親ノード名、子ノード名
    出力はそのノードの座標
    */
    let priority = priority_determine(nodes);
    println!("{:#?}", priority);
    let a: Vec<(String, (f64, f64))> = Vec::new();
    a
}

fn priority_determine(nodes: &Vec<Element>) -> Vec<Vec<&Element>> {
    // 最も上位の親ノードをから優先順位を決定
    // 返り値の構造: インデックスが優先順位、中身がその優先順位のノード名
    let mut priority:Vec<Vec<&Element>> = Vec::new();
    for node in nodes {
        if node.from.is_none() {
            priority.push(Vec::new());
            priority[0].push(node);
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
                    next[0].push(to_node);
                }
            }
        }
        i += 1;
        if next[0].is_empty() {
            priority.pop();
            break;
        }
    }
    priority
}
