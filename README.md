# trend-grid
使用rust实现的一种趋势网格(主要用来学习)

## 代码仓库管理
### 提交代码的正确性
在根目录生成`.pre-commit-config.yaml`，运行`pre-commit install`(需要安装`pip install pre-commit`)，以后`git commit`时就会自动做这一系列的检查，保证提交代码的最基本的正确性。
### 检测授权
根目录下最好还声明一个`deny.toml`，使用`cargo-deny`(需要安装`cargo install --locked cargo-deny`)来确保你使用的第三方依赖没有不该出现的授权（比如不使用任何`GPL/APGL`的代码）、没有可疑的来源（比如不是来自某个 fork 的 GitHub repo 下的 commit），以及没有包含有安全漏洞的版本。  
`cargo-deny`对于生产环境下的代码非常重要，因为现代软件依赖太多，依赖树过于庞杂，靠人眼是很难审查出问题的。通过使用`cargo-deny`，可以避免很多有风险的第三方库。  

## 参考
[Rust Binance API](https://github.com/PrivateRookie/bian-rs)  
[币安开发文档](https://binance-docs.github.io/apidocs/spot/cn/#45fa4e00db)  
[币安Spot Test](https://testnet.binance.vision/)  
