use crate::mmdparser::Element;
use std::collections::HashSet;

// graph_typeがTDの場合は横書き、LRの場合は縦書き(LRの場合はWIDTHとHEIGHTを入れ替える)
const MAX_WIDTH: f64 = 100.0;
const MAX_HEIGHT: f64 = 20.0;
const LINE_MAX_CHAR_NUM: usize = 10;
const MARGIN: f64 = 10.0;

pub fn corrdinate_determine<'a>(nodes: &mut Vec<Element>, graph_type: &str) {
    /*
    入力: ノード名、親ノード名、子ノード名
    出力はそのノードの座標
    */
    let mut priority = priority_determine(nodes);
    // println!("{:#?}", priority);
    for (i, group) in priority.iter_mut().enumerate() {
        for node in group.iter_mut() {
            let width = if node.text.len() > LINE_MAX_CHAR_NUM {
                MAX_WIDTH
            } else {
                node.text.len() as f64 * 10.0
            };
            let height = MAX_HEIGHT;
            node.width = width;
            node.height = height;
            if i == 0 {
                node.x = MARGIN;
                node.y = MARGIN;
            } else {
                let mut max_y = 0.0;
                for (to, _) in node.to.as_ref().unwrap() {
                    let to_node = nodes.iter().find(|n| n.name == *to).unwrap();
                    if to_node.y > max_y {
                        max_y = to_node.y;
                    }
                }
                node.y = max_y + MAX_HEIGHT + MARGIN;
                node.x = MARGIN;
            }
        }
    }
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
            // 最後の要素に終点(子ノードがないノード)を追加
            priority[i].push(nodes.iter().find(|n| n.to.is_none()).unwrap());
            break;
        }
    }
    remove_duplicates(&mut priority);
    priority.iter().filter(|p| !p.is_empty()).map(|p| p.to_vec()).collect()
}

fn remove_duplicates(priority: &mut Vec<Vec<&Element>>) {
    // priority内に同じ要素がある場合は、インデックスの小さいものを全て削除
    // 普通にHashSetを使うと、小さいインデックスのものが残るので、逆順からかける
    let mut seen = HashSet::new();
    priority.iter_mut().rev().for_each(|group| {
        group.retain(|element| seen.insert(element.name.clone()));
    });
}
