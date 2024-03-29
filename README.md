# mdbook-html-taco

`mdbook-html-taco` is an alternate backend for [mdBook] that is essentially the
HTML renderer with some slight modifications. It is itself a fork of mdBook due
to the HTML renderer's coupling with mdBook.

## Installation

Clone this repository and run `cargo install --path .` inside the directory.
Then, modify the `book.toml` to ensure that the `html-taco` backend will be used
rather than the `html` backend:

```toml
[book]
authors = ["Alluet"]
title = "Example Documentation"
multilingual = false
src = "src"

[output.html-taco]
root-path = "https://example.com/docs/"
print-path = "print/index.md"
strip-index = true
```

## Usage

The `html-taco` backend is identical to the `html` backend, except for the
following changes:

 - All URLs generated by the backend are absolute. `output.html-taco.root-path` specifies the absolute root path of the book, including scheme, domain, and a trailing slash.
 - The location of the generated print page can be overriden with `output.html-taco.print-path`.
 - If `output.html-taco.strip-index` is true, `index.html` will be stripped from generated links.

## License

All the code in this repository is released under the ***Mozilla Public License
v2.0***, for more information take a look at the [LICENSE] file.

[mdBook]: https://github.com/rust-lang-nursery/mdbook
[LICENSE]: https://github.com/alluet/mdbook-html-taco/blob/master/LICENSE
