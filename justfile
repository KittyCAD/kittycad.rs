default:
    @just --list

# Create a git tag to release openapitor-runtime independently.
tag-runtime:
    #!/usr/bin/env bash
    set -euo pipefail
    package_id=$(cargo pkgid --offline --locked --manifest-path openapitor-runtime/Cargo.toml)
    tag="openapitor-runtime-v${package_id##*#}"
    git tag -sa "$tag" -m "$tag"
    echo "Run git push origin $tag to push your new tag to GitHub and release openapitor-runtime."
