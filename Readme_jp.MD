### Language：[中文](Readme.md "Readme.md") [日本語](Readme_jp.md "Readme_jp.md")
---
### 全結合ネットワークで手書き数字を識別する（MNIST）

#### 紹介

- ブログ[全結合ネットワークで手書き数字を識別する（MNIST）](https://blog.csdn.net/xuanwolanxue/article/details/71565934) のインスピレーションを受けて、
0~9手書き数字を識別できるのプログラムを作りました。目的はRustの基本文法と全結合ネットワーク実現方法を勉強するため。

- 参照元ブログは[CC 4.0 BY-SA](https://creativecommons.org/licenses/by-sa/4.0/) 著作権契約に従っています。
元のソースはC++で作って、[doc/guide_src](/doc/guide_src)に格納しました、

- テストの結果は[test_result.log](/test_result.log)にあります、各パラメータを調整して、精度は89%～92％ぐらいなります。

#### 実行方法

- ステップ１、[resources/gz](/resources/gz)配下圧縮したファイルを[resources](/resources)に解凍する。

- ステップ２、[test_result.log](/test_result.log)の結果を参照して、適切なパラメータを設定する（省略可）

- ステップ３、Cargoで実行する（注意：releaseモードで実行する、Debugモードは結構時間がかかります）
    ```
    cargo run --release >> test_result.log
    ```
