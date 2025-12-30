# [P01] OSS Product Definition and Scope

## 1. Problem / Context
Without a clear purpose, target users, and MVP scope, design/implementation/release decisions will drift.

## 2. Definition of Done
- Mission/value/target users are summarized succinctly
- MVP in-scope/out-of-scope is clarified with reasons
- Three primary use cases are defined
- Success metrics and priorities are defined

## 3. Action Items
- [x] Summarize mission, value proposition, and target users
- [x] Clarify MVP scope in/out with reasons
- [x] Define three use cases with input examples
- [x] Define success metrics for “experience/performance/onboarding”
- [x] Define v0.x phases briefly

## 4. References
- README.md
- ROADMAP.md
- PRD.md

## 5. Use Cases and Success Metrics

### Use Cases
1. Monitor popular paths in access logs
   - Example: `tail -f access.log | cut -d ' ' -f 7 | tally`
2. Detect spikes in error logs
   - Example: `tail -f app.log | grep ERROR | tally`
3. Monitor API status code distribution
   - Example: `tail -f access.log | tally -f 9`

### Success Metrics
- Experience: render rankings within 2 seconds
- Performance: sustain 10k lines/sec for 30 seconds
- Onboarding: runnable within 10 minutes using README

### Sample Logs
- `samples/access.log`
- `samples/app.log`

## 6. One-page Summary

### Mission
Reduce waiting time and cognitive load in log analysis so users can see “what is happening now” in the terminal.

### Value Proposition
- Real-time aggregation for immediate insights
- Simple pipe-based usage
- Local-first, no extra infra

### Target Users
- SRE / Infrastructure engineers
- Backend engineers
- On-call responders

## 7. MVP Scope with Rationale

### Scope In (Why)
- stdin streaming: essential entry point
- In-memory aggregation: minimal cost, fast MVP
- TUI Top N display: core differentiation
- `-f` field selection: minimal cut/awk replacement
- Final output on exit: reuse results

### Scope Out (Why)
- Disk-based aggregation: too heavy for MVP
- Complex query language: higher learning cost
- Log storage: not aligned with viewer role

## 8. v0.x Phases (Short)
- v0.1: MVP (streaming + TUI)
- v0.2–0.5: usability (`-f/-d`/display options)
- v0.6–0.7: distribution (binaries/Homebrew)
- v0.8+: performance (speed/approx)

## 9. OSS Documentation Set
- `README.md`
- `PRD.md`
- `ROADMAP.md`
- `LICENSE`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`

---

# [P01] OSS公開に向けたプロダクト定義とスコープ

## 1. 課題・現状 (Problem/Context)
OSSとして公開するための目的と対象、MVPの範囲が曖昧だと、設計/実装/公開判断がぶれる。

## 2. 達成すること (Definition of Done)
- ミッション/価値提案/対象ユーザーが短く言語化されている
- MVPのスコープ内/外が明確になっている
- 主要ユースケースが3つ定義されている
- 成功指標と優先度が整理されている

## 3. やること (Action Items)
- [x] ミッション、価値提案、対象ユーザーを1ページに要約する
- [x] MVPスコープ内/外を明確化し、理由を添える
- [x] 主要ユースケース3つを定義し、入力例を添える
- [x] 成功指標を「体感」「性能」「導入容易性」で定義する
- [x] v0.xのフェーズ分割を簡潔に設定する

## 4. 参考・メモ (References)
- README.md
- ROADMAP.md
- PRD.md

## 5. ユースケースと成功指標の具体案

### ユースケース
1. アクセスログの人気パスをライブ監視する
   - 入力例: `tail -f access.log | cut -d ' ' -f 7 | tally`
2. エラーログの急増を検知する
   - 入力例: `tail -f app.log | grep ERROR | tally`
3. APIレスポンスコードの分布を監視する
   - 入力例: `tail -f access.log | tally -f 9`

### 成功指標
- 体感: 実行から2秒以内にランキングが描画される
- 性能: 1万行/秒の入力で30秒間落ちない
- 導入容易性: READMEの手順だけで10分以内に動かせる

### サンプルログ
- `samples/access.log`
- `samples/app.log`

## 6. 1ページ要約

### ミッション
ログ解析の待ち時間と認知負荷を減らし、ターミナルで「今起きていること」を即座に把握できるようにする。

### 価値提案
- リアルタイム集計で今多いものがすぐ見える
- パイプで繋ぐだけのシンプル操作
- 追加基盤なしでローカル完結

### 対象ユーザー
- SRE/インフラエンジニア
- バックエンド開発者
- オンコール対応者

## 7. MVPスコープ内/外の理由

### Scope In (理由)
- stdinストリーム処理: 主要ユースケースの入口で必須
- インメモリ集計: MVPで最小の実装コストで高速
- TUIランキング表示: 本ツールの差別化価値の中心
- `-f` フィールド指定: cut/awkの代替として必要最小限
- 終了時の最終出力: 解析結果の再利用に必要

### Scope Out (理由)
- ディスクベース処理: 実装/運用コストが高くMVP超過
- 複雑なクエリ言語: 学習コストが増えシンプルさと逆行
- ログ保存/蓄積: ビューアとしての位置付けと不一致

## 8. v0.xフェーズの簡潔版
- v0.1: MVP（リアルタイム集計とTUI）
- v0.2-0.5: 使い勝手改善（-f/-d/表示設定）
- v0.6-0.7: 配布強化（バイナリ/ホームブリュー）
- v0.8+: 性能強化（高速化/近似集計）

## 9. OSSドキュメント構成
- `README.md`
- `PRD.md`
- `ROADMAP.md`
- `LICENSE`
- `CHANGELOG.md`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
