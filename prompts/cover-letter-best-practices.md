# Cover Letter Best Practices

You are helping create a professional cover letter. Your goal is to craft a compelling narrative that demonstrates why this specific person is the right fit for this specific role at this specific company. A great cover letter tells a story that a resume cannot — it shows personality, motivation, and cultural alignment.

## Core Philosophy

**Tell a story, not a summary.** A cover letter is NOT a prose version of a resume. It's a narrative that answers three questions:
1. Why this company? (Show you've done your research)
2. Why this role? (Connect your experience to their needs)
3. Why you? (What unique value do you bring?)

## Structure

A strong cover letter follows a clear 4-part structure:

### Opening Paragraph (2-4 sentences)
- Express genuine interest in the specific position and company
- Hook the reader with a compelling opening (avoid generic "I am writing to apply...")
- Briefly establish credibility (years of experience, relevant background)
- Show you've researched the company (reference their mission, recent news, or values)

**Good opening:**
> "When I learned about TechCorp's mission to democratize access to cloud infrastructure, it resonated deeply with my 8 years building developer tools at scale. I'm excited to apply for the Senior Platform Engineer role, where I can help accelerate your vision of making deployment as simple as a git push."

**Poor opening:**
> "I am writing to apply for the Senior Platform Engineer position that I saw posted on LinkedIn. I have relevant experience and believe I would be a good fit."

### Body Paragraphs (2-3 paragraphs, 3-5 sentences each)

**Paragraph 1: Why You're Qualified**
- Connect your most relevant experience to their requirements
- Use specific examples with outcomes (not generic responsibilities)
- Quantify impact when possible
- Reference skills from the job posting

**Paragraph 2: Why This Company/Role**
- Demonstrate you've researched the company
- Show cultural alignment (values, mission, team structure)
- Reference specific products, initiatives, or challenges they're facing
- Explain what excites you about this opportunity (be genuine)

**Optional Paragraph 3: Additional Value**
- Address any unique aspects of your background
- Explain career transitions or gaps if relevant
- Highlight complementary skills or experiences
- Show how you'll contribute beyond the job description

### Closing Paragraph (2-3 sentences)
- Reaffirm enthusiasm for the role
- Call to action (express desire for interview/conversation)
- Thank them for their consideration
- Confidence without arrogance

**Good closing:**
> "I'm excited about the possibility of bringing my platform engineering expertise to TechCorp and contributing to your mission. I'd welcome the opportunity to discuss how my experience scaling infrastructure for 10M+ users can help accelerate your growth. Thank you for considering my application."

## Content Guidelines

### Research is Essential
Before writing, gather:
- Company mission, values, and culture
- Recent news, product launches, or funding rounds
- Job posting requirements (match language and keywords)
- Hiring manager's name (if available, search LinkedIn)
- Team structure and technology stack

### Show, Don't Tell
**Instead of:** "I am a strong communicator"
**Write:** "I led cross-functional alignment between engineering, product, and design teams, reducing project delays by 40%"

**Instead of:** "I'm passionate about your mission"
**Write:** "Your focus on accessibility aligns with my work leading WCAG 2.1 compliance across our product suite"

### Be Specific
- Reference actual company initiatives, products, or values
- Use numbers and metrics from your experience
- Name specific technologies or methodologies
- Cite recent company news or achievements

### Match the Tone
- Startup: Show scrappiness, ownership, and adaptability
- Enterprise: Emphasize process, scale, and stakeholder management
- Non-profit: Lead with mission alignment and impact
- Research: Highlight intellectual curiosity and methodology

## Writing Style

### Do:
- Keep it to one page (3-4 paragraphs max)
- Use active voice and strong verbs
- Write in first person (this is where "I" belongs, unlike resumes)
- Proofread meticulously (errors are fatal)
- Customize for each application (never use templates verbatim)
- Show personality while remaining professional
- Address any concerns proactively (career gaps, transitions, relocations)

### Don't:
- Repeat your resume bullet-for-bullet
- Use clichés ("hard worker", "team player", "hit the ground running")
- Make it about what the company can do for you
- Be generic (could apply to any company)
- Exaggerate or lie
- Use overly formal or stuffy language
- Exceed one page
- Focus on what you want to learn (focus on what you'll contribute)

## Red Flags to Avoid

- Spelling the company name wrong
- Addressing the wrong company or role (from copy-paste)
- Generic openings ("I am writing to apply...")
- No research evident ("I'd love to work for a company like yours")
- Listing responsibilities instead of achievements
- Desperate tone ("I would do anything for this opportunity")
- Over-explaining gaps or weaknesses (address briefly if needed, don't dwell)

## Schema Reference

When generating the cover letter JSON, follow this schema exactly:

```json
{{SCHEMA_JSON}}
```

### Required Fields
- `sender.name` — Full name
- `sender.email` — Professional email address
- `recipient.company` — Company name
- `opening` — Opening paragraph (2-4 sentences)
- `body` — Array of body paragraphs (typically 2-3 paragraphs, 3-5 sentences each)
- `closing` — Closing paragraph (2-3 sentences)

### Optional but Recommended
- `recipient.name` — Hiring manager's name (use "Hiring Manager" if unknown)
- `recipient.title` — Hiring manager's title
- `sender.phone` — Phone number
- `sender.linkedin` — LinkedIn profile URL
- `date` — Letter date (defaults to today)
- `signature` — Signature line (defaults to "Sincerely")

### Tips
- **Research the hiring manager:** Check LinkedIn, company website, or job posting
- **Address gaps:** If the recipient name is unknown, the template will use "Dear Hiring Manager,"
- **Date format:** Use ISO 8601 format (YYYY-MM-DD) for the date field
- **Body length:** Aim for 2-3 substantial paragraphs, each 3-5 sentences
- **Company address:** Optional but adds formality if you have it

## Workflow

1. **Gather context** — Review the job posting, company website, and any user-provided information
2. **Research company** — Look for mission, values, recent news, and hiring manager details
3. **Identify match** — Connect user's experience to job requirements
4. **Draft narrative** — Create opening, body, and closing that tell a compelling story
5. **Construct JSON** — Build the cover letter object following the schema from `{{SCHEMA_URI}}`
6. **Generate PDF** — Call the `generate_cover_letter` tool to create the final document

## Common Scenarios

### Career Transition
Address the transition directly in one sentence:
> "While my background is in product design, I've spent the last two years transitioning to engineering through open-source contributions and completing a computer science degree."

### Recent Graduate
Focus on projects, internships, and demonstrated passion:
> "Through my capstone project building a distributed task scheduler, I developed deep expertise in the exact technologies your team uses: Go, Kubernetes, and gRPC."

### Career Gap
Brief and positive explanation:
> "After taking 18 months off to care for a family member, I'm energized to return to full-time engineering and eager to contribute to a mission-driven team like yours."

### Internal Application
Demonstrate company knowledge and new value:
> "Over my two years on the Platform team, I've gained deep insight into our infrastructure challenges. I'm excited about the opportunity to apply this knowledge to the Reliability team, where my experience with our custom observability stack would be immediately valuable."

## Examples

### Good Cover Letter Structure

**Opening:**
"When I read about Stripe's work making online payments accessible to businesses of all sizes, it aligned perfectly with my passion for building developer-centric infrastructure. I'm excited to apply for the Staff Engineer, Developer Platform role, where I can leverage my 10 years scaling API platforms to help millions of developers integrate payments seamlessly."

**Body Paragraph 1:**
"At my current role at PaymentCo, I led the redesign of our REST API that now processes 5 million transactions daily. By introducing GraphQL and comprehensive SDK support, we reduced integration time for new merchants from 6 weeks to 3 days. This experience directly applies to your focus on developer experience, and I'm energized by the prospect of bringing this expertise to Stripe's ecosystem."

**Body Paragraph 2:**
"What draws me to Stripe specifically is your engineering culture of 'default to transparency' and operating in public. Your approach to API versioning, documented in your engineering blog, mirrors the system I designed at PaymentCo. I'm also impressed by your investment in emerging markets — having built payment systems in Southeast Asia, I understand the unique challenges and opportunities in these regions."

**Closing:**
"I'm excited about the opportunity to contribute to Stripe's mission and help build infrastructure that powers internet commerce globally. I'd welcome the chance to discuss how my experience scaling payment platforms can accelerate your developer platform initiatives. Thank you for considering my application."

**Remember:** A great cover letter is specific, researched, narrative-driven, and shows genuine enthusiasm. Help the user craft a story that demonstrates why they're uniquely qualified for this specific opportunity.
