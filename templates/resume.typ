#let resume(data) = {
  set text(font: "Libertinus Serif", size: 11pt)
  set page(
    paper: "us-letter",
    margin: (x: 0.5in, y: 0.5in),
  )
  
  // Helper for section headers
  let section-header(title) = {
    v(0.5em)
    text(size: 14pt, weight: "bold", smallcaps(title))
    v(-8pt)
    line(length: 100%, stroke: 0.5pt)
    v(3pt)
  }

  // Header
  align(center)[
    #text(2.5em, weight: "bold", smallcaps(data.basics.name))
    #v(-5pt)
    
    #let contact = ()
    #if "phone" in data.basics and data.basics.phone != none { contact.push(data.basics.phone) }
    #contact.push(link("mailto:" + data.basics.email)[#underline(data.basics.email)])
    #if "profiles" in data.basics {
      for p in data.basics.profiles {
         contact.push(link(p.url)[#underline(p.url.replace("https://", "").replace("http://", ""))])
      }
    }
    #contact.join(" | ")
  ]
  
  v(5pt)

  // Education
  if "education" in data and data.education.len() > 0 [
    #section-header("Education")
    #for edu in data.education [
      #block(breakable: false)[
        // Row 1: Institution | Location
        #box(width: 1fr, align(left)[*#edu.institution*])
        #box(align(right)[
          #if "location" in edu and edu.location != none [#edu.location]
        ]) \
        // Row 2: Degree, Field | Date
        #box(width: 1fr, align(left)[
           #text(style: "italic")[
             #if "degree" in edu [#edu.degree]
             #if "fieldOfStudy" in edu [, #edu.fieldOfStudy]
           ]
        ])
        #box(align(right)[
          #text(style: "italic")[
            #if "startDate" in edu and edu.startDate != none [#edu.startDate]
            #if "endDate" in edu and edu.endDate != none [ -- #edu.endDate]
          ]
        ])
        #if "gpa" in edu and edu.gpa != none [
          \ GPA: #edu.gpa
        ]
      ]
      #v(4pt)
    ]
  ]
  
  // Experience
  if "work" in data and data.work.len() > 0 [
    #section-header("Experience")
    #for w in data.work [
      #block(breakable: false)[
         // Row 1: Title | Date
         #box(width: 1fr, align(left)[*#w.position*])
         #box(align(right)[
            #if "startDate" in w and w.startDate != none [#w.startDate]
            #if "endDate" in w and w.endDate != none [ -- #w.endDate]
         ]) \
         // Row 2: Company | Location
         #box(width: 1fr, align(left)[#text(style: "italic")[#w.company]])
         #box(align(right)[
            #text(style: "italic")[
              #if "location" in w and w.location != none [#w.location]
            ]
         ])
         
         // Highlights
         #if "highlights" in w [
           #v(2pt)
           #set list(marker: text(size: 0.8em)[$bullet$], spacing: auto)
           #for highlight in w.highlights [
             - #highlight
           ]
         ]
      ]
      #v(4pt)
    ]
  ]
  
  // Projects
  if "projects" in data and data.projects.len() > 0 [
    #section-header("Projects")
    #for p in data.projects [
       #block(breakable: false)[
          // Row 1: Name | Tech Stack | Date
          #box(width: 1fr, align(left)[
            *#p.name* 
            #if "keywords" in p and p.keywords.len() > 0 [
               $|$ #text(style: "italic")[#p.keywords.join(", ")]
            ]
          ])
          #box(align(right)[
             #if "startDate" in p and p.startDate != none [#p.startDate]
             #if "endDate" in p and p.endDate != none [ -- #p.endDate]
          ]) \
          
          // Highlights
          #if "highlights" in p [
             #v(2pt)
             #set list(marker: text(size: 0.8em)[$bullet$], spacing: auto)
             #for highlight in p.highlights [
               - #highlight
             ]
          ]
       ]
       #v(4pt)
    ]
  ]

  // Skills
  if "skills" in data and data.skills.len() > 0 [
    #section-header("Technical Skills")
    #for skill in data.skills [
       *#skill.name*: #skill.keywords.join(", ") \
    ]
  ]
}
