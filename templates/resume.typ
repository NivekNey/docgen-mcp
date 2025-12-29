#let resume(data) = {
  set text(font: "Libertinus Serif", size: 10pt)
  set page(
    paper: "us-letter",
    margin: (x: 0.5in, y: 0.5in),
  )
  set par(justify: true)

  // Helper for section headers
  let section-header(title) = {
    v(6pt)
    text(size: 12pt, weight: "bold", smallcaps(title))
    v(-8pt)
    line(length: 100%, stroke: 0.5pt)
    v(2pt)
  }

  // Helper for entry headers (4-quadrant layout)
  let entry-header(top-left, top-right, bottom-left, bottom-right) = {
    grid(
      columns: (1fr, auto),
      rows: (auto, auto),
      gutter: 0pt,
      text(weight: "bold")[#top-left],
      align(right)[#top-right],
      text(style: "italic")[#bottom-left],
      align(right, text(style: "italic")[#bottom-right]),
    )
  }

  // Format date range
  let format-dates(start, end) = {
    if start != none and end != none [#start -- #end]
    else if start != none [#start]
    else if end != none [#end]
  }

  // === HEADER ===
  align(center)[
    #text(2em, weight: "bold", smallcaps(data.basics.name))
    #v(-4pt)

    // Location line (if present)
    #if "location" in data.basics and data.basics.location != none [
      #text(size: 10pt)[#data.basics.location]
      #v(-2pt)
    ]

    // Contact line
    #let contact = ()
    #if "phone" in data.basics and data.basics.phone != none { contact.push(data.basics.phone) }
    #contact.push(link("mailto:" + data.basics.email)[#underline(data.basics.email)])
    #if "profiles" in data.basics {
      for p in data.basics.profiles {
         contact.push(link(p.url)[#underline(p.url.replace("https://", "").replace("http://", ""))])
      }
    }
    #text(size: 9pt)[#contact.join("  |  ")]
  ]

  // === SUMMARY ===
  if "summary" in data.basics and data.basics.summary != none [
    #v(6pt)
    #text(style: "italic")[#data.basics.summary]
  ]

  // === EDUCATION ===
  if "education" in data and data.education.len() > 0 [
    #section-header("Education")
    #for edu in data.education [
      #block(breakable: false)[
        #entry-header(
          edu.institution,
          if "location" in edu and edu.location != none [#edu.location],
          [#if "degree" in edu [#edu.degree]#if "fieldOfStudy" in edu [, #edu.fieldOfStudy]],
          format-dates(
            if "startDate" in edu { edu.startDate } else { none },
            if "endDate" in edu { edu.endDate } else { none }
          )
        )
        #if "gpa" in edu and edu.gpa != none [
          #v(1pt)
          GPA: #edu.gpa
        ]
        #if "highlights" in edu and edu.highlights.len() > 0 [
          #v(2pt)
          #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 3pt)
          #for h in edu.highlights [
            - #h
          ]
        ]
      ]
      #v(4pt)
    ]
  ]

  // === EXPERIENCE ===
  if "work" in data and data.work.len() > 0 [
    #section-header("Experience")
    #for w in data.work [
      #block(breakable: false)[
        #entry-header(
          w.position,
          format-dates(
            if "startDate" in w { w.startDate } else { none },
            if "endDate" in w { w.endDate } else { none }
          ),
          w.company,
          if "location" in w and w.location != none [#w.location]
        )
        #if "highlights" in w and w.highlights.len() > 0 [
          #v(2pt)
          #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 3pt)
          #for h in w.highlights [
            - #h
          ]
        ]
      ]
      #v(4pt)
    ]
  ]

  // === PROJECTS ===
  if "projects" in data and data.projects.len() > 0 [
    #section-header("Projects")
    #for p in data.projects [
      #block(breakable: false)[
        #grid(
          columns: (1fr, auto),
          [
            *#p.name*
            #if "keywords" in p and p.keywords.len() > 0 [
              #h(4pt) | #h(4pt) #text(style: "italic", size: 9pt)[#p.keywords.join(", ")]
            ]
            #if "url" in p and p.url != none [
              #h(4pt) | #h(4pt) #link(p.url)[#underline(text(size: 9pt)[#p.url.replace("https://", "").replace("http://", "")])]
            ]
          ],
          align(right)[
            #format-dates(
              if "startDate" in p { p.startDate } else { none },
              if "endDate" in p { p.endDate } else { none }
            )
          ]
        )
        #if "description" in p and p.description != none [
          #v(1pt)
          #text(style: "italic", size: 9pt)[#p.description]
        ]
        #if "highlights" in p and p.highlights.len() > 0 [
          #v(2pt)
          #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 3pt)
          #for h in p.highlights [
            - #h
          ]
        ]
      ]
      #v(4pt)
    ]
  ]

  // === CERTIFICATIONS ===
  if "certifications" in data and data.certifications.len() > 0 [
    #section-header("Certifications")
    #for cert in data.certifications [
      #block(breakable: false)[
        #grid(
          columns: (1fr, auto),
          [
            *#cert.name*
            #if "issuer" in cert and cert.issuer != none [
              #h(4pt) | #h(4pt) #text(style: "italic")[#cert.issuer]
            ]
          ],
          align(right)[
            #if "date" in cert and cert.date != none [#cert.date]
          ]
        )
        #if "url" in cert and cert.url != none [
          #link(cert.url)[#underline(text(size: 9pt)[#cert.url.replace("https://", "").replace("http://", "")])]
        ]
      ]
      #v(3pt)
    ]
  ]

  // === AWARDS ===
  if "awards" in data and data.awards.len() > 0 [
    #section-header("Awards")
    #for award in data.awards [
      #block(breakable: false)[
        #grid(
          columns: (1fr, auto),
          [
            *#award.title*
            #if "awarder" in award and award.awarder != none [
              #h(4pt) | #h(4pt) #text(style: "italic")[#award.awarder]
            ]
          ],
          align(right)[
            #if "date" in award and award.date != none [#award.date]
          ]
        )
        #if "summary" in award and award.summary != none [
          #v(1pt)
          #text(size: 9pt)[#award.summary]
        ]
      ]
      #v(3pt)
    ]
  ]

  // === PUBLICATIONS ===
  if "publications" in data and data.publications != none [
    #section-header("Publications")
    #text[#data.publications]
    #v(4pt)
  ]

  // === SKILLS ===
  if "skills" in data and data.skills.len() > 0 [
    #section-header("Technical Skills")
    #for skill in data.skills [
      *#skill.name:* #skill.keywords.join(", ")
      #linebreak()
    ]
  ]

  // === LANGUAGES ===
  if "languages" in data and data.languages.len() > 0 [
    #section-header("Languages")
    #let lang-items = data.languages.map(lang => {
      if "fluency" in lang and lang.fluency != none [*#lang.language* (#lang.fluency)]
      else [*#lang.language*]
    })
    #lang-items.join("  •  ")
  ]
}
