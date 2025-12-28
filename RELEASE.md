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
