# Product Requirements Document (PRD)

## 1. Mission
Reduce waiting time and cognitive load in log analysis so users can see “what is happening now” in the terminal.

## 2. Background & Problems

- `tail -f` is too fast to read
- `sort | uniq -c` needs full input before output, not suitable for incidents
- Heavy log platforms are expensive and slow to set up for quick local analysis

## 3. Target Users

- SRE / Infrastructure engineers
- Backend engineers
- On-call responders

## 4. Value Proposition

- Real-time aggregation: rankings update immediately
- Simple usage: pipe-based, no query language
- Local-first: no extra infra required

## 5. Competition & Differentiation

| Alternative | Strength | Weakness | tally advantage |
| --- | --- | --- | --- |
| sort | uniq | Low cost | Batch only | Real-time |
| datamash | Stats features | Needs sorted input | No sort required |
| angle-grinder | Powerful | High learning cost | Simple usage |

## 6. Scope

### Scope In (MVP)
- stdin line streaming
- In-memory HashMap aggregation
- TUI Top N display
- Field selection (`-f`)
- Final results on exit

### Scope Out (MVP)
- Disk-based aggregation for huge cardinality
- Complex query language
- Log storage

## 7. Non-functional Requirements

- High throughput (tens of thousands of lines/sec)
- Stable long-running behavior
- Readable TUI

## 8. Success Metrics

- Feels more “real-time” than `sort | uniq`
- Usable within minutes from README
- Works same day without setup

## 9. Release Policy

- Follow phases in `ROADMAP.md`
- SemVer-compatible changes

---

# Product Requirements Document (PRD)（日本語）

## 1. ミッション
ログ解析における「待ち時間」と「認知負荷」を減らし、
ターミナル上で「今起きていること」を即座に把握できるようにする。

## 2. 背景と課題

- `tail -f` は流速が速いと追えない
- `sort | uniq -c` は全件読み込みが必要で、緊急対応に使いにくい
- 高機能なログ基盤は導入コストが高く、手元の端末で完結したい

## 3. ターゲットユーザー

- SRE / インフラエンジニア
- バックエンド開発者
- オンコール対応者

## 4. 価値提案

- リアルタイム集計: 実行直後からランキングが更新され続ける
- シンプル操作: パイプで繋ぐだけ、学習コストが低い
- ローカル完結: 追加の基盤やセットアップ不要

## 5. 競合と差別化

| 競合/代替 | 強み | 弱み | tallyの差別化 |
| --- | --- | --- | --- |
| sort | uniq | 低コスト | 完了待ち | リアルタイム性 |
| datamash | 統計機能 | 入力ソート必要 | ソート不要 |
| angle-grinder | 高機能 | 学習コスト | シンプル操作 |

## 6. スコープ

### Scope In (MVP)
- stdinの行ストリーム処理
- HashMapによるインメモリ集計
- Top NランキングのTUI表示
- フィールド指定（`-f`）
- 終了時に最終結果を出力

### Scope Out (MVPではやらない)
- ディスクベース処理（巨大ユニーク数への対応）
- 複雑なクエリ言語
- ログの保存・蓄積

## 7. 非機能要件

- 高スループット（数万行/秒の処理を想定）
- 長時間稼働に耐える安定性
- TUIの視認性

## 8. 成功指標

- `sort | uniq` より「今が見える」と感じること
- セットアップなしで即日使えること
- 開発者が README を見て数分で試せること

## 9. リリース方針

- `ROADMAP.md` のフェーズに従う
- 互換性方針はSemVerに準拠
