# [REL01] Release / Publication / Operations

## 1. Problem / Context
OSS release requires release flow, documentation, and support process.

## 2. Definition of Done
- Release procedure and versioning defined
- Core docs are available
- Post-release support policy defined

## 3. Action Items
- [x] Define release flow (tag/version/distribution)
- [x] Update README/FAQ/examples
- [x] Define CHANGELOG policy
- [x] Prepare Issue/PR templates
- [x] Define post-release support/maintenance policy

## 4. References
- README.md
- ROADMAP.md

## 5. README/FAQ/Examples Update Policy
- README: clearly distinguish implemented vs planned features
- README: add examples using `samples/`
- FAQ: quick Q/A for stuck output or wrong results
- Examples: combinations of `-f`/`-d`/`-n`/`--interval`

## 6. Post-release Support Policy
- Support channel: GitHub Issues (labels `question`/`bug`/`enhancement`)
- Response target: first response within 1 week
- Bug fixes: prioritize high severity, batch minor fixes
- Security: follow `SECURITY.md`

---

# [REL01] リリース/公開/運用

## 1. 課題・現状 (Problem/Context)
OSS公開にはリリースフロー、ドキュメント、サポート導線が必要。

## 2. 達成すること (Definition of Done)
- リリース手順とバージョニングが整備されている
- 主要ドキュメントが揃っている
- 公開後の運用方針が定義されている

## 3. やること (Action Items)
- [x] リリースフロー(タグ/バージョン/配布)を定義する
- [x] README/FAQ/使用例を最新化する
- [x] CHANGELOG運用方針を決める
- [x] Issue/PRテンプレートを用意する
- [x] 公開後のサポート/メンテ方針を整理する

## 4. 参考・メモ (References)
- README.md
- ROADMAP.md

## 5. README/FAQ/使用例 更新方針
- README: 実装済み機能と予定機能の明確な区別（MVP/Phase2）
- README: `samples/` を使った具体例の追加
- FAQ: 入力が止まる/更新されない/結果が違うの簡易QA
- 使用例: `-f`/`-d`/`-n`/`--interval` の組み合わせ例

## 6. 公開後のサポート/メンテ方針
- 対応窓口: GitHub Issues（`question`/`bug`/`enhancement` ラベル運用）
- 返信目安: 1週間以内の一次返信を目標
- バグ修正: 重大度高は優先対応、軽微は次期リリースに集約
- セキュリティ: `SECURITY.md` の手順に従う
