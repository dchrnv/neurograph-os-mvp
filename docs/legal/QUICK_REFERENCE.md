# CLA Quick Reference Guide

**For:** Project Owner (Chernov Denys)
**Purpose:** Quick lookup for CLA management

---

## 🚨 First PR Arrives - What to Do?

### Step 1: Check for CLA Comment (10 seconds)

Look for this text in PR comments:
```
I have read and agree to the NeuroGraph Contributor License Agreement (CLA)
```

**Found it?** → Go to Step 2
**Not found?** → Go to Step 3

---

### Step 2: CLA Found ✅

1. Add label: `cla-signed`
2. Record in CLA database (optional: spreadsheet)
3. Review code normally
4. Merge when ready
5. Add contributor to CONTRIBUTORS.md
6. Thank them publicly! 🎉

**Done!** PR can proceed.

---

### Step 3: CLA Not Found ⚠️

1. Add label: `cla-pending`
2. Copy-paste this comment:

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

3. Wait for contributor to add CLA comment
4. When they sign → Go to Step 2

---

## 🏷️ GitHub Labels

### Quick Add

In PR, click "Labels" → Select:

- `cla-signed` (green) - CLA accepted ✅
- `cla-pending` (yellow) - Waiting for CLA ⚠️
- `cla-not-required` (gray) - Trivial change ℹ️

### When to Use Each

**cla-signed:**
- Contributor added CLA comment
- CLA recorded in database
- Ready to review/merge

**cla-pending:**
- PR submitted without CLA
- Waiting for signature
- Code review on hold

**cla-not-required:**
- Typo fixes (1-2 characters)
- Bug reports (no code contribution)
- Documentation formatting only
- Markdown fixes

---

## 🤔 Common Questions

### Q: Contributor says "My employer owns my work"

**A:** Request Corporate CLA via email:

```
Subject: NeuroGraph Corporate CLA Required

Hi [Name],

Thank you for your interest in contributing!

Since your employer owns your work, we'll need a Corporate CLA signed by an authorized representative of your company.

Please have someone with signing authority (e.g., Engineering Manager, Legal, CTO) email dreeftwood@gmail.com with:

Subject: NeuroGraph Corporate CLA Agreement - [Company Name]

Body:
---
Organization Name: [Company Name]
Authorized Signatory: [Name, Title]
GitHub Username(s): @your-username
Date: [YYYY-MM-DD]

On behalf of [Company Name], I agree to the terms of the NeuroGraph Corporate CLA:
https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md

I confirm that I have the authority to bind [Company Name] to this agreement.

Authorized Signature: ___________________________
Name: [Full Name]
Title: [Job Title]
Email: [Corporate Email]
---

We'll respond within 2 business days.

Full CLA: https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md

Thanks!
Chernov Denys
```

### Q: Contributor asks "Why do you need CLA?"

**A:** Link to: [docs/legal/DUAL_LICENSING.md](DUAL_LICENSING.md#why-do-we-need-a-cla)

**TL;DR:** Dual licensing (free open source + paid commercial) requires CLA to legally sell commercial licenses.

### Q: "Can I revoke my CLA later?"

**A:** No, CLA is irrevocable (by design, for project stability).

### Q: "Will I get credit?"

**A:** Yes! You'll be in:
- GitHub Contributors (automatic)
- CONTRIBUTORS.md (manual)
- Release notes (for significant contributions)

### Q: Contributor contributed before CLA existed

**A:** Request retroactive CLA acceptance:

```markdown
Hi @username,

Thank you for your past contributions! We've recently implemented a CLA to enable dual licensing.

Could you please sign the CLA for your existing contributions by adding this comment:

> I have read and agree to the NeuroGraph Contributor License Agreement (CLA):
> https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md
>
> I confirm that I have the rights to submit this Contribution and grant the licenses described in the CLA.
>
> This acceptance covers all my past and future contributions to NeuroGraph.

Thank you!
```

---

## 📊 CLA Tracking

### Manual Tracking (Spreadsheet)

Create a Google Sheet with columns:

| Date | GitHub Username | Name | Email | PR # | CLA Type | Status |
|------|-----------------|------|-------|------|----------|--------|
| 2025-12-09 | @alice | Alice J. | alice@example.com | #42 | Individual | Signed |
| 2025-12-10 | @bob | Bob Corp | bob@company.com | #43 | Corporate | Pending |

### Automated Tracking (CLA Assistant)

1. Go to: https://cla-assistant.io/
2. Click "Configure CLA"
3. Link to: https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md
4. Bot handles everything automatically

**Recommended:** Start manual, automate later.

---

## 🚩 Red Flags

### ❌ Third-Party Code Included

**Symptom:** PR includes code from other projects (GPL, LGPL, proprietary)

**Action:**
1. Ask: "Where did this code come from?"
2. Check license compatibility
3. If incompatible → Request removal or rewrite
4. If compatible → Ensure attribution

### ❌ Dataset Without Provenance

**Symptom:** PR includes large dataset without source information

**Action:**
1. Ask: "Where did you get this data?"
2. Request proof of data rights
3. If scraped/questionable → Reject
4. If legitimate → Require documentation

### ❌ Large Corporate Contribution Without Corporate CLA

**Symptom:** @bigcorp-employee submits major feature (100+ lines)

**Action:**
1. Request Corporate CLA immediately
2. Do NOT review code until CLA signed
3. Risk: Individual may not have authority

### ❌ Contributor Refuses to Sign

**Symptom:** Contributor says "I don't want to sign CLA"

**Action:**
1. Explain benefits (they keep copyright)
2. Link to DUAL_LICENSING.md
3. If still refuses → Close PR with thanks
4. Alternative: They can fork and maintain separately

---

## 📞 Quick Contacts

**Legal Questions:** Consult lawyer (recommended for first commercial license)

**CLA Questions:** Point contributors to:
- [.github/CLA_INSTRUCTIONS.md](.github/CLA_INSTRUCTIONS.md)
- dreeftwood@gmail.com

**Technical Questions:** CONTRIBUTING.md

---

## 📝 Templates

### Add Contributor to CONTRIBUTORS.md

```markdown
**[Full Name]** ([@username](https://github.com/username))
- Contributions: [Brief description]
- Notable PRs: #123, #456
- CLA Signed: 2025-12-09
```

### Thank Contributor (After Merge)

```markdown
Thank you @username! 🎉

Your contribution has been merged and will be included in the next release.

You've been added to [CONTRIBUTORS.md](CONTRIBUTORS.md).

We appreciate your support of NeuroGraph! 🧠✨
```

---

## ⏱️ Time Estimates

- **Check for CLA:** 10 seconds
- **Add label:** 5 seconds
- **Request CLA (if missing):** 30 seconds (copy-paste)
- **Record in database:** 1 minute
- **Add to CONTRIBUTORS.md:** 2 minutes

**Total per PR:** ~4 minutes (after CLA signed)

---

## 🔗 Full Documentation

**Detailed guides:**
- [CLA.md](CLA.md) - Full legal text
- [DUAL_LICENSING.md](DUAL_LICENSING.md) - Business model explanation
- [CLA_INSTRUCTIONS.md](.github/CLA_INSTRUCTIONS.md) - How to sign
- [CHECKLIST.md](CHECKLIST.md) - Complete maintenance guide
- [GITHUB_LABELS_SETUP.md](GITHUB_LABELS_SETUP.md) - Label creation guide

**This guide:** Quick reference only (bookmark this page!)

---

*Last Updated: 2025-12-09*
