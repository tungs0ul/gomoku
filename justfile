fmt:
    cargo fmt

sqlx:
    cd backend && cargo sqlx prepare

clippy:
    cargo clippy --workspace --all-targets --all-features --fix

test:
    cargo test

init_db:
    ./scripts/init_db.sh

clear_test_db:
    ./scripts/clear_test_dbs.sh