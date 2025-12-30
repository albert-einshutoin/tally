# [IMP01] Implementation: Core MVP & CLI

## 1. Problem / Context
Without streaming aggregation and TUI, we cannot validate MVP value.

## 2. Definition of Done
- stdin stream aggregation works
- Top N updates in real time
- CLI options work end-to-end

## 3. Action Items
- [x] Implement input reading and line splitting
- [x] Implement field extraction (-f) and delimiter (-d)
- [x] Implement aggregation storage and Top N
- [x] Implement TUI rendering (ranking + bars)
- [x] Implement final output on signal exit
- [x] Implement CLI parsing

## 4. References
- PRD.md
- ROADMAP.md

## 5. Implementation Order (MVP)
1. CLI parsing (`-f`, `-d`, `-n`, `--interval`)
2. Input read + line split
3. Field extraction + delimiter handling
4. Aggregation + Top N
5. TUI render loop
6. Final output on exit

## 6. Detailed Tasks

### CLI
- [x] `--field` 1-based indexing and boundary handling
- [x] `--delimiter` single-char and default space
- [x] `--top` default and upper bound
- [x] `--interval` min/max clamp

### Input / Extraction
- [x] Decide and implement empty line/field handling
- [x] Decide handling for consecutive delimiters
- [x] Define UTF-8 behavior

### Aggregation
- [x] HashMap count updates
- [x] Top N algorithm (full sort in MVP)
- [x] Tie-break ordering

### Rendering
- [x] TUI ranking and bar rendering
- [x] Clear & redraw strategy
- [x] Bar length based on terminal width

### Termination
- [x] Safe exit on SIGINT/SIGTERM
- [x] Final output format

### Interactivity (v0.3.0)
- [x] Non-blocking input (`crossterm` polling)
- [x] `Space` pause/resume
- [x] `r` reset counts
- [x] `q` quit
- [x] Show state (running/paused) in header

### Stability (v0.4.0)
- [x] Handle BrokenPipe safely
- [x] Restore raw mode on drop
- [x] Permission/read error messaging
- [x] Redraw on terminal resize
- [x] Color output for bars
- [x] Layout optimization for narrow terminals
- [x] NO_COLOR support to disable ANSI output
- [x] Debounce resize redraws

### Performance (v0.5.0)
- [x] Reuse line buffers to reduce per-line allocations
- [x] Table UI (rank/count/percent/key) without bars
- [x] JSON key aggregation (`--json`)
- [x] Fixed-width table layout (`--width`)
- [x] ANSI color toggle (`--no-color`)
- [x] Disable final text output (`--no-final`)

## 7. Spec Notes

### Input / Extraction
- Default delimiter: space (`split_whitespace`), ignore consecutive whitespace
- With `-d`, keep empty fields
- Skip empty lines
- Skip lines with missing field
- Treat input as UTF-8 and replace invalid bytes

### Top N
- Full sort per render (MVP)
- Sort by count desc, key asc

---

# [IMP01] 実装: コア機能(MVP)とCLI

## 1. 課題・現状 (Problem/Context)
MVPとして必要なストリーム集計とTUI表示が未整備だと、価値検証ができない。

## 2. 達成すること (Definition of Done)
- stdinのストリーム集計が動作する
- Top Nのランキングがリアルタイムに更新される
- CLIオプションが一通り動作する

## 3. やること (Action Items)
- [x] 入力読み込みと行分割を実装する
- [x] フィールド抽出(-f)と区切り文字(-d)を実装する
- [x] 集計ストレージとTop N計算を実装する
- [x] TUI描画(ランキング+バー)を実装する
- [x] シグナル終了時の最終出力を実装する
- [x] CLIオプション解析を実装する

## 4. 参考・メモ (References)
- PRD.md
- ROADMAP.md

## 5. 実装順序（MVP）
1. CLIオプション解析（`-f`, `-d`, `-n`, `--interval`）
2. 入力読み込みと行分割
3. フィールド抽出と区切り文字処理
4. 集計ストレージとTop N計算
5. TUI描画ループ
6. シグナル終了時の最終出力

## 6. 詳細タスク

### CLI
- [x] `--field` の1始まりインデックスと境界処理
- [x] `--delimiter` の1文字制約とデフォルト空白
- [x] `--top` のデフォルト値と上限
- [x] `--interval` の最小/最大値の制約

### 入力/抽出
- [x] 空行/空フィールドの扱いを決めて実装する
- [x] 区切り連続時のフィールド解釈を決める
- [x] UTF-8入力の扱いを明確化する

### 集計
- [x] HashMapでのカウント更新処理
- [x] Top Nの抽出アルゴリズム（全件ソート or 部分ソート）
- [x] 同率時の並び順の定義

### 表示
- [x] TUIのランキング表示とバー描画
- [x] 画面クリア/再描画の方式を決める
- [x] 表示幅に応じたバー長の調整

### 終了処理
- [x] SIGINT/SIGTERMでの安全な終了
- [x] 最終結果のstdout出力フォーマット

### インタラクティブ（v0.3.0）
- [x] 非ブロッキング入力（`crossterm` のポーリング）
- [x] `Space` で一時停止/再開
- [x] `r` でカウント初期化
- [x] `q` で終了
- [x] 状態表示（running/paused）をヘッダーに反映

### 安定性（v0.4.0）
- [x] stdoutのBrokenPipeを安全に扱う
- [x] raw modeをDropで確実に復元する
- [x] 権限/読み取りエラーのユーザー向けメッセージ整備
- [x] 端末リサイズ時の再描画
- [x] バーチャートのカラー出力
- [x] 狭い端末幅でのレイアウト調整
- [x] NO_COLORでANSI出力を無効化
- [x] リサイズ再描画のデバウンス

### パフォーマンス（v0.5.0）
- [x] 行バッファ再利用でアロケーションを削減
- [x] バーなしテーブルUI（順位/件数/割合/キー）
- [x] JSONキー集計（`--json`）
- [x] テーブル幅固定（`--width`）
- [x] ANSIカラー切替（`--no-color`）
- [x] EOF後の最終テキスト出力を無効化（`--no-final`）

## 7. 仕様確定メモ

### 入力/抽出
- デフォルト区切りは空白。空白区切りは `split_whitespace` 相当で連続空白は無視
- 明示的な区切り文字 `-d` 指定時は1文字で分割し、空フィールドを保持
- 空行はスキップする
- 指定フィールドが存在しない行はスキップする
- UTF-8として処理し、不正バイトは置換扱い

### Top N
- 描画タイミングごとに全件をVec化してソート（MVPは単純実装）
- ソート順は「件数の降順、キーの昇順」
