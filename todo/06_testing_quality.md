# [TEST01] Testing / Quality Assurance

## 1. Problem / Context
Streaming input varies by environment; weak tests make regressions hard to catch.

## 2. Definition of Done
- Unit tests cover core logic
- Integration tests cover representative flows
- Debugging guide exists

## 3. Action Items
- [x] Add unit tests for line split / field extraction
- [x] Add unit tests for Top N
- [x] Add integration tests for short streams
- [x] Define quick perf benchmark method
- [x] Create troubleshooting guide

## 4. References
- README.md
- PRD.md

## 5. Quick Performance Check

### Goal
- Sustain 10k lines/sec for 30 seconds

### Preconditions
- Local execution
- Release build

### Steps
1. Build
   - `cargo build --release`
2. Generate input (300k lines)
   - `seq 1 300000 | awk '{print "path=/api/v1/users"}' > /tmp/tally_bench.log`
3. Measure
   - `time cat /tmp/tally_bench.log | ./target/release/tally -f 1 -d '=' -n 5 --interval 200 > /dev/null`

### Pass Criteria
- real time <= 30 seconds
- no crash

## 6. Troubleshooting Guide

### Input not read / stuck
- Ensure stdin is open (`cat file | tally`)
- Check trailing newline behavior

### Output not refreshing
- Check if `--interval` is too large
- Confirm input is flowing (`pv`)

### Aggregation incorrect
- `-f` is 1-based
- Check delimiter settings
- Confirm behavior with consecutive delimiters

### Mojibake
- Ensure UTF-8 input
- Check `LC_ALL`

## 7. Phase 2 Test Items
- [x] Boundary tests for `-f` (0/negative/large)
- [x] `-d` single-char/empty/multi-char errors
- [x] `--help` content (options/defaults)

## 8. v0.4.0 Stability Tests
- [x] Exception handling for PermissionDenied in stdin
- [x] BrokenPipe handling for stdout

---

# [TEST01] テスト/品質保証

## 1. 課題・現状 (Problem/Context)
ストリーム処理は環境差や入力揺れが大きく、テストが弱いと回帰が見つけにくい。

## 2. 達成すること (Definition of Done)
- 主要ロジックに単体テストがある
- 代表的なユースケースの統合テストがある
- 失敗時のデバッグ手順が整理されている

## 3. やること (Action Items)
- [x] 行分割/フィールド抽出の単体テストを追加する
- [x] Top N計算の単体テストを追加する
- [x] 短いストリームを用いた統合テストを追加する
- [x] 性能の簡易ベンチ/計測方法を整理する
- [x] 失敗時の切り分けガイドを用意する

## 4. 参考・メモ (References)
- README.md
- PRD.md

## 5. 性能計測の簡易手順

### 目的
- 1万行/秒を30秒処理できることを確認する

### 前提
- ローカル環境で実行
- リリースビルドを使用

### 手順
1. ビルド
   - `cargo build --release`
2. テスト入力生成（30万行）
   - `seq 1 300000 | awk '{print "path=/api/v1/users"}' > /tmp/tally_bench.log`
3. 計測
   - `time cat /tmp/tally_bench.log | ./target/release/tally -f 1 -d '=' -n 5 --interval 200 > /dev/null`

### 判定
- `time` の real が30秒以内で完了すること
- 実行中にクラッシュしないこと

## 6. 失敗時の切り分けガイド

### 入力が読めない/止まる
- 標準入力が閉じているか確認（`cat file | tally` で再現）
- ファイル末尾の改行有無を確認（末尾改行なしでも処理できるか）

### 出力が更新されない
- `--interval` が極端に大きくないか確認
- 入力が流れているか確認（`pv` で流速を見る）

### 集計結果が期待と違う
- `-f` の1始まりを確認
- 区切りが空白か `-d` か確認
- 連続区切り時の空フィールド扱いを確認

### 文字化けする
- UTF-8以外の入力が混在していないか確認
- `LC_ALL` の設定を確認

## 7. Phase 2向けテスト項目
- [x] `-f` の境界値テスト（0/負数/大きい値）
- [x] `-d` の1文字制約/空文字/複数文字のエラー確認
- [x] `--help` の表示内容（オプション一覧/既定値）

## 8. v0.4.0安定性テスト
- [x] stdinのPermissionDenied例外ハンドリング
- [x] stdoutのBrokenPipeハンドリング
