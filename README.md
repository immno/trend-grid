# trend-grid(TGS)
[![License](https://img.shields.io/badge/license-Apache%202-4EB1BA.svg)](https://www.apache.org/licenses/LICENSE-2.0.html)
[![GitHub release](https://img.shields.io/github/v/release/immno/trend-grid.svg)](https://github.com/immno/trend-grid/releases)

使用rust实现的一种趋势网格(主要用来学习)。**趋势判断，不在固定点位开单，根据k线选择更优的开仓点位。**  
其中参考了一些项目的优秀设计:
- [Rust Binance API](https://github.com/PrivateRookie/bian-rs)
- [币安开发文档](https://binance-docs.github.io/apidocs/spot/cn/#45fa4e00db)
- [币安Spot Test](https://testnet.binance.vision/)
- [Python 网格实现](https://github.com/hengxuZ/spot-trend-grid.git)

## License

[Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

## 快速启动
### 使用Docker
```shell
git clone https://github.com/immno/trend-grid.git
cd trend-grid
docker build -t immno/tg:v0.1.0 .
docker run -d --name tgs -v ./:/etc/tgs/ -e TGS_CONFIG=/etc/tgs/tgs.conf immno/tg:v0.1.0
```
### 使用 Git
```shell
git clone https://github.com/immno/trend-grid.git
cd trend-grid
cargo run --color=always --bin tgs
```
- 需要修改`./fixtures/tag.conf`文件

## 参数配置
```toml
[trade]
key = 'xx'
secret = 'xx'
# 正式API
# url = 'https://api.binance.com/api/v3/'
# 测试网的API地址
url = 'https://testnet.binance.vision/api/v3/'

[coin]
# eth/btc/bnb
[coin.eth]
# 初始买入标准，系统会一直等待或者满足卖出标准后自动调整
buy_price = 3800
# 强制卖出标准,系统也会调整
sell_price = 4000
profit_ratio = '2.3%'
double_throw_ratio = '2.3%'
# 每次交易的数量
quantity = 0.003

[log]
enable_log_file = false
# debug/info/warn/error
log_level = 'info'
path = '/tmp/tgs-log'
# Hourly/Daily/Never
rotation = 'Daily'
```

## 代码提交
### 提交代码的正确性
在根目录生成`.pre-commit-config.yaml`，运行`pre-commit install`(需要安装`pip install pre-commit`)，以后`git commit`时就会自动做这一系列的检查，保证提交代码的最基本的正确性。
### 检测授权（暂未生效）
根目录下最好还声明一个`deny.toml`，使用`cargo-deny`(需要安装`cargo install --locked cargo-deny`)来确保你使用的第三方依赖没有不该出现的授权（比如不使用任何`GPL/APGL`的代码）、没有可疑的来源（比如不是来自某个 fork 的 GitHub repo 下的 commit），以及没有包含有安全漏洞的版本。  
`cargo-deny`对于生产环境下的代码非常重要，因为现代软件依赖太多，依赖树过于庞杂，靠人眼是很难审查出问题的。通过使用`cargo-deny`，可以避免很多有风险的第三方库。
