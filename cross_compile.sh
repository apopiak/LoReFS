set -ex

docker run --userns host --rm --user 1000:1000 \
    -e CARGO_HOME=/cargo -e CARGO_TARGET_DIR=/target -e USER=$USER -e XARGO_HOME=/xargo \
    -v $HOME/.xargo:/xargo \
    -v $HOME/.cargo:/cargo \
    -v $(pwd):/project \
    -v $HOME/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu:/rust:ro \
    -v $(pwd)/target:/target \
    -w /project \
    -it japaric/x86_64-sun-solaris:latest \
    sh -c "PATH=$PATH:/rust/bin xargo build --release --target=x86_64-sun-solaris -v"
