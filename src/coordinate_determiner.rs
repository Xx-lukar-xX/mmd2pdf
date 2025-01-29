// クーロン力を参考にして、ノードの座標を決定する

pub fn corrdinate_determin(elements: Vec<(String, usize)>, graph_type: String) -> Vec<(f64, f64)> {
    /*
    入力はノード名とそのノードに接続しているエッジの数
    出力はそのノードの座標
     */
    (0..elements.len())
        .map(|i| {
            let x = 0.0;
            let y = 0.0;
            (x, y)
        })
        .collect()
}
