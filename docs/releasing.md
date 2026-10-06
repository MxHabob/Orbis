# Desktop release process

Production releases are tag-triggered. CI validates the version, builds the native solver and desktop installers, signs the Tauri updater artifacts, and creates a **draft** GitHub Release. It uploads release images and supporting resources after all platform builds succeed. A maintainer reviews the draft and publishes it manually; staging builds never publish to the production updater channel.

## Prepare a release

Start from a clean, up-to-date `main` branch after its changes have passed CI:

```sh
bun run release:prepare patch
```

Use `minor`, `major`, or an explicit stable SemVer such as `1.2.3` when appropriate. The command synchronizes the desktop package, Tauri production and staging configs, desktop Cargo package, solver sidecar Maven project, and Bun lockfile. The sidecar is bundled and released with Orbis, so its product version follows the desktop release version. It then drafts `releases/vX.Y.Z.md` from commits since the latest version tag and creates:

```text
release-assets/vX.Y.Z/
  images/       # screenshots, feature images, banners
  resources/    # supporting documents or other release files
```

The commit-derived entries are a starting point, not a substitute for release notes. Edit the summary, verify the generated changes are user-facing and accurate, and replace the Known Issues template. Add any version-specific images/resources before committing. Use unique filenames across both asset folders because GitHub Release assets share a flat filename namespace.

## Commit, tag, and build

After the release notes are complete and all intended assets are in place:

```sh
bun run release:commit
git push origin main
git push origin vX.Y.Z
```

`release:commit` validates the version and notes, stages only the release manifests, lockfile, notes, and that version's assets, then creates a release commit and annotated tag. It does not push anything. Pushing the tag triggers the production workflow; it will stop if the versioned notes still contain template comments or TODO/TBD markers.

The workflow validates that the solver Maven version matches the release tag, then builds `orbis-solver-<version>-runner` for each platform and stages it under the target-triple `solver-service` name expected by Tauri. It creates a draft release and attaches the signed updater metadata/installers plus the versioned assets. Review the generated assets, signatures, updater manifest, notes, and platform builds in GitHub before publishing the draft. Publishing the draft is the production rollout gate.

## Commit message convention

Use Conventional Commit subjects (`feat:`, `fix:`, `perf:`, `refactor:`, etc.). `!` or a `BREAKING CHANGE:` footer marks a breaking change in the generated draft. The release author must still verify completeness, migration guidance, and known issues before the draft is tagged.
