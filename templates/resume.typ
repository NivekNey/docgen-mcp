#let resume(data) = {
  set text(font: "Libertinus Serif", size: 10pt)

  // Extract configuration options with defaults
  let show-header = if "showHeader" in data { data.showHeader } else { true }
  let show-page-numbers = if "showPageNumbers" in data { data.showPageNumbers } else { true }

  set page(
    paper: "us-letter",
    margin: (x: 0.5in, y: 0.5in),
    header: if show-header {
      context {
        let page-num = counter(page).get().first()
        if page-num > 1 [
          #set text(size: 9pt)
          #line(length: 100%, stroke: 0.5pt)
          #v(-8pt)
          #align(center)[#data.basics.name]
          #v(-4pt)
        ]
      }
    },
    footer: if show-page-numbers {
      context {
        set text(size: 9pt)
        let page-num = counter(page).get().first()
        let page-count = counter(page).final().first()
        align(center)[Page #page-num of #page-count]
      }
    },
  )
  set par(justify: true)

  // Prevent orphaned headlines and widow/orphan lines
  set par(leading: 0.65em, spacing: 0.65em)
  set block(spacing: 0.65em)

  // Helper for section headers with custom title support
  let section-header(default-title, section-name: none) = {
    let title = default-title
    if section-name != none and "sectionTitles" in data and data.sectionTitles != none {
      if section-name in data.sectionTitles {
        title = data.sectionTitles.at(section-name)
      }
    }
    v(0pt)
    text(size: 12pt, weight: "bold", smallcaps(title))
    v(-10pt)
    line(length: 100%, stroke: 0.5pt)
  }

  // Helper for entry headers (4-quadrant layout)
  let entry-header(top-left, top-right, bottom-left, bottom-right) = {
    grid(
      columns: (1fr, auto),
      rows: (auto, auto),
      gutter: 4pt,
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

  // === SECTION RENDERERS ===

  let render-education() = {
    if "education" in data and data.education.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Education", section-name: "education")
        #if data.education.len() > 0 {
          let edu = data.education.at(0)
          entry-header(
            edu.institution,
            if "location" in edu and edu.location != none [#edu.location],
            [#if "degree" in edu [#edu.degree]#if "fieldOfStudy" in edu [, #edu.fieldOfStudy]],
            format-dates(
              if "startDate" in edu { edu.startDate } else { none },
              if "endDate" in edu { edu.endDate } else { none }
            )
          )
          if "gpa" in edu and edu.gpa != none [
            GPA: #edu.gpa
          ]
          if "highlights" in edu and edu.highlights.len() > 0 [
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in edu.highlights [
              - #h
            ]
          ]
        }
      ]
      // Render remaining entries
      for edu in data.education.slice(1) [
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
            GPA: #edu.gpa
          ]
          #if "highlights" in edu and edu.highlights.len() > 0 [
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in edu.highlights [
              - #h
            ]
          ]
        ]
      ]
    }
  }

  let render-experience() = {
    if "work" in data and data.work.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Experience", section-name: "experience")
        #if data.work.len() > 0 {
          let w = data.work.at(0)
          entry-header(
            w.position,
            format-dates(
              if "startDate" in w { w.startDate } else { none },
              if "endDate" in w { w.endDate } else { none }
            ),
            w.company,
            if "location" in w and w.location != none [#w.location]
          )
          if "highlights" in w and w.highlights.len() > 0 [
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in w.highlights [
              - #h
            ]
          ]
        }
      ]
      // Render remaining entries
      for w in data.work.slice(1) [
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
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in w.highlights [
              - #h
            ]
          ]
        ]
      ]
    }
  }

  let render-projects() = {
    if "projects" in data and data.projects.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Projects", section-name: "projects")
        #if data.projects.len() > 0 {
          let p = data.projects.at(0)
          grid(
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
          if "description" in p and p.description != none [
            #text(style: "italic", size: 9pt)[#p.description]
          ]
          if "highlights" in p and p.highlights.len() > 0 [
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in p.highlights [
              - #h
            ]
          ]
        }
      ]
      // Render remaining entries
      for p in data.projects.slice(1) [
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
            #text(style: "italic", size: 9pt)[#p.description]
          ]
          #if "highlights" in p and p.highlights.len() > 0 [
            #set list(marker: text(size: 0.7em)[•], body-indent: 0.5em, spacing: 4pt)
            #for h in p.highlights [
              - #h
            ]
          ]
        ]
      ]
    }
  }

  let render-certifications() = {
    if "certifications" in data and data.certifications.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Certifications", section-name: "certifications")
        #if data.certifications.len() > 0 {
          let cert = data.certifications.at(0)
          grid(
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
          if "url" in cert and cert.url != none [
            #link(cert.url)[#underline(text(size: 9pt)[#cert.url.replace("https://", "").replace("http://", "")])]
          ]
        }
      ]
      // Render remaining entries
      for cert in data.certifications.slice(1) [
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
      ]
    }
  }

  let render-awards() = {
    if "awards" in data and data.awards.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Awards", section-name: "awards")
        #if data.awards.len() > 0 {
          let award = data.awards.at(0)
          grid(
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
          if "summary" in award and award.summary != none [
            #text(size: 9pt)[#award.summary]
          ]
        }
      ]
      // Render remaining entries
      for award in data.awards.slice(1) [
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
            #text(size: 9pt)[#award.summary]
          ]
        ]
      ]
    }
  }

  let render-publications() = {
    if "publications" in data and data.publications.len() > 0 {
      // Wrap header with first entry to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Publications", section-name: "publications")
        #if data.publications.len() > 0 {
          let pub = data.publications.at(0)
          grid(
            columns: (1fr, auto),
            [
              *#pub.title*
              #if "authors" in pub and pub.authors.len() > 0 [
                \ #text(style: "italic", size: 9pt)[#pub.authors.join(", ")]
              ]
              #if "venue" in pub and pub.venue != none [
                \ #text(size: 9pt)[#pub.venue]
              ]
              #if "url" in pub and pub.url != none [
                \ #link(pub.url)[#underline(text(size: 9pt)[#pub.url.replace("https://", "").replace("http://", "")])]
              ]
            ],
            align(right)[
              #if "date" in pub and pub.date != none [#pub.date]
            ]
          )
          if "summary" in pub and pub.summary != none [
            #text(size: 9pt)[#pub.summary]
          ]
        }
      ]
      // Render remaining entries
      for pub in data.publications.slice(1) [
        #block(breakable: false)[
          #grid(
            columns: (1fr, auto),
            [
              *#pub.title*
              #if "authors" in pub and pub.authors.len() > 0 [
                \ #text(style: "italic", size: 9pt)[#pub.authors.join(", ")]
              ]
              #if "venue" in pub and pub.venue != none [
                \ #text(size: 9pt)[#pub.venue]
              ]
              #if "url" in pub and pub.url != none [
                \ #link(pub.url)[#underline(text(size: 9pt)[#pub.url.replace("https://", "").replace("http://", "")])]
              ]
            ],
            align(right)[
              #if "date" in pub and pub.date != none [#pub.date]
            ]
          )
          #if "summary" in pub and pub.summary != none [
            #text(size: 9pt)[#pub.summary]
          ]
        ]
      ]
    }
  }

  let render-skills() = {
    if "skills" in data and data.skills.len() > 0 {
      // Wrap header with content to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Technical Skills", section-name: "skills")
        #for skill in data.skills [
          *#skill.name:* #skill.keywords.join(", ")
          #linebreak()
        ]
      ]
    }
  }

  let render-languages() = {
    if "languages" in data and data.languages.len() > 0 {
      // Wrap header with content to prevent orphaned headlines
      block(breakable: false)[
        #section-header("Languages", section-name: "languages")
        #let lang-items = data.languages.map(lang => {
          if "fluency" in lang and lang.fluency != none [*#lang.language* (#lang.fluency)]
          else [*#lang.language*]
        })
        #lang-items.join("  •  ")
      ]
    }
  }

  // Section dispatcher
  let render-section(name) = {
    if name == "education" { render-education() }
    else if name == "experience" { render-experience() }
    else if name == "projects" { render-projects() }
    else if name == "certifications" { render-certifications() }
    else if name == "awards" { render-awards() }
    else if name == "publications" { render-publications() }
    else if name == "skills" { render-skills() }
    else if name == "languages" { render-languages() }
  }

  // Default section order
  let default-order = ("education", "experience", "projects", "certifications", "awards", "publications", "skills", "languages")

  // Determine section order to use
  let section-order = if "sectionOrder" in data and data.sectionOrder != none {
    data.sectionOrder
  } else {
    default-order
  }

  // === HEADER ===
  align(center)[
    #text(2em, weight: "bold", smallcaps(data.basics.name))

    // Location line (if present)
    #if "location" in data.basics and data.basics.location != none [
      #text(size: 10pt)[#data.basics.location]
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
    #text(size: 9pt)[
      #for (i, item) in contact.enumerate() [
        #if i > 0 [  |  ]#item
      ]
    ]
  ]

  // === SUMMARY ===
  if "summary" in data.basics and data.basics.summary != none [
    #data.basics.summary
  ]

  // === RENDER SECTIONS IN ORDER ===
  for section in section-order {
    render-section(section)
  }
}
