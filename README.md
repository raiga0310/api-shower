# ShowerTime API

[![Rust](https://github.com/raiga0310/api-shower/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/raiga0310/api-shower/actions/workflows/rust.yml) [![codecov](https://codecov.io/github/raiga0310/api-shower/branch/main/graph/badge.svg?token=3YERKCDNF1)](https://codecov.io/github/raiga0310/api-shower) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Environment / 環境

‐ Rust/Cargo 1.68.2
- Docker 20.10.17

## Installation / インストール

1. Clone this repository / リポジトリをクローンする

```bash
git clone https://github.com/raiga0310/api-shower.git
```

2. Move to the directory / ディレクトリに移動する
```bash
cd api-shower
```

3. Build the docker image / Dockerイメージをビルドする
```bash
docker-compose build
```

4. Run the docker container / Dockerコンテナを起動する
```bash
docker-compose up -d
```

5. Setup the database / データベースをセットアップする
```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx db create
sqlx migrate run
```

6. Set Environment Variables / 環境変数を設定する
```bash
echo "DATABASE_URL=postgres://user:password@localhost:5432/shower" > .env
```

7. Run the server / サーバーを起動する
```bash
cargo run
```

## Usage / 使い方

クライアント
[front-shower](https://github.com/raiga0310/front-shower)
