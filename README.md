# mmd_to_pdf

## 概要

Powerpointでスライドを作成する際、GUIでのフローチャートが面倒だったので作成を決意。現時点では_急ぎ_で作成しているため、簡単なフローチャートのみの対応。

以下のようなmermaidの入力はパースできる想定

```plaintext
graph TD
    a["hoge"]
    a --> hoge
    b["hoge"] --> bar
    a -- b --> bar
```

結果イメージ
```mermaid
graph TD
    a["hoge"]
    a --> hoge
    b["hoge"] --> bar
    a -- b --> bar
```

ほか、graph LRも使用可能。

### 不可なパターン

基本、簡単なフローチャート以外の出力は不可
例: `hoge --> huge --> foo`(3つ並行で連結を記述)`

## 特徴

graph LRと宣言すると、mermaidでは横書きになるが、スライドのフローチャートには日本語を載せたいため、文字が縦書きになるように

## 使用方法

Once you have cloned this and got into the directory, try the following:

[1]

```sh
cargo build --release
```

[2]

```sh
./target/release/mmd2pdf INPUT_FILE OUTPUT_FILE
```
