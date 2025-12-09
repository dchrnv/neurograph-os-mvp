# Legal Framework Checklist

**Status:** ✅ Complete (2025-12-09)

This checklist ensures all legal infrastructure is in place for NeuroGraph's dual licensing model.

---

## ✅ Documentation Created

- [x] **CLA.md** - Comprehensive Contributor License Agreement
  - Individual contributor provisions
  - Corporate contributor provisions
  - ML weights/datasets coverage
  - Patent grants
  - Irrevocability clause
  - Moral rights waiver
  - Warranty disclaimers

- [x] **DUAL_LICENSING.md** - Business model explanation
  - Open source vs. commercial comparison
  - Real-world use case examples
  - FAQ section
  - Pricing contact information
  - Similar projects reference

- [x] **CLA_INSTRUCTIONS.md** - Step-by-step signing guide
  - 3 signing methods (PR comment, issue, email)
  - Individual vs. corporate workflows
  - FAQ for common questions
  - Verification procedures

- [x] **CONTRIBUTORS.md** - Recognition template
  - Categories (core, code, docs, ML)
  - Release history
  - Claude Code acknowledgment
  - Statistics placeholders

- [x] **LICENSE-DATA** - CC BY-NC-SA 4.0 for data/models
  - Clear scope definition
  - Commercial licensing contact
  - Link to full legal text

---

## ✅ Existing Files Updated

- [x] **CONTRIBUTING.md**
  - CLA section added at top
  - Dual licensing explanation
  - Links to legal docs
  - Updated checklist with CLA requirement
  - Commercial licensing contact

- [x] **README.md**
  - Dual licensing section
  - Links to CLA.md and DUAL_LICENSING.md
  - Commercial contact information

---

## ⚠️ Action Items for Project Owner

### Before Accepting First External Contribution

- [ ] **Review CLA.md**
  - Verify all provisions match your intentions
  - Confirm jurisdiction (currently "[Specify Jurisdiction]")
  - Update email addresses (currently placeholders)
  - Consult with lawyer (optional but recommended)

- [ ] **Update Jurisdiction** in CLA.md
  - Line 275: `This Agreement shall be governed by and construed in accordance with the laws of **[Specify Jurisdiction]**`
  - Recommended: Your country/state (e.g., "Ukraine", "California, USA", "Germany")

- [ ] **Update Email Addresses**
  - CLA.md line 346: `**Email:** [your-email@example.com]`
  - CLA.md line 389: `**Email:** [your-email@example.com]`
  - Replace with: `dreeftwood@gmail.com` (already used elsewhere)

- [ ] **Create CLA Database**
  - Decide how to track CLA signatures:
    - **Option 1:** Manual tracking (spreadsheet)
    - **Option 2:** GitHub Issues labels
    - **Option 3:** CLA Assistant bot (https://github.com/cla-assistant/cla-assistant)
  - Recommended: Start with manual tracking, automate later

- [ ] **Set Up GitHub Labels**
  - `cla-signed` - Contributor has signed CLA
  - `cla-pending` - Waiting for CLA signature
  - `cla-not-required` - Bug report or non-code contribution

### Optional (Recommended)

- [ ] **Legal Review**
  - Have a lawyer review CLA.md (especially if planning significant commercial licensing)
  - Ensure CLA is enforceable in your jurisdiction
  - Verify compliance with local employment/IP laws

- [ ] **GitHub CLA Bot**
  - Set up CLA Assistant: https://github.com/cla-assistant/cla-assistant
  - Automatic CLA checking on PRs
  - Persistent signature storage
  - Cost: Free for open source

- [ ] **Commercial License Template**
  - Draft template for commercial licenses
  - Define pricing tiers (perpetual, subscription, usage-based)
  - Prepare SLA (Service Level Agreement) for enterprise customers

- [ ] **Contributor Rewards**
  - Consider recognizing top contributors:
    - Swag (t-shirts, stickers)
    - Public acknowledgment (blog posts, tweets)
    - Early access to commercial features
    - Revenue sharing for significant contributions (optional)

---

## 📋 Workflow for First Pull Request

When you receive your first external PR:

### 1. Check for CLA Signature

Look for this comment in the PR:
```
I have read and agree to the NeuroGraph Contributor License Agreement (CLA):
https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md

I confirm that I have the rights to submit this Contribution and grant the licenses described in the CLA.
```

### 2. If CLA Not Found

Add this comment to the PR:
```
Thank you for your contribution! 🎉

Before we can merge this PR, we need you to accept our Contributor License Agreement (CLA).

**Why?** NeuroGraph uses dual licensing (open source + commercial). The CLA allows us to:
- Keep the project open source (AGPLv3/CC BY-NC-SA)
- Fund development through commercial licenses
- Give you credit for your work

**How to sign:**

Please add this comment to this PR:

> I have read and agree to the NeuroGraph Contributor License Agreement (CLA):
> https://github.com/dchrnv/neurograph-os/blob/main/docs/legal/CLA.md
>
> I confirm that I have the rights to submit this Contribution and grant the licenses described in the CLA.

**Learn more:**
- [CLA Signing Guide](.github/CLA_INSTRUCTIONS.md)
- [Dual Licensing Explanation](docs/legal/DUAL_LICENSING.md)

Questions? Feel free to ask here or email dreeftwood@gmail.com
```

### 3. After CLA Signed

1. ✅ Add `cla-signed` label to PR
2. ✅ Record signature in CLA database (spreadsheet/issue)
3. ✅ Review code normally
4. ✅ Merge PR
5. ✅ Add contributor to CONTRIBUTORS.md
6. ✅ Thank them publicly! 🎉

### 4. Add to CONTRIBUTORS.md

After merging first PR:
```markdown
**[Contributor Name]** ([@github-username](https://github.com/username))
- Contributions: [Brief description]
- Notable PRs: #123
- CLA Signed: 2025-12-09
```

---

## 🚨 Red Flags

Watch out for these issues:

### ❌ No CLA Signature
- **Action:** Request CLA before merging
- **Never:** Merge without CLA (breaks dual licensing model)

### ❌ Contributor Claims Employer Owns Work
- **Action:** Request corporate CLA from employer
- **Never:** Accept without employer authorization

### ❌ Code Includes Third-Party GPL/LGPL Dependencies
- **Action:** Review license compatibility
- **AGPLv3 is compatible with:** GPL, LGPL (but not MIT/Apache in all cases)
- **Consult:** Lawyer if unsure

### ❌ ML Contribution from Unclear Source
- **Action:** Request proof of data rights
- **Example:** "Where did you get this dataset?"
- **Never:** Accept datasets without clear provenance

### ❌ Large Corporate Contribution Without CLA
- **Action:** Request corporate CLA before starting review
- **Risk:** Individual employee may not have authority

---

## 📊 Metrics to Track

### CLA Acceptance Rate
- Total PRs: X
- CLA signed: Y
- Acceptance rate: Y/X%

**Goal:** >90% acceptance rate

### Contributor Growth
- Total contributors: X
- CLAs signed: Y
- Corporate CLAs: Z

### Commercial Licensing
- Inquiries: X
- Conversions: Y
- Revenue: $Z

---

## 🔄 Maintenance Schedule

### Monthly
- [ ] Review CLA signatures (ensure database is up-to-date)
- [ ] Update CONTRIBUTORS.md with new contributors
- [ ] Check for unsigned PRs (add reminders)

### Quarterly
- [ ] Review CLA.md for any needed updates
- [ ] Update DUAL_LICENSING.md with new examples
- [ ] Analyze CLA acceptance rate

### Annually
- [ ] Legal review of CLA (if making major changes)
- [ ] Update jurisdiction/contact information if changed
- [ ] Review commercial licensing strategy

---

## 🆘 Support

### Questions About Legal Framework

**Contact:** Chernov Denys
- Email: dreeftwood@gmail.com
- GitHub: [@dchrnv](https://github.com/dchrnv)

### Legal Issues

If you're uncertain about:
- CLA enforceability
- Commercial licensing terms
- IP ownership disputes
- Corporate contributor agreements

**Recommendation:** Consult with a lawyer specializing in open source and IP law.

**Resources:**
- Software Freedom Law Center: https://www.softwarefreedom.org/
- Open Source Initiative: https://opensource.org/licenses
- Creative Commons: https://creativecommons.org/licenses/

---

## ✅ Final Checklist

Before going live with dual licensing:

- [x] CLA.md created and reviewed
- [x] DUAL_LICENSING.md explains business model
- [x] CLA_INSTRUCTIONS.md provides clear signing process
- [x] CONTRIBUTING.md updated with CLA requirements
- [x] README.md mentions dual licensing
- [x] CONTRIBUTORS.md template ready
- [x] LICENSE-DATA created for CC BY-NC-SA
- [ ] Jurisdiction specified in CLA.md ⚠️ **ACTION REQUIRED**
- [ ] Email placeholders replaced ⚠️ **ACTION REQUIRED**
- [ ] CLA tracking system chosen (manual/automated)
- [ ] GitHub labels created (cla-signed, cla-pending)
- [ ] First PR workflow documented (this checklist)

**Status:** 🟡 Nearly Complete (2 action items remaining)

---

## 🎉 You're Ready!

Once you've completed the action items above, your dual licensing framework is production-ready!

**Next Steps:**
1. Fix jurisdiction and email placeholders
2. Choose CLA tracking method
3. Create GitHub labels
4. Wait for first external PR
5. Follow workflow from this checklist

**Questions?** Email dreeftwood@gmail.com

---

*Last Updated: 2025-12-09 (v0.45.0)*
