# Release Instructions

## Creating a Release on GitHub

### Step 1: Update Version (if needed)

The version is already set to `0.1.0` in `Cargo.toml`. For future releases, update:
- `Cargo.toml` - `version = "X.Y.Z"`
- `CHANGELOG.md` - Add new version section
- `README.md` - Update version badge

### Step 2: Commit All Changes

```bash
git add .
git commit -m "Release v0.1.0"
```

### Step 3: Create Git Tag

```bash
# Create annotated tag (recommended)
git tag -a v0.1.0 -m "Release version 0.1.0"

# Or create lightweight tag
git tag v0.1.0
```

### Step 4: Push to GitHub

```bash
# Push commits
git push origin main

# Push tags
git push origin v0.1.0
```

### Step 5: Create GitHub Release

1. Go to your repository on GitHub
2. Click on "Releases" (right sidebar)
3. Click "Create a new release"
4. Select tag: `v0.1.0`
5. Release title: `v0.1.0 - Initial Release`
6. Description: Copy content from `RELEASE.md` or `CHANGELOG.md`
7. Check "Set as the latest release"
8. Click "Publish release"

### Alternative: Using GitHub CLI

```bash
# Install gh CLI if not installed
# Then create release:
gh release create v0.1.0 \
  --title "v0.1.0 - Initial Release" \
  --notes-file RELEASE.md
```

## Version Numbering (Semantic Versioning)

Follow [SemVer](https://semver.org/): `MAJOR.MINOR.PATCH`

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.2.0): New features, backward compatible
- **PATCH** (0.1.1): Bug fixes, backward compatible

## Pre-release Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] CHANGELOG.md updated
- [ ] Version updated in Cargo.toml
- [ ] README.md updated (if needed)
- [ ] Documentation is complete
