# svg2sixel
Render a SVG as sixel for display in terminals

## Usage

As a library:
```rust
    match svg2sixel::svg2sixel(svg_string) {
        Ok(data) => println!("{data}"),
        Err(err) => eprintln!("{err}"),
    }
```

As a command line tool:

```bash
$ ./svg2sixel image.svg
```

or via stdin

```bash
cat image.svg | $ ./svg2sixel -
```
