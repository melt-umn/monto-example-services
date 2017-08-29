# monto-example-services

Example Monto3 services, mainly for demos.

## Services

All of the services operate on the `text` language.

### `edu.umn.cs.melt.monto_example_services.char_count`

A simple character count of the source file.

### `edu.umn.cs.melt.monto_example_services.line_count`

A simple line count of the source file.
Only counts Unix-style `\n` newlines, although this should be accurate for Windows' `\r\n` newlines.
Watch out if you're on Mac OS 9 or an MIT Lisp Machine!

### `edu.umn.cs.melt.monto_example_services.reverse`

Reverses the source file at the grapheme cluster level.
This means, for example, that "noël" gets reversed to "lëon", rather than "l̈eon".

## Installation

```
git clone https://github.com/melt-umn/monto-example-services.git
cd monto-example-services
cargo install
```

## Configuration

TODO
