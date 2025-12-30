# [D02] Design: Security / License / Dependency Policy

## 1. Problem / Context
OSS release requires license selection and dependency review. Skipping this is risky.

## 2. Definition of Done
- License policy is decided
- Dependency license review policy exists
- Security reporting channel and process are defined

## 3. Action Items
- [x] Compare license options (MIT/Apache-2.0) and decide
- [x] Review dependency licenses and define policy
- [x] Define security reporting channel and response flow
- [x] Define privacy/log handling policy

## 4. References
- README.md
- PRD.md

## 5. Decisions & Policy

### License
- Adopt: MIT License (`LICENSE`)
- Rationale: simple, widely adopted for early OSS

### Dependencies
- No dependencies at the moment (when policy was written)
- Review license before adding dependencies

### Security Contact
- Contact is being prepared; temporary process in `SECURITY.md`

### Privacy / Logs
- stdin only; no network transmission or storage by default
- See `SECURITY.md`

---

# [D02] 設計: セキュリティ/ライセンス/依存管理

## 1. 課題・現状 (Problem/Context)
OSS公開ではライセンス選定や依存関係の整理が必須。未対応だと公開リスクが高い。

## 2. 達成すること (Definition of Done)
- ライセンス方針が決定されている
- 依存ライブラリのライセンス確認方針がある
- セキュリティ報告窓口と対応方針が決まっている

## 3. やること (Action Items)
- [x] 採用ライセンス候補(MIT/Apache-2.0等)を比較し決定する
- [x] 主要依存のライセンスを確認し、方針を整理する
- [x] セキュリティ報告窓口(メール等)と対応フローを決める
- [x] プライバシー/ログの取扱い方針を明文化する

## 4. 参考・メモ (References)
- README.md
- PRD.md

## 5. 決定事項と運用方針

### ライセンス
- 採用: MIT License（`LICENSE`）
- 理由: シンプルで採用実績が多く、OSS公開の初期段階に適合

### 依存ライブラリとライセンス方針
- 現時点で `Cargo.toml` は未作成のため依存なし
- 依存追加時は導入前にライセンス確認し、許容リストに従う

### セキュリティ窓口
- 連絡先は準備中。暫定運用は `SECURITY.md` に記載

### プライバシー/ログ取扱い
- 入力はstdinのみ、ネットワーク送信や保存は行わない方針
- 詳細は `SECURITY.md` に記載
