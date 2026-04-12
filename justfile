# AnkiTUI - Task Runner
# Usage: just <command>

# ========== 开发 ==========

dev:
    cargo run

check:
    cargo check --workspace --lib --bins

fmt:
    cargo fmt --all

lint:
    cargo clippy --workspace --lib --bins -- -D warnings

test:
    cargo test --workspace --lib

test-watch:
    cargo test --workspace --lib -- --nocapture

# ========== 构建 ==========

build:
    cargo build

release:
    cargo build --release

run:
    ./target/release/ankitui

# ========== 清理 ==========

clean:
    cargo clean

deep-clean:
    cargo clean
    rm -rf .cargo/.rustc_info.json

# ========== 安装 ==========

install:
    bash scripts/install.sh

uninstall:
    bash scripts/uninstall.sh

# ========== CI ==========

ci: fmt check lint test

# ========== 文档 ==========

doc:
    cargo doc --no-deps --workspace

doc-open:
    cargo doc --no-deps --workspace --open

# ========== 发布 ==========

tag version:
    git tag v{{version}}
    git push origin v{{version}}

bump version:
    # Update version in all Cargo.toml files
    sed -i '' 's/version = ".*"/version = "{{version}}"/' Cargo.toml
    sed -i '' 's/version = ".*"/version = "{{version}}"/' ankitui-core/Cargo.toml
    sed -i '' 's/version = ".*"/version = "{{version}}"/' ankitui-tui/Cargo.toml
    sed -i '' 's/version = ".*"/version = "{{version}}"/' ankitui/Cargo.toml
    git add Cargo.toml ankitui-core/Cargo.toml ankitui-tui/Cargo.toml ankitui/Cargo.toml
    git commit -m "bump version to {{version}}"
