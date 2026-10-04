# Contributing

## Branches

`main` is protected: every change lands through a pull request with green CI.
Branch names follow `<type>/<short-description>` in kebab-case, with the issue
number first when there is one:

| Prefix | Use for | Example |
|---|---|---|
| `feat/` | new functionality | `feat/ws-futures-heartbeat` |
| `fix/` | bug fixes | `fix/42-order-book-resync` |
| `refactor/` | refactoring without behaviour change | `refactor/shared-http-layer` |
| `perf/` | performance | `perf/order-book-buffer` |
| `docs/` | documentation only | `docs/readme-error-handling` |
| `test/` | tests only | `test/ws-driver-harness` |
| `chore/` | dependencies, CI, tooling | `chore/trim-dependencies` |
| `release/` | a release: version bump and final touches | `release/v0.1.11` |
| `hotfix/` | urgent fix of a released version | `hotfix/v0.1.11-signature` |

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/)
(`feat:`, `fix:`, `chore(deps):`, ...). Dependabot branches (`dependabot/...`)
are the only exception to the naming scheme.

## Checks

CI (`.github/workflows/ci.yml`) runs on every pull request: `rustfmt`,
`clippy -D warnings`, tests on Linux/macOS/Windows, the MSRV build
(`rust-version` in `Cargo.toml`), `rustdoc -D warnings`,
`cargo publish --dry-run` and `cargo deny check` (`deny.toml`). The branch
protection requires the aggregate **CI result** check.

Run the same checks locally before pushing; the git hooks in `.githooks/`
cover the fast ones (`./.githooks/install.sh`).

## Releasing

1. On a `release/vX.Y.Z` branch bump `version` in `Cargo.toml` and the pinned
   version in `README.md`, commit `chore(release): vX.Y.Z`.
2. Open a pull request to `main` and merge it once CI is green. Any merge
   method works.
3. `.github/workflows/release.yml` sees an unpublished version on `main`,
   runs the tests, publishes to crates.io (Trusted Publishing, `release`
   environment) and creates the `vX.Y.Z` tag and GitHub Release on the merged
   commit.

Do not create release tags by hand: the workflow tags the commit that actually
landed on `main`, and refuses to release if `vX.Y.Z` already points elsewhere.
