FROM rust:1.67

WORKDIR /usr/src/rustep
COPY . .
RUN cargo test --release -- --color always --nocapture
RUN cargo install --path . --profile release

CMD ["rustep"]
