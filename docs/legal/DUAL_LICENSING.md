# NeuroGraph Dual Licensing Model

**Last Updated:** December 9, 2025
**Version:** 1.0

---

## Overview

NeuroGraph uses a **dual licensing model** to balance open-source community access with sustainable commercial development. This document explains how the licensing works, why it exists, and how it benefits both free and commercial users.

---

## TL;DR

- **Free (Open Source)**: Use under AGPLv3 (code) and CC BY-NC-SA 4.0 (data/models)
- **Commercial**: Purchase proprietary license without AGPL/CC restrictions
- **Contributors**: Must sign CLA to enable dual licensing

---

## License Breakdown

### 1. Open Source License (Default)

#### Code: GNU Affero General Public License v3.0 (AGPLv3)

**You CAN:**
- ✅ Use NeuroGraph for any purpose (including commercial)
- ✅ Study and modify the source code
- ✅ Distribute copies and modifications
- ✅ Run NeuroGraph as a service (SaaS)

**You MUST:**
- 📋 Provide source code to users (including over network)
- 📋 License your modifications under AGPLv3
- 📋 Include copyright and license notices
- 📋 Disclose all changes you make

**Why AGPLv3?**
- Closes the "SaaS loophole" of GPL (network use = distribution)
- Ensures derivative works remain open source
- Protects community contributions from being closed off

#### Data/Models: Creative Commons Attribution-NonCommercial-ShareAlike 4.0 (CC BY-NC-SA 4.0)

**You CAN:**
- ✅ Share and redistribute CDNA, datasets, model weights
- ✅ Adapt and modify for non-commercial purposes
- ✅ Use for research, education, personal projects

**You CANNOT:**
- ❌ Use for commercial purposes without a commercial license
- ❌ Remove attribution
- ❌ Apply more restrictive licenses

**You MUST:**
- 📋 Give appropriate credit (attribution)
- 📋 Share modifications under the same CC BY-NC-SA 4.0 license
- 📋 Indicate if changes were made

**Why CC BY-NC-SA 4.0?**
- Prevents commercial exploitation of community-contributed data/models without compensation
- Ensures improvements to data/models benefit the community
- Allows free use for research and education

---

### 2. Commercial License (Paid)

For organizations that:
- Want to use NeuroGraph in proprietary products
- Cannot comply with AGPLv3 source disclosure requirements
- Need to use models/data commercially without NC/SA restrictions
- Want to integrate NeuroGraph into closed-source software
- Require custom terms or enterprise support

**Benefits:**
- ✅ No source code disclosure required (closed-source allowed)
- ✅ No copyleft obligations (keep modifications private)
- ✅ Commercial use of data/models without restrictions
- ✅ Remove CC BY-NC-SA limitations (non-commercial, share-alike)
- ✅ Sublicensing rights for your products
- ✅ Patent protection and indemnification
- ✅ Priority support and custom development
- ✅ Flexible deployment (cloud, on-premise, embedded)

**Pricing:** Contact dreeftwood@gmail.com for commercial licensing inquiries.

---

## How Dual Licensing Works

### Open Source Path (Free)

```
┌──────────────┐
│  User/Company │
└──────┬───────┘
       │
       ▼
┌────────────────────────┐
│ Uses NeuroGraph under  │
│ AGPLv3 + CC BY-NC-SA   │
└────────┬───────────────┘
         │
         ▼
┌──────────────────────────────┐
│ MUST: Disclose source code   │
│ MUST: License modifications  │
│       under AGPL             │
│ MUST: Share data/model mods  │
│       under CC BY-NC-SA      │
└──────────────────────────────┘
```

### Commercial Path (Paid)

```
┌──────────────┐
│  Company     │
└──────┬───────┘
       │
       ▼
┌────────────────────────┐
│ Purchases commercial   │
│ license from owner     │
└────────┬───────────────┘
         │
         ▼
┌──────────────────────────────┐
│ ✅ No source disclosure      │
│ ✅ Keep modifications private│
│ ✅ Use models commercially   │
│ ✅ Proprietary integrations  │
└──────────────────────────────┘
```

---

## Why This Model?

### Problem: Tragedy of the Commons

**Scenario without dual licensing:**

1. Developer spends years building NeuroGraph (open source)
2. BigCorp takes code, builds proprietary product, makes $$$
3. Developer gets: ⭐ stars on GitHub
4. Community gets: No funding for improvements
5. Project stagnates or gets abandoned

**Result:** Lose-lose for everyone except BigCorp.

### Solution: Dual Licensing

**With dual licensing:**

1. Developer builds NeuroGraph (open source under AGPL)
2. SmallUser uses for free (AGPL compliance)
3. BigCorp pays for commercial license → $$$
4. Revenue funds:
   - Continued development
   - Bug fixes and security patches
   - New features
   - Documentation improvements
   - Community support
5. Everyone benefits!

**Result:** Sustainable open-source development.

---

## Contributor License Agreement (CLA)

### Why Do We Need a CLA?

**Problem:** Without a CLA, contributors retain exclusive copyright over their code. The project owner cannot sell commercial licenses because they don't have the right to relicense contributions.

**Example:**

```
Alice contributes a feature to NeuroGraph (no CLA).
BigCorp wants to buy a commercial license.
→ Problem: Owner can only license their own code, not Alice's contribution.
→ BigCorp cannot use the feature Alice wrote.
→ Commercial licensing is impossible.
```

**Solution:** CLA grants the project owner dual licensing rights:

1. **Public License** (AGPL/CC): Community can use freely
2. **Commercial License**: Owner can sell proprietary licenses

### What Does the CLA Do?

**You (Contributor):**
- ✅ Keep copyright to your contributions
- ✅ Can use your contributions freely
- ✅ Get credit in CONTRIBUTORS.md

**Project Owner:**
- ✅ Can distribute your contribution under AGPL/CC (open source)
- ✅ Can distribute your contribution under proprietary licenses (commercial)
- ✅ Can sublicense to third parties
- ✅ Can use in closed-source commercial products

**Key Point:** You're not giving up rights—you're granting **additional** rights to the project owner to enable dual licensing.

---

## Real-World Examples

### Example 1: Research Lab (Free Use)

**Scenario:**
- University lab uses NeuroGraph for cognitive science research
- Publishes papers with NeuroGraph-generated results
- Modifies CDNA for custom experiments

**License:** Open Source (AGPL + CC BY-NC-SA)

**Requirements:**
- ✅ Cite NeuroGraph in papers (attribution)
- ✅ Share CDNA modifications if distributing (CC BY-NC-SA)
- ✅ Share code modifications if distributing system (AGPL)
- ✅ Use is non-commercial (research)

**Cost:** Free

---

### Example 2: Startup Building Product (Free Use with Compliance)

**Scenario:**
- Startup builds open-source cognitive assistant using NeuroGraph
- Offers SaaS product (free + premium tiers)
- All code is open source

**License:** Open Source (AGPL + CC BY-NC-SA)

**Requirements:**
- ✅ Provide source code to all users (including SaaS users)
- ✅ License entire product under AGPL
- ✅ Disclose all modifications publicly
- ✅ Offer data/models under CC BY-NC-SA (non-commercial data use)

**Cost:** Free

**Benefit:** Attracts open-source community, gets contributions

---

### Example 3: Enterprise Using NeuroGraph Internally (Gray Area)

**Scenario:**
- Enterprise uses NeuroGraph internally for employee productivity tools
- Not offering as a service to customers
- Some custom modifications

**License:** Arguably Free (AGPL)

**Analysis:**
- AGPL only triggers on **distribution** or **network use as service**
- Internal use without external access = no source disclosure required
- However, if employees access over network, AGPL may apply

**Recommendation:**
- If uncertain, get commercial license for peace of mind
- If definitely not distributing/serving externally, free use is OK

**Cost:** Potentially free, but commercial license recommended for clarity

---

### Example 4: BigCorp Building Proprietary Product (Paid)

**Scenario:**
- BigCorp builds closed-source enterprise platform
- Integrates NeuroGraph as cognitive reasoning engine
- Sells licenses to Fortune 500 companies
- Wants to keep modifications secret (trade secrets)

**License:** Commercial (Proprietary)

**Benefits:**
- ✅ No source disclosure required
- ✅ Keep all modifications private
- ✅ Use pre-trained models commercially (no CC BY-NC-SA restrictions)
- ✅ Sublicense to enterprise customers
- ✅ Patent protection
- ✅ Priority support from NeuroGraph team

**Cost:** Custom pricing (contact owner)

**Negotiable Terms:**
- Perpetual vs. subscription licensing
- Number of deployments/users
- Revenue sharing vs. flat fee
- Custom development support
- SLA guarantees

---

### Example 5: AI Research Lab (Free Use for Models)

**Scenario:**
- Lab trains large cognitive models on NeuroGraph architecture
- Publishes model weights for research community
- Uses models for non-commercial academic research

**License:** Open Source (CC BY-NC-SA for model weights)

**Requirements:**
- ✅ Release model weights under CC BY-NC-SA
- ✅ Attribute NeuroGraph in papers and model cards
- ✅ Share training code if distributing models (AGPL for code)
- ✅ Non-commercial use only

**Cost:** Free

---

### Example 6: ML Startup (Paid for Models)

**Scenario:**
- Startup trains domain-specific cognitive models on NeuroGraph
- Offers inference API (commercial service)
- Wants to keep model weights proprietary

**License:** Commercial (for model weights and inference service)

**Benefits:**
- ✅ Use trained models commercially without CC BY-NC restrictions
- ✅ No requirement to share model weights
- ✅ Offer proprietary inference API
- ✅ Sublicense model access to customers

**Cost:** Custom pricing based on revenue/usage

---

## Machine Learning Assets: Special Considerations

### Datasets

**Open Source (CC BY-NC-SA):**
- ✅ Research and education
- ✅ Non-commercial training
- ✅ Must share modifications

**Commercial License:**
- ✅ Commercial training data
- ✅ Proprietary datasets (keep private)
- ✅ No share-alike requirements

### Model Weights

**Open Source (CC BY-NC-SA):**
- ✅ Academic research
- ✅ Personal projects
- ✅ Non-commercial inference
- ✅ Must share fine-tuned weights

**Commercial License:**
- ✅ Commercial inference services (SaaS)
- ✅ Proprietary model weights
- ✅ Enterprise deployments
- ✅ Keep fine-tuned weights private

### Pre-trained Models

If you train a model on NeuroGraph and want to:
- **Share for free:** Use CC BY-NC-SA (non-commercial only)
- **Sell or commercialize:** Need commercial license from NeuroGraph owner

---

## Comparison Table

| Use Case | AGPL/CC BY-NC-SA | Commercial License |
|----------|------------------|-------------------|
| **Open-source project** | ✅ Free | Not needed |
| **Academic research** | ✅ Free | Not needed |
| **Internal corporate use (no external access)** | ✅ Free* | Optional |
| **SaaS with open-source code** | ✅ Free | Not needed |
| **Proprietary product (closed-source)** | ❌ Not allowed | ✅ Required |
| **Commercial model inference** | ❌ Not allowed | ✅ Required |
| **Keep modifications secret** | ❌ Not allowed | ✅ Required |
| **Sublicense to customers** | ❌ Limited | ✅ Full rights |
| **Patent protection** | ⚠️ Limited | ✅ Included |

*Consult with owner if uncertain

---

## How to Get a Commercial License

### Step 1: Contact Us

**Email:** dreeftwood@gmail.com
**Subject:** NeuroGraph Commercial License Inquiry

**Include:**
- Company name and size
- Intended use case
- Deployment scope (users, instances, geography)
- Timeline and budget

### Step 2: Consultation

We'll schedule a call to:
- Understand your requirements
- Explain licensing options
- Discuss custom terms if needed

### Step 3: Proposal

You'll receive a custom licensing proposal with:
- Pricing (perpetual or subscription)
- Scope of license (products, users, geography)
- Support options (SLA, priority support)
- Payment terms

### Step 4: Agreement

Sign the commercial license agreement and make payment.

### Step 5: Deployment

Receive license key and documentation. Deploy NeuroGraph in your proprietary environment.

---

## FAQ

### Q: Can I use NeuroGraph for free in my startup?

**A:** Yes, if:
- Your product is open-source (AGPL for code, CC BY-NC-SA for models)
- You provide source code to all users
- You don't use models/data commercially (or share under CC BY-NC-SA)

**A:** No (need commercial license) if:
- You want to keep code or models proprietary
- You want to use models commercially without restrictions

### Q: What if I'm not sure if my use case requires a commercial license?

**A:** Contact us! We're happy to discuss your scenario and clarify whether you need a commercial license. Gray areas can often be resolved with a conversation.

### Q: Can I try NeuroGraph before buying a commercial license?

**A:** Yes! Use it under AGPL/CC for evaluation. Once you decide to go commercial (proprietary), purchase a license before deploying.

### Q: What if I contributed code before the CLA existed?

**A:** We will retroactively request CLA acceptance for your past contributions. If you decline, we will remove your contributions or offer alternative arrangements.

### Q: Can I negotiate custom commercial license terms?

**A:** Yes! Commercial licenses are negotiable based on your needs (pricing, scope, support, revenue sharing, etc.).

### Q: Does the CLA mean you can steal my contributions?

**A:** No! You retain copyright. The CLA only grants **additional rights** to enable dual licensing. You can still use your contributions freely, and you'll be credited.

### Q: Why not use MIT/Apache license (more permissive)?

**A:** MIT/Apache allow anyone to take the code, close it off, and commercialize without giving back. AGPL ensures derivative works remain open source, while dual licensing enables sustainable funding.

### Q: Can I fork NeuroGraph and create a competing commercial product?

**A:** Under AGPL: Yes, but your fork must also be AGPL (source code disclosure required). You cannot close off the source.

**Practical consideration:** Contributors signed CLAs with the original project, so you won't have rights to create proprietary versions. You'd be limited to AGPL distributions.

### Q: What about ML model licensing for fine-tuned models?

**A:** If you fine-tune a model based on NeuroGraph:
- **Open Source:** Share under CC BY-NC-SA (non-commercial)
- **Commercial:** Need commercial license to keep weights private or use commercially

### Q: Can I contribute datasets/models without signing the CLA?

**A:** No. All contributions (code, data, models) require CLA acceptance to enable dual licensing.

---

## Similar Projects Using Dual Licensing

NeuroGraph is in good company! Many successful open-source projects use dual licensing:

### Software

- **MySQL** (Oracle): GPL + Commercial
- **Qt** (The Qt Company): LGPL + Commercial
- **MongoDB** (MongoDB Inc.): SSPL + Commercial
- **GitLab** (GitLab Inc.): MIT + Commercial (Enterprise Edition)
- **Sentry** (Functional Software): BSL + Commercial
- **Elastic** (Elastic NV): SSPL + Commercial

### Machine Learning

- **Hugging Face Transformers**: Apache 2.0 (permissive) + Commercial support
- **OpenAI models**: Proprietary (no open-source version, but API access)
- **Anthropic Claude**: Proprietary with API access
- **Stability AI**: CreativeML Open RAIL++-M + Commercial licensing for enterprise

**NeuroGraph Approach:** Combines strict copyleft (AGPL) for code + non-commercial (CC BY-NC-SA) for data/models with flexible commercial licensing.

---

## Version History

- **v1.0** (2025-12-09): Initial dual licensing documentation

---

## Contact

**Questions about licensing?**

**Project Owner:** Chernov Denys
**Email:** dreeftwood@gmail.com
**GitHub:** [@dchrnv](https://github.com/dchrnv)
**Website:** https://github.com/dchrnv/neurograph-os

---

**Thank you for supporting NeuroGraph!**

Whether you're using it for free under open-source terms or purchasing a commercial license, your support helps build a sustainable cognitive architecture for the future.
