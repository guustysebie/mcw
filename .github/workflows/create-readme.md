name: README to PDF

on:
  workflow_dispatch:      # lets you run it manually from the Actions tab
  push:
    paths: ['README.md']  # optional: rebuild whenever the README changes

permissions:
  contents: read

jobs:
  convert:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: itext/markdown2pdf-gh-action@v0.3.0
        id: convert
        with:
          input: 'README.md'
          output: 'dist/README.pdf'

      - uses: actions/upload-artifact@v4
        with:
          name: README-pdf
          path: ${{ steps.convert.outputs.output-path }}
          if-no-files-found: error
