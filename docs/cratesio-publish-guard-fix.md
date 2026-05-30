# Crates.io publish guard: User-Agent fix

The Release workflow `publish_guard` step must send a `User-Agent` header when calling the crates.io API. Without it, the API returns **403** and `cargo publish` never runs.

See `.github/workflows/release.yml` — apply the change in PR #22 via the suggested commit on that file, or merge the workflow commit after:

```bash
gh auth refresh -h github.com -s workflow
git push origin fix/cratesio-publish-guard-user-agent
```
