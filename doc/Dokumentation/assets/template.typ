#import "@preview/nassi:0.1.4"
#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge, shapes
#import "@preview/diagraph:0.3.3": render, raw-render
#import "@preview/wordometer:0.1.4": word-count, total-words
//#import "@preview/structogrammer:0.1.0": structogram
#import "strukto.typ": structogram

// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(
  title: "",
  subtitle: "",
  author: none,
  picture: none,
  class: "",
  teacher: "",
  subject: "",
  body,
) = {
  // Set the document's basic properties.
  set document(author: author, title: title)
  set page(numbering: "I", number-align: center)
  set text(font: "Libertinus Serif", lang: "de")
  set heading(numbering: "1.")
  show raw: set text(size: 10pt)
  show: nassi.shneiderman()
  show link: set text(fill: blue)
  show link: underline
  show ref: set text(fill: blue)
  show ref: underline
  set ref(supplement: "Absatz")

  let date = datetime.today().display("[day].[month].[year]")

  // Title page.
  // The page can contain a picture if you pass one with `picture: "picture.png"`.
  if picture != none {
    align(center, image(picture, width: 110%))
  }
  v(10.2fr)

  text(1.1em, date)
  v(1.2em, weak: true)
  text(2em, weight: 700, title)
  v(0em)
  text(1.3em, weight: 400, subtitle)

  // Author information.
  pad(
    top: 0.7em,
    right: 50%,
    grid(
      columns: (1fr, 1fr),
      gutter: 1em,
      align(right)[
        \
        Klasse: \
        Fach: \
        Lehrer: \
      ],
      [
        *#author* \
        #class \
        #subject \
        #teacher \
      ]
    ),
  )

  show math.equation: it => {
    show regex("\d+\.\d+"): it => {show ".": {","+h(0pt)}
      it
    }
    it
  }

  v(2.4fr)
  pagebreak()


  // Table of contents.
  outline(depth: 2, title: "Inhalt")
  v(1fr)
  
  context {
    let todos = query(figure.where(kind: "todo"))
  
    if todos.len() > 0 {
      outline(title: "TODOs", target: figure.where(kind: "todo"))
    }
  }

  outline(title: "Listings", target: figure.where(kind: raw))
  outline(title: "Tabellen", target: figure.where(kind: table))
  outline(title: "Abbildungen", target: figure.where(kind: image))
  pagebreak()

  counter(page).update(1)
  set page(
    numbering: "1",
    number-align: right,
    header: [
      #grid(
        columns: (1fr, 1fr, 1fr),
        align(left, author),
        align(center, title),
      )
      #line(length: 100%, stroke: color.rgb(150, 150, 150, 240))
    ],
    footer: context [
      #date
      #h(1fr)
      #counter(page).display()
    ]
  )

  show figure.where(kind: image): set figure(kind: image, supplement: "Abbildung")
  show figure.where(kind: table): set figure(kind: table, supplement: "Tabelle")
  show figure.where(kind: raw): set figure(kind: table, supplement: "Listing")
  

  // Main body.
  set par(justify: true)

  set text(size: 11pt)

  show: word-count.with(exclude: <code>)

  body
}

#let todo(message) = {
  set text(weight: 700, size: 20pt, fill: color.red)
  set align(center)

  show figure.caption: it => context [
    #it.body
  ]

  figure(
    kind: "todo",
    supplement: "TODO",
    caption: message,
    [! TODO !]
  )
}