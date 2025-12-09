# GitHub Labels Setup for CLA Tracking

This guide helps you create GitHub labels for tracking CLA signatures.

---

## Required Labels

You need to create 3 labels for CLA tracking:

### 1. `cla-signed` (Green)
- **Color:** `#0E8A16` (green)
- **Description:** "Contributor has signed the CLA"
- **Usage:** Add this label when contributor signs CLA

### 2. `cla-pending` (Yellow)
- **Color:** `#FBCA04` (yellow)
- **Description:** "Waiting for CLA signature"
- **Usage:** Add this when PR needs CLA signature

### 3. `cla-not-required` (Gray)
- **Color:** `#D4C5F9` (light purple/gray)
- **Description:** "CLA not required (bug report, documentation typo, etc.)"
- **Usage:** Use for trivial changes that don't require CLA

---

## Method 1: Manual Creation (Web UI)

### Step 1: Go to Labels Page

Navigate to: `https://github.com/dchrnv/neurograph-os/labels`

Or:
1. Open your repository on GitHub
2. Click "Issues" tab
3. Click "Labels" button (next to Milestones)

### Step 2: Create Each Label

For each label, click **"New label"** and enter:

#### Label 1: cla-signed
```
Name: cla-signed
Description: Contributor has signed the CLA
Color: #0E8A16 (or choose green from palette)
```

#### Label 2: cla-pending
```
Name: cla-pending
Description: Waiting for CLA signature
Color: #FBCA04 (or choose yellow from palette)
```

#### Label 3: cla-not-required
```
Name: cla-not-required
Description: CLA not required (bug report, documentation typo, etc.)
Color: #D4C5F9 (or choose light purple from palette)
```

---

## Method 2: GitHub CLI (if installed)

If you have GitHub CLI (`gh`) installed:

```bash
# Install gh (if needed)
# Ubuntu/Debian: sudo apt install gh
# macOS: brew install gh
# Windows: winget install GitHub.cli

# Login
gh auth login

# Create labels
gh label create "cla-signed" \
  --description "Contributor has signed the CLA" \
  --color "0E8A16" \
  --repo dchrnv/neurograph-os

gh label create "cla-pending" \
  --description "Waiting for CLA signature" \
  --color "FBCA04" \
  --repo dchrnv/neurograph-os

gh label create "cla-not-required" \
  --description "CLA not required (bug report, documentation typo, etc.)" \
  --color "D4C5F9" \
  --repo dchrnv/neurograph-os
```

---

## Method 3: GitHub API (automated)

Create a script to automate label creation:

```bash
#!/bin/bash
# save as: scripts/create_cla_labels.sh

REPO_OWNER="dchrnv"
REPO_NAME="neurograph-os"
GITHUB_TOKEN="your_personal_access_token"  # Replace with your token

# Function to create label
create_label() {
  local name=$1
  local description=$2
  local color=$3

  curl -X POST \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/labels \
    -d "{\"name\":\"$name\",\"description\":\"$description\",\"color\":\"$color\"}"

  echo "" # newline
}

# Create labels
create_label "cla-signed" "Contributor has signed the CLA" "0E8A16"
create_label "cla-pending" "Waiting for CLA signature" "FBCA04"
create_label "cla-not-required" "CLA not required (bug report, documentation typo, etc.)" "D4C5F9"

echo "Labels created successfully!"
```

**To get GitHub token:**
1. Go to https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Select scopes: `public_repo` (or `repo` for private repos)
4. Copy token and replace in script

---

## Usage in Pull Requests

### When a PR Arrives:

#### 1. Check for CLA Comment

Look for this text in PR comments:
```
I have read and agree to the NeuroGraph Contributor License Agreement (CLA)
```

#### 2. Add Appropriate Label

**If CLA found:**
- Add label: `cla-signed` ✅
- Proceed with code review
- Merge when ready

**If CLA NOT found:**
- Add label: `cla-pending` ⚠️
- Add this comment (use template below)
- Wait for CLA signature

**If CLA not required:**
- Add label: `cla-not-required` ℹ️
- Examples: typo fixes, bug reports without code, documentation formatting

---

## Comment Template for Missing CLA

Use this when requesting CLA signature:

```markdown
Thank you for your contribution! 🎉

Before we can merge this PR, we need you to accept our **Contributor License Agreement (CLA)**.

### Why?

NeuroGraph uses dual licensing (open source + commercial). The CLA allows us to:
- Keep the project open source (AGPLv3/CC BY-NC-SA)
- Fund development through commercial licenses
- Give you credit for your work

### How to sign:

Please add this exact comment to this PR:

> I have read and agree to the NeuroGraph Contributor License Agreement (CLA):
> https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md
>
> I confirm that I have the rights to submit this Contribution and grant the licenses described in the CLA.

### Learn more:

- [CLA Signing Guide](.github/CLA_INSTRUCTIONS.md)
- [Dual Licensing Explanation](docs/legal/DUAL_LICENSING.md)
- [Full CLA Text](docs/legal/CLA.md)

Questions? Email dreeftwood@gmail.com or ask here!
```

---

## Automation (Optional)

### GitHub Actions Workflow

Create `.github/workflows/cla-check.yml` to automate CLA checking:

```yaml
name: CLA Check

on:
  pull_request_target:
    types: [opened, synchronize, reopened]
  issue_comment:
    types: [created]

jobs:
  cla-check:
    runs-on: ubuntu-latest
    steps:
      - name: Check CLA
        uses: contributor-assistant/github-action@v2.3.0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          path-to-signatures: '.github/cla-signatures.json'
          path-to-document: 'https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md'
          branch: 'main'
          allowlist: 'dependabot,renovate'

      - name: Add label on success
        if: success()
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.issues.addLabels({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              labels: ['cla-signed']
            })
```

**Note:** This requires CLA Assistant GitHub App or similar automation.

---

## Alternative: CLA Assistant Bot

**Easiest option:** Use CLA Assistant bot (free for open source)

1. Go to: https://cla-assistant.io/
2. Click "Configure CLA"
3. Grant access to your repository
4. Point to your CLA: `https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md`
5. Bot will automatically comment on PRs and check for signatures

**Benefits:**
- ✅ Automatic CLA checking
- ✅ Persistent signature storage
- ✅ No manual tracking needed
- ✅ Free for open source

**Setup time:** ~5 minutes

---

## Verification

After creating labels, verify:

1. Go to: `https://github.com/dchrnv/neurograph-os/labels`
2. You should see:
   - `cla-signed` (green)
   - `cla-pending` (yellow)
   - `cla-not-required` (light purple)

---

## Troubleshooting

### Label already exists?

If you see "Label already exists" error:
1. Go to labels page
2. Edit existing label
3. Update color and description to match above

### Can't create labels?

Check permissions:
- You need **write access** to the repository
- GitHub token needs `public_repo` or `repo` scope

### Wrong color?

Colors are hex codes without `#`:
- Green: `0E8A16`
- Yellow: `FBCA04`
- Purple: `D4C5F9`

---

## Next Steps

After creating labels:

1. ✅ Test labels on a test issue/PR
2. ✅ Save the comment template somewhere accessible
3. ✅ (Optional) Set up CLA Assistant bot for automation
4. ✅ Wait for first external PR!

---

**Questions?**

- Email: dreeftwood@gmail.com
- See: [docs/legal/CHECKLIST.md](CHECKLIST.md) for full CLA workflow

---

*Last Updated: 2025-12-09*
