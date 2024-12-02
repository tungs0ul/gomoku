fmt:
    cargo fmt

sqlx:
    cargo sqlx prepare --workspace -- -p backend

clippy:
    cargo clippy --workspace --all-targets --all-features --fix