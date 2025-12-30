# Roadmap & Versioning Strategy

## 1. Versioning Strategy

We follow Semantic Versioning 2.0.0. Before v1.0.0 (v0.x.y), use the rules below.

### v0.x Rules
- **v0.x.0 (Minor update):** New features and breaking changes
- **v0.x.y (Patch update):** Bug fixes, docs, internal performance only

### v1.0.0+ Rules
- **Major (X.0.0):** Breaking changes
- **Minor (0.X.0):** Backward-compatible features
- **Patch (0.0.X):** Backward-compatible fixes

---

## 2. Phases & Milestones

### 🏁 Phase 1: MVP - "It Works" (v0.1.0)
Goal: Establish basic pipeline processing and real-time display.

- [x] **Core:** Stream stdin and process per line
- [x] **Logic:** In-memory HashMap aggregation
- [x] **TUI:** Clear screen and render Top N
- [x] **CLI:** Basic options (`-n`, `--interval`)
- [x] **Ops:** CI (GitHub Actions)

---

### 🛠 Phase 2: Usability - "Daily Driver" (v0.2.0 - v0.4.0)
Goal: Make daily use comfortable and flexible.

**Priority start:** boundary tests for `-f`/`-d` and richer `--help`.

#### v0.2.0: Flexible Input
- [x] Full `-f, --field` implementation with boundary tests
- [x] `-d, --delimiter` (CSV/TSV)
- [x] Replace invalid UTF-8 bytes during processing
- [x] Enrich `--help` with defaults and constraints

#### v0.3.0: Interactivity
- [x] Non-blocking key input
- [x] `Space`: pause/resume
- [x] `r`: reset counts
- [x] `q`: quit
- [x] Separate render loop and input handling
- [x] Define state transitions (run/pause/reset/quit)

#### v0.4.0: Stability & UX
- [ ] Stronger error handling (permissions, broken pipes)
- [ ] Richer `--help`
- [ ] Color output (bar colors)
- [ ] Improved layout for terminal width changes

---

### 📦 Phase 3: Distribution & Ecosystem (v0.5.0 - v0.7.0)
Goal: Make installation easy for everyone.

#### v0.5.0: Release Automation
- [ ] GitHub Releases with binaries
- [ ] Multi-platform builds (Linux/macOS/Windows)

#### v0.6.0: Package Managers
- [ ] Homebrew tap
- [ ] `cargo-binstall` support

#### v0.7.0: Structured Data
- [ ] JSON log parsing (`--json key`)

---

### 🚀 Phase 4: Performance & Scale (v0.8.0 - v1.0.0)
Goal: Reliability under heavy load and long-running use.

#### v0.8.0: High Performance
- [ ] Faster hashing (`ahash` / `fxhash`)
- [ ] Channel optimization (`flume` etc.)
- [ ] Reduced allocations (zero-copy parsing)

#### v0.9.0: Scalability
- [ ] Count-Min Sketch option

#### v1.0.0: Stable Release
- [ ] Security review
- [ ] Freeze CLI API
- [ ] Long-term support policy

---

## 3. Git Branching

- `main`: always releasable
- `develop`: optional integration branch
- `feat/xxx`: feature branches
- `fix/xxx`: bug fix branches

## 4. Release Checklist

1. Move "Unreleased" to a version in `CHANGELOG.md`
2. `cargo test`
3. `cargo clippy`
4. Manual E2E check on real logs

---

# Roadmap & Versioning Strategy（日本語）

## 1. バージョニング方針 (Versioning Strategy)

本プロジェクトは [Semantic Versioning 2.0.0](https://semver.org/) に準拠しますが、v1.0.0 到達前の **初期開発フェーズ (v0.x.y)** においては以下のルールを適用します。

### v0.x 時代のルール
* **v0.x.0 (Minor Update):**
    * 新機能の追加、または**破壊的変更 (Breaking Changes)** を含むリリース。
    * CLIオプションの変更や削除はこのタイミングで行う。
* **v0.x.y (Patch Update):**
    * バグ修正、ドキュメント更新、内部パフォーマンス改善のみ。
    * 既存のCLIオプションの挙動は変更しない（後方互換性を維持）。

### v1.0.0 以降のルール
* **Major (X.0.0):** 破壊的な変更。
* **Minor (0.X.0):** 後方互換性のある機能追加。
* **Patch (0.0.X):** 後方互換性のあるバグ修正。

---

## 2. 開発フェーズとマイルストーン

### 🏁 Phase 1: MVP - "It Works" (v0.1.0)
**ゴール:** パイプライン処理とリアルタイム表示の基本動作を確立する。

- [x] **Core:** 標準入力からのストリーム読み込みと行単位処理
- [x] **Logic:** HashMapによるメモリ内集計
- [x] **TUI:** 画面クリアとTop Nランキングの描画
- [x] **CLI:** 基本オプション (`-n`, `--interval`) の実装
- [x] **Ops:** CI (GitHub Actions) の構築

---

### 🛠 Phase 2: Usability - "Daily Driver" (v0.2.0 - v0.4.0)
**ゴール:** 日常的な業務や調査でストレスなく使える柔軟性を持たせる。

**優先着手:** v0.2.0 の `-f`/`-d` 境界値テストと `--help` の充実化

#### v0.2.0: Flexible Input
`cut` コマンドに頼らず、tally単体でフィールド抽出ができるようにする。
- [x] フィールド指定 (`-f, --field`) の完全実装と境界値テスト
- [x] 区切り文字指定 (`-d, --delimiter`) の実装（CSV/TSV対応）
- [x] UTF-8の不正バイトは置換して処理する方針を明記・検証
- [x] `--help` の詳細化（既定値/制約の明記）

#### v0.3.0: Interactivity
流れるログを「止めて見る」「リセットする」対話機能。
- [x] キー入力ハンドリング（非ブロッキング）
- [x] `Space`: 一時停止/再開
- [x] `r`: カウントのリセット
- [x] `q`: 即時終了
- [x] 画面描画と入力処理のイベントループを分離
- [x] 主要キー操作の状態遷移（run/pause/reset/quit）を定義

#### v0.4.0: Stability & UX
- [ ] エラーハンドリングの強化（権限エラー、パイプ切断時の挙動改善）
- [ ] ヘルプメッセージ (`--help`) のリッチ化
- [ ] カラー出力対応（バーチャートの色分けなど）
- [ ] 端末幅に応じた表示最適化の強化

---

### 📦 Phase 3: Distribution & Ecosystem (v0.5.0 - v0.7.0)
**ゴール:** 誰でも簡単にインストールして使える状態にする。

#### v0.5.0: Release Automation
- [ ] GitHub Releases へのバイナリ自動アップロード
- [ ] マルチプラットフォームビルド (Linux, macOS, Windows)

#### v0.6.0: Package Managers
- [ ] Homebrew Tap の作成 (`brew install tally`)
- [ ] `cargo-binstall` 対応

#### v0.7.0: Structured Data (The "Killer Feature")
- [ ] JSONログのパース対応 (`--json key`)
    - 例: `tail -f app.log | tally --json level` でJSON内の特定キーを集計

---

### 🚀 Phase 4: Performance & Scale (v0.8.0 - v1.0.0)
**ゴール:** 本番環境の高負荷・長時間稼働に耐える信頼性を確立する。

#### v0.8.0: High Performance
- [ ] 高速ハッシュアルゴリズム (`ahash` / `fxhash`) の導入
- [ ] ロック競合の最適化（`flume` などのチャネル検討）
- [ ] メモリ割り当ての最小化（Zero-copy parsing）

#### v0.9.0: Scalability
- [ ] 近似集計アルゴリズム (Count-Min Sketch) の導入オプション
    - メモリ固定で数億ユニークIPなどを扱えるようにする

#### v1.0.0: Stable Release
- [ ] 全機能のセキュリティ監査
- [ ] CLI APIの固定（これ以降の破壊的変更を禁止）
- [ ] 长期サポート方針の策定

---

## 3. Git ブランチ運用

* **`main`**: 常にビルド可能で、リリースタグが打たれるブランチ。
* **`develop`**: 次期バージョンの開発用（必要に応じて作成）。
* **`feat/xxx`**: 機能追加用トピックブランチ。PRを経てマージされる。
* **`fix/xxx`**: バグ修正用。

## 4. リリースチェックリスト

リリース前には必ず以下を実施する:
1. `CHANGELOG.md` の "Unreleased" をバージョン番号へ移動
2. `cargo test` 全通過を確認
3. `cargo clippy` の警告ゼロを確認
4. 実際のログファイルを用いた動作確認（End-to-End）
