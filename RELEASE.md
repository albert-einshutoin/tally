# Release Process

## Versioning
This project follows SemVer.

## Steps
1. Update `CHANGELOG.md` under "Unreleased"
2. Bump version in `Cargo.toml`
3. Run `cargo test`, `cargo fmt --all -- --check`, `cargo clippy -- -D warnings`
4. Tag the release: `git tag vX.Y.Z`
5. Push tag and publish release notes

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
4. タグ作成: `git tag vX.Y.Z`
5. タグをpushし、リリースノートを公開

## 補足
- リリース後も "Unreleased" は残す
