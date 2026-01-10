#let cover_letter(data) = {
  set text(font: "Libertinus Serif", size: 11pt)

  set page(
    paper: "us-letter",
    margin: (x: 1in, y: 1in),
  )

  set par(justify: true, leading: 0.65em, spacing: 0.65em)

  // Helper to format date
  let format-date(date-str) = {
    if date-str != none {
      date-str
    } else {
      datetime.today().display("[month repr:long] [day], [year]")
    }
  }

  // === SENDER'S CONTACT INFO (top left) ===
  text(weight: "bold", data.sender.name)
  linebreak()

  if "address" in data.sender and data.sender.address != none [
    #data.sender.address
    #linebreak()
  ]

  if "phone" in data.sender and data.sender.phone != none [
    #data.sender.phone |
  ]

  data.sender.email

  if "linkedin" in data.sender and data.sender.linkedin != none [
    #linebreak()
    #link(data.sender.linkedin)[LinkedIn Profile]
  ]

  v(1.5em)

  // === DATE ===
  let letter-date = if "date" in data and data.date != none { data.date } else { none }
  format-date(letter-date)

  v(1.5em)

  // === RECIPIENT INFO ===
  if "name" in data.recipient and data.recipient.name != none [
    #data.recipient.name
    #linebreak()
  ]

  if "title" in data.recipient and data.recipient.title != none [
    #data.recipient.title
    #linebreak()
  ]

  data.recipient.company
  linebreak()

  if "address" in data.recipient and data.recipient.address != none [
    #data.recipient.address
    #linebreak()
  ]

  v(1.5em)

  // === SALUTATION ===
  let salutation = if "name" in data.recipient and data.recipient.name != none {
    "Dear " + data.recipient.name + ","
  } else {
    "Dear Hiring Manager,"
  }

  salutation

  v(1em)

  // === OPENING PARAGRAPH ===
  par(data.opening)

  v(0.65em)

  // === BODY PARAGRAPHS ===
  for paragraph in data.body [
    #par(paragraph)
    #v(0.65em)
  ]

  // === CLOSING PARAGRAPH ===
  par(data.closing)

  v(1em)

  // === SIGNATURE ===
  let sig = if "signature" in data and data.signature != none { data.signature } else { "Sincerely" }

  sig + ","

  v(3em)

  data.sender.name
}

// Main document entry point
#show: cover_letter
