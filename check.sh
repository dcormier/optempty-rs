set -x
cargo clippy && \
cargo clippy --all-features && \
cargo clippy --no-default-features && \
cargo clippy --no-default-features --features serdejson && \
cargo clippy --no-default-features --features querymap && \
cargo clippy --tests && \
cargo clippy --tests --all-features && \
cargo clippy --tests --no-default-features && \
cargo clippy --tests --no-default-features --features serdejson && \
cargo clippy --tests --no-default-features --features querymap && \
cargo test && \
cargo test --all-features && \
cargo test --no-default-features && \
cargo doc --all-features

cargo publish --dry-run -v --allow-dirty
set +x
