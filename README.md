# tally - The `top` command for log streams

[![CI](https://github.com/OWNER/tally/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/tally/actions/workflows/ci.yml)

`tally` は標準入力のログをリアルタイムで集計し、頻出項目をランキング表示するCLIツールです。
`sort | uniq -c | sort -nr` のように全件読み込み完了を待たず、実行直後から「今多いもの」を見られます。

## 主な特徴

- リアルタイム集計: 入力が流れ続けてもトップNが更新され続ける
- TUI表示: バーチャート付きの見やすいランキング
- 高速: Rust実装で高スループットに対応
- シンプル: パイプで繋ぐだけ、複雑なクエリ不要

## 使いどころ

- アクセスログの人気パスをライブ監視
- 失敗ステータスやエラー文言の急増を検知
- バッチでは追いにくいトラブルシューティングの一次調査

## インストール

```bash
# Cargo (Rust)
cargo install tally

# 将来的に Homebrew / バイナリ配布を予定
```

## 使い方

```bash
tail -f access.log | cut -d ' ' -f 7 | tally
```

### フィールド指定（cut不要）

```bash
# 空白区切りの7番目を集計
tail -f access.log | tally -f 7

# カンマ区切りの3番目を集計
tail -f access.log | tally -f 3 -d ','
```

## オプション（予定含む）

- `-f, --field <N>`: 区切り文字で分割したN番目の要素を集計
- `-d, --delimiter <CHAR>`: 区切り文字を指定
- `-n, --top <N>`: 上位N件のみ表示
- `--interval <MS>`: 描画更新間隔（ミリ秒）

## 競合とポジショニング

`tally` は「リアルタイム性」と「学習コストの低さ」を両立します。

| ツール | リアルタイム性 | 学習コスト | 表示 | 備考 |
| --- | --- | --- | --- | --- |
| tally | 高い | 低い | TUI | パイプで利用 |
| sort | uniq | 低い | 低い | テキスト | 完了待ち |
| angle-grinder | 高い | 高い | TUI | 高機能 |

## 開発

```bash
# ビルド
cargo build

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy
```

## 性能計測（簡易）

```bash
# リリースビルド
cargo build --release

# サンプル入力での簡易計測
time cat samples/bench.log | ./target/release/tally -f 1 -d '=' -n 5 --interval 200 > /dev/null
```

## コントリビューション

- まずは `ROADMAP.md` と `PRD.md` を確認してください
- 開発フローとルールは `CONTRIBUTING.md` を参照してください

## ドキュメント

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `CHANGELOG.md`
- `RELEASE.md`
- `LICENSE`

## ライセンス

- MIT License（`LICENSE`）

## セキュリティ

- 報告手順は `SECURITY.md` を参照してください
