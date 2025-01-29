# mmd_to_pdf

## 概要

以下のようなmermaidの入力はパースできる

```plaintext
graph TD
    a["hoge"]
    a --> huge
    b["aaa"] --> bar
```

ほか、graph LRも使用可能。

### 不可なパターン

基本、簡単なフローチャート以外の出力は不可
