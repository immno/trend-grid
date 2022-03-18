FROM rust:latest AS chef

# 更换源，提升构建速度
RUN touch $CARGO_HOME/config && \
    echo "[source.crates-io]" >> $CARGO_HOME/config && \
    echo "replace-with = 'ustc'" >> $CARGO_HOME/config && \
    echo "[source.ustc]" >> $CARGO_HOME/config && \
    echo "registry = 'git://mirrors.ustc.edu.cn/crates.io-index'" >> $CARGO_HOME/config

# cargo-chef 是一个新的cargo 子命令，用于在基于JSON 描述文件（即recipe）上构建Rust 项目的依赖项。
# 在一个基准测试中， cargo-chef 将Docker 的build 时间从10分钟缩减到2分钟。
RUN cargo install cargo-chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# 构建依赖；会加到缓存 Docker 层！
RUN cargo chef cook --release --recipe-path recipe.json

# 构建应用
COPY . .
RUN cargo install --path .

# 不需要 Rust 工具链来运行二进制文件！
FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/tgs /usr/local/bin

EXPOSE 8866
CMD [ "/usr/local/bin/tgs" ]
