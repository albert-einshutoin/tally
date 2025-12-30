# [D01] Design: Core Architecture / Data Flow

## 1. Problem / Context
If responsibilities between input, aggregation, and rendering are unclear, performance and maintainability suffer.

## 2. Definition of Done
- Input/aggregation/rendering responsibilities are separated
- Data flow and refresh timing are documented
- Public CLI behavior is defined

## 3. Action Items
- [x] Decide input pipeline (read/split/extract)
- [x] Decide aggregation storage (hash/top N)
- [x] Decide render loop (interval/diff)
- [x] Finalize CLI options (-f, -d, -n, --interval)
- [x] Define behavior on termination signals

## 4. References
- PRD.md
- ROADMAP.md

## 5. CLI Priority
1. Required: `-f, --field`
2. Required: `-d, --delimiter`
3. Recommended: `-n, --top`
4. Recommended: `--interval`

## 6. Design Notes

### Input Pipeline
- Read stdin with buffering and process per line
- Single-character delimiter, default is space
- If `-f` is set, aggregate only the Nth field

### Aggregation Storage
- Use HashMap for counts, compute Top N on render
- Simple full sort per render in MVP

### Render Loop
- Re-render every `--interval`
- Clear & re-render first, optimize later

### Termination
- Capture SIGINT/SIGTERM and print final ranking to stdout

### Interactive Design (v0.3.0)
- Non-blocking input via `crossterm` event polling
- Rendering via interval timer, input polled at short intervals

### State Transitions (v0.3.0)
- States: `running` / `paused` / `quitting`
- `running` --(Space)--> `paused`
- `paused` --(Space)--> `running`
- `running` --(r)--> `running` (clear counts)
- `paused` --(r)--> `paused` (clear counts)
- `running` --(q)--> `quitting`
- `paused` --(q)--> `quitting`

### Event Loop (v0.3.0)
- Input poll: every 50ms via `crossterm::event::poll`
- Priority: handle input (q/space/r) before render decision
- Render interval: `--interval`, render immediately on first draw
- stdin reading: separate thread, non-blocking receive in main loop

---

# [D01] 設計: コアアーキテクチャ/データフロー

## 1. 課題・現状 (Problem/Context)
ストリーム入力、集計、描画の責務が曖昧だと、性能劣化や変更時の影響範囲が増える。

## 2. 達成すること (Definition of Done)
- 入力/集計/描画の責務が分離されている
- データフローと更新タイミングが説明できる
- 公開API(CLI)の仕様が整理されている

## 3. やること (Action Items)
- [x] 入力パイプライン(読み込み/行分割/フィールド抽出)の設計を決める
- [x] 集計ストレージ(ハッシュ/トップN計算)の方針を決める
- [x] 描画ループ(更新間隔/差分描画)の設計を決める
- [x] CLIオプションの設計を固める(-f, -d, -n, --interval)
- [x] 停止シグナル時の挙動(最終出力)を設計する

## 4. 参考・メモ (References)
- PRD.md
- ROADMAP.md

## 5. CLI仕様の優先度
1. 必須: `-f, --field` (フィールド指定)
2. 必須: `-d, --delimiter` (区切り文字)
3. 推奨: `-n, --top` (上位N件)
4. 推奨: `--interval` (描画更新間隔)

## 6. 設計メモ

### 入力パイプライン
- stdinをバッファ読み込みし、行単位で処理
- 区切り文字は1文字指定を基本とし、未指定時は空白
- `-f` 指定がある場合はN番目のみをキーとして集計する

### 集計ストレージ
- HashMapでカウントし、描画時にTop Nをソート/抽出
- Top Nは毎描画で計算（MVP段階の単純実装）

### 描画ループ
- `--interval` 毎にランキングを再描画
- クリア&再描画の単純実装を先行、最適化は後続

### 終了挙動
- SIGINT/SIGTERMを捕捉し、最終ランキングをstdoutへ出力

### インタラクティブ設計（v0.3.0案）
- 入力処理は `crossterm` のイベントポーリングで非ブロッキング化
- 描画は一定間隔のタイマー、入力は短いポーリング間隔で監視

### 状態遷移（v0.3.0）
- 状態: `running` / `paused` / `quitting`
- `running` --(Space)--> `paused`
- `paused` --(Space)--> `running`
- `running` --(r)--> `running`（カウント初期化）
- `paused` --(r)--> `paused`（カウント初期化）
- `running` --(q)--> `quitting`
- `paused` --(q)--> `quitting`

### イベントループ（v0.3.0）
- 入力ポーリング: 50ms間隔で `crossterm::event::poll`
- 描画優先度: 入力（q/space/r）処理後に描画判定
- 描画周期: `--interval` に従う（初回は即時描画）
- stdin読み込み: 別スレッドで行単位に受信し、メインは非ブロッキングで処理
