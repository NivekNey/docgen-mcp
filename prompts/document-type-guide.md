# Document Type Guide

When helping users create professional documents, it's important to understand which document type is appropriate for their situation. This guide explains the differences between resumes, CVs, and cover letters, and when to use each.

## Quick Decision Tree

```
User needs job application materials?
│
├─ Are they applying in academia/research/medicine? → CV
├─ Do they need to introduce themselves for a specific role? → Cover Letter
└─ Do they need to summarize their professional background? → Resume
```

## Resume vs CV vs Cover Letter

### Resume
**What it is:** A concise (1-2 page) summary of your professional experience, skills, and education tailored to a specific job.

**When to use:**
- Applying for jobs in industry (tech, business, non-profit, etc.)
- Networking events or job fairs
- When you need a quick professional snapshot
- Any non-academic role in North America

**Key characteristics:**
- **Length:** 1 page (early career) to 2 pages (10+ years experience)
- **Focus:** Relevant experience for the target role
- **Customization:** Tailored for each application
- **Content:** Work experience, education, skills, projects
- **Style:** Concise, achievement-focused bullet points
- **Geography:** Standard in USA, Canada, and most industries globally

**Tool to use:** `generate_resume`

---

### CV (Curriculum Vitae)
**What it is:** A comprehensive document listing your entire academic and professional history, typically used in academic, research, or medical fields.

**When to use:**
- Applying for academic positions (professor, researcher, postdoc)
- Research grants and fellowships
- Medical residencies and faculty positions
- International applications (CV is standard outside North America)
- Applying to graduate programs (PhD, some Master's programs)

**Key characteristics:**
- **Length:** No limit (typically 3-10+ pages)
- **Focus:** Complete academic record
- **Customization:** Usually not customized (comprehensive is the goal)
- **Content:** Publications, research, teaching, grants, conferences, awards, education, work
- **Style:** Detailed, chronological, comprehensive
- **Geography:** Standard for academic roles globally; standard for ALL roles in Europe, Asia, Africa

**Current status:** Not yet implemented
**Note:** For now, users can use `generate_resume` with the Publications section for academic positions, but a full CV template is planned for future releases.

---

### Cover Letter
**What it is:** A one-page letter introducing yourself and explaining why you're interested in and qualified for a specific position at a specific company.

**When to use:**
- Accompanying a resume/CV for a job application
- Expressing interest in a company (even without an opening)
- Explaining career transitions or gaps
- Networking introductions
- Graduate school applications (sometimes called a "statement of purpose")

**Key characteristics:**
- **Length:** Always 1 page (3-4 paragraphs)
- **Focus:** Why you + why this company + why this role
- **Customization:** Must be highly customized for each application
- **Content:** Narrative connecting your experience to their needs
- **Style:** Professional but personal; tells a story
- **Purpose:** Showcase personality, motivation, and research

**Tool to use:** `generate_cover_letter`

---

## When to Create Both

### Resume + Cover Letter (Most Common)
**Use case:** Standard job application in industry

**Workflow:**
1. Create resume highlighting relevant experience
2. Create cover letter explaining why you're interested and qualified
3. Submit both together

**Example:** Applying for a Software Engineer role at a tech company

---

### CV + Cover Letter
**Use case:** Academic or research position

**Workflow:**
1. Create CV with comprehensive publication and research history
2. Create cover letter explaining research interests and fit
3. Submit both together

**Example:** Applying for an Assistant Professor position

---

### Resume Only
**Use case:** Quick applications, job fairs, networking

**When it's appropriate:**
- Company doesn't request a cover letter
- Internal referral (resume forwarded by employee)
- Networking events where you're handing out materials
- Job fair or recruiter submission
- LinkedIn/online profile updates

**Note:** Even when "optional," a cover letter usually helps unless you have a strong referral.

---

### Cover Letter Only
**Use case:** Very rare; usually informal outreach

**When it's appropriate:**
- Cold email to a hiring manager (not a formal application)
- Networking introduction via shared connection
- Expressing interest when no position is posted

**Note:** This is uncommon. Most formal applications require resume/CV.

---

## Common Confusions

### "Isn't a CV just a longer resume?"
**Not quite.** In North America:
- **Resume** = concise, tailored, for industry jobs
- **CV** = comprehensive, academic, for research/teaching roles

**Outside North America:**
- "CV" is used to mean what Americans call a "resume"
- They're typically 2 pages and function like US resumes
- When in doubt, check job posting language and country norms

### "Do I always need a cover letter?"
**Best practice:** Yes, unless the application explicitly says not to include one.

**Reality:**
- Some companies don't read cover letters
- Some recruiters skip them
- BUT: A good cover letter can differentiate you when it IS read
- It's insurance — costs you 20 minutes, could win you the job

**When you can skip:**
- Application form explicitly says "no cover letter"
- Quick-apply systems (LinkedIn Easy Apply)
- Internal referrals where resume is forwarded directly
- Recruiter submissions (they usually don't forward cover letters)

### "Can I use the same resume for every job?"
**No.** Each resume should be tailored:
- Reorder sections to highlight relevant experience first
- Adjust bullet points to match job requirements
- Update skills section to mirror job posting keywords
- Modify summary to speak to the specific role

**Exception:** Networking resumes can be more general.

---

## Workflow Guidance for AI Agents

When a user asks for help with job application materials:

### Step 1: Determine the Need
Ask clarifying questions:
- "What type of position are you applying for?" (industry vs academic)
- "Where is the job located?" (US vs Europe vs other)
- "Do you have a specific job posting?" (for customization)

### Step 2: Recommend Document Types
Based on answers:
- **Industry job in US/Canada** → Resume + Cover Letter
- **Academic job anywhere** → CV + Cover Letter (note: use Resume tool with Publications for now)
- **Industry job in Europe/Asia** → CV (use Resume tool) + Cover Letter
- **Networking/general** → Resume only

### Step 3: Gather Information
- For **resume**: Use `get_resume_best_practices` and `get_resume_schema`
- For **cover letter**: Use `get_cover_letter_best_practices` and `get_cover_letter_schema`

### Step 4: Generate Documents
- Call `generate_resume` and/or `generate_cover_letter` as appropriate
- Provide file paths/URLs to user
- Remind user to review and customize further

---

## Summary Table

| Document | Length | Purpose | Customization | When to Use |
|----------|--------|---------|---------------|-------------|
| **Resume** | 1-2 pages | Highlight relevant experience | High (per job) | Industry jobs, North America |
| **CV** | 3-10+ pages | Comprehensive record | Low (mostly static) | Academic/research roles, international |
| **Cover Letter** | 1 page | Explain interest & fit | Very high (per job) | Accompany resume/CV |

---

## Tips for AI Agents

1. **Default to Resume + Cover Letter for US industry jobs** — this is the most common case
2. **Ask about geography and field** — CV means different things in different contexts
3. **Always offer cover letter** — even when optional, it usually helps
4. **Explain the differences** — many users confuse CV and resume
5. **Tailor the advice** — what works for a software engineer differs from an academic
6. **Remind users to customize** — especially resumes and cover letters

---

**Remember:** The best document depends on the user's specific situation. When in doubt, ask clarifying questions about the role, location, and industry before recommending a document type.
