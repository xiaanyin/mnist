### Language：[中文](Readme.md "Readme.md") [日本語](Readme_jp.md "Readme_jp.md")
---
### 全连接神经网络实现手写数字识别（MNIST）

#### 简介

- 受博客[神经网络实现手写数字识别（MNIST）](https://blog.csdn.net/xuanwolanxue/article/details/71565934) 的启发，
写了一个0到9的数字识别程序，目的是为了学习Rust和全连接神经网络的算法实现。<br>

- 原博客内容遵循[CC 4.0 BY-SA](https://creativecommons.org/licenses/by-sa/4.0/)版权协议。<br>
原博客的代码是用c++写的，放在[doc/guide_src](/doc/guide_src)下，供参考。

- 测试的结果在[test_result.log](/test_result.log)，各个参数经过调整以后，准确率大概在89%到92%之间。

#### 执行

- 首先，将[resources/gz](/resources/gz)下的所有内容解压缩到[resources](/resources)下。

- 然后根据[test_result.log](/test_result.log)适当调整自己需要的参数（可以略过此步）。

- 最后执行cargo命令（注意：要用release模式执行，否则会很慢）。
    ```
    cargo run --release >> test_result.log
    ```
