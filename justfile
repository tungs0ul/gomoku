fmt:
    cargo fmt
    cargo sqlx prepare

clippy:
    cargo clippy --workspace --all-targets --all-features --fix