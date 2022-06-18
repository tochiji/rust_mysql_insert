# rust_mysql_insert
CSVデータをMySQLのデータベースに入れるサンプルコードです。

## 挿入するデータ
「東京都 新型コロナウイルス陽性患者発表詳細」サイトより、下記のCSVをダウンロードし、MySQLのデータベースへ入れています。

https://stopcovid19.metro.tokyo.lg.jp/data/130001_tokyo_covid19_patients_2021.csv

## 事前準備

### 環境変数の指定
`.env` ファイルを作成し、有効なMySQLのURLを記載してください。コードの`main`関数内でこの値を取得し、使用します。

```sh
echo 'DB_URL="mysql://<user>:<password>@<url>"' >.env
```

### cargoのインストール
コードの実行には `cargo` コマンドが必要です。下記URLよりインストールしてください。

https://doc.rust-lang.org/cargo/getting-started/installation.html

`cargo version`を実行してバージョンが表示されていればOKです。

```sh
$ cargo version
cargo 1.59.0 (49d8809dc 2022-02-10)
```

## コードの実行
ローカルで下記コマンドを実行すると、CSVデータの先頭10行（ヘッダーはカウントしない）を、DB_URLで指定したデータベースへ挿入します。

```sh
cargo run
```

複数回実行すると、同一キーでの挿入エラーが出ます。

## ディレクトリ構成

```console
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── corona_csv_row.rs  # CSVのDeserializeを行うための型を定義しています
    ├── db.rs   # MySQLとのコネクションを作成する関数+INSERTクエリです
    ├── lib.rs  # 実処理を行うrun関数です
    └── main.rs # 起動関数です
```