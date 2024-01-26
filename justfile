# List the just recipe list
list:
    just --list

format:
    cargo fmt

build platform="desktop":
    just build-{{platform}}

build-web:
    wasm-pack build --target web --release

build-desktop:
    cargo build --release

run platform="desktop":
    just run-{{platform}}

run-desktop:
    cargo run --release

run-web:
    just build web
    http

clippy:
    cargo clippy

test:
    cargo test

checks:
    just format
    just build
    just clippy
    just test

clean:
    find . -name target -type d -exec rm -r {} +
    just remove-lockfiles

remove-lockfiles:
    find . -name Cargo.lock -type f -exec rm {} +

list-outdated:
    cargo outdated -R -w

update:
    cargo update --aggressive

package:
    just package-windows
    just package-web

package-windows:
    cargo build --release
    rm -rf ./dist/windows/
    mkdir -p ./dist/windows/
    cp -rf ./assets ./dist/windows/
    cp -f ./target/release/krakjam2024.exe ./dist/windows/
    cp -f ./README.md ./dist/windows/
    powershell Compress-Archive -Force "./dist/windows/*" ./dist/krakjam2024-windows.zip

package-web:
    just build web
    rm -rf ./dist/web/
    mkdir -p ./dist/web/pkg/
    cp -f ./index.html ./dist/web/
    cp -f ./pkg/game_web.js ./dist/web/pkg/
    cp -f ./pkg/game_web_bg.wasm ./dist/web/pkg/
    cp -f ./README.md ./dist/web/
    powershell Compress-Archive -Force "./dist/web/*" ./dist/krakjam2024-web.zip
