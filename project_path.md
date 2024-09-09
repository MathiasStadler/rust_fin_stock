# create date

- date

```bash
date
Di 20. Aug 07:44:16 UTC 2024
```

- rustc --version

```bash
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```

``` bash
rustc --version
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
cargo --version
cargo 1.80.1 (376290515 2024-07-16)
```

## create rust project folder

```bash
cd 
cd workspace_rust
RUST_PROJECT_NAME="rust_stock_multipanel"
mkdir $RUST_PROJECT_NAME && cd $_
```

## init project

```bash
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup  show \
&& rustup  check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& rustup show \
&& cargo add rustfmt \
&& cargo add cargo-edit \
&& rustup component add rustfmt \
&& rustup show \
&& touch FROM_HERE.md \
&& cargo --version \
&& rustc --version
```

## build plain project

```bash
cargo build
```

## install vscode extension manually

- prettier
- rust-analyser
- spell-checker  /code-spell-checker

## add crate for project

```bash
cargo add chrono
cargo add decimal
cargo add num_traits
cargo add plotters
cargo add rand
cargo add rust_decimal
cargo add rust_decimal_macros
cargo add ta
cargo add csv
cargo add rust_decimal
cargo add serde
# FROM HERE
# https://users.rust-lang.org/t/how-to-update-dependencies-to-the-latest/110232/2
# cargo upgrade -i allow --verbose
cargo update --verbose
cargo build
```

## add cargo watch

```bash
cargo install cargo-watch
```

## add plotters necessary dependencies

```bash
sudo apt update
sudo apt upgrade --Yes 
sudo apt install --Yes pkg-config libfreetype6-dev libfontconfig1-dev
sudo apt-get install libssl-dev
```

## add stock data in folder

```bash
# change to project folder
mkdir stock_data
# inside a browser
# https://stooq.com/q/d/l/?s=TREX.US&i=d&d1=19900101&d2=20241231
# curl --output stock_trex_data.csv https://stooq.com/q/d/l/?s=TREX.US&i=d&d1=19900101&d2=20241231
curl --output stock_data/stock_trex_data.csv https://stooq.com/q/d/l/?s=TREX.US&i=d&d1=19900101&d2=20241231

```

## start /w cargo watch

```bash
cargo watch

```

## [removed unused dependencies](https://stackoverflow.com/questions/72082550/how-do-i-remove-unused-dependencies-in-cargo-toml)

- [crates cargo-shear](https://crates.io/search?q=cargo-shear)

```bash
# install
cargo install cargo-shear
# Check if there are any unused dependencies
cargo shear
# To fix (remove) an unused dependency:
cargo shear --fix

```

## Updating a local repository with changes from a GitHub repository

```bash
git pull origin master
```
