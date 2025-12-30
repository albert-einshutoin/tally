# Zero-Copy Aggregation Design (v0.8.0)

## Goal
Reduce per-line allocations by aggregating on byte slices and deferring UTF-8 conversion to display.

## Core Ideas
- Keep input as bytes (`Vec<u8>`) and split without creating `String`
- Use byte keys for aggregation
- Convert to UTF-8 only when rendering

## Design Options

### Option A: Aggregate in Reader Thread
- Reader thread performs field extraction and updates a shared map
- Main thread only renders snapshots
- Avoids per-line message passing overhead

### Option B: Byte-Key HashMap
- Store keys as `Vec<u8>` in `HashMap<Vec<u8>, usize>`
- On each line, create a key only when needed
- Render uses `String::from_utf8_lossy` on the fly

### Option C: Interned Keys
- Add a key-intern table (`HashMap<Vec<u8>, Arc<[u8]>>`)
- Frequent keys reuse the same allocation
- Higher complexity, better memory behavior

## Parsing Strategy
- Custom byte-splitting by delimiter
- For whitespace, treat consecutive whitespace as a single separator
- For custom delimiter, keep empty fields

## Trade-offs
- Byte-keys complicate display but reduce allocations
- Interning improves memory but adds overhead
- Reader-thread aggregation improves throughput but complicates locking

## Next Steps
- Build a benchmark harness for byte-key vs String-key
- Decide on Option A/B/C based on throughput and memory
- Document final design in `ROADMAP.md`

---

# ゼロコピー集計設計（v0.8.0）

## 目的
行ごとのアロケーションを削減し、UTF-8変換は表示時まで遅延する。

## 主要方針
- 入力はバイト列のまま扱う（`Vec<u8>`）
- 集計キーはバイト列で保持
- 表示時にのみUTF-8変換

## 設計案

### 案A: Readerスレッド内で集計
- Reader側で抽出と集計を実施し、メインは描画のみ
- 行ごとのメッセージ送信を回避

### 案B: バイトキーHashMap
- `HashMap<Vec<u8>, usize>` に集計
- 必要なときのみキーを確保

### 案C: インターン化
- キーの再利用テーブルを導入
- 高頻度キーの再アロケーションを抑制

## パース方針
- デリミタ区切りのバイト分割
- 空白は連続空白を1区切りとして扱う
- カスタム区切りは空フィールドを保持

## トレードオフ
- 表示が複雑になるがアロケーション削減
- インターン化は効果があるが複雑
- Reader集計は速いが同期が必要

## 次の作業
- バイトキー/文字列キーのベンチ比較
- A/B/Cの採用判断
- `ROADMAP.md` に最終方針を反映
