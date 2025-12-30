# Release Process

## Versioning
This project follows SemVer.

## Steps
1. Update `CHANGELOG.md` under "Unreleased"
2. Bump version in `Cargo.toml`
3. Run `cargo test`, `cargo fmt --all -- --check`, `cargo clippy -- -D warnings`
4. Verify package metadata (`Cargo.toml`) and crate name
5. Tag the release: `git tag vX.Y.Z`
6. Push tag and publish release notes
7. Publish to crates.io: `cargo publish`

## Notes
- Keep "Unreleased" in `CHANGELOG.md` after each release

---

# リリース手順（日本語）

## バージョニング
SemVerに従います。

## 手順
1. `CHANGELOG.md` の "Unreleased" を更新
2. `Cargo.toml` のバージョンを更新
3. `cargo test` / `cargo fmt --all -- --check` / `cargo clippy -- -D warnings` を実行
4. パッケージメタデータとcrate名を確認
5. タグ作成: `git tag vX.Y.Z`
6. タグをpushし、リリースノートを公開
7. crates.ioへ公開: `cargo publish`

## 補足
- リリース後も "Unreleased" は残す
