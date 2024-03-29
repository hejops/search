# search

Simple command-line tool to perform web searches

Usage:
```sh
search <engine> <query>
```

A file at `~/.config/search_engines` is required. This file must be tab-separated. Empty lines
and commented lines (beginning with `#`) are ignored. All other lines must possess exactly two
fields, containing the name of a search engine, and a base URL to which a query will be
appended to:

```
ddg  https://duckduckgo.com/?t=ffab&q=
```

One printf substitution is allowed:

```
rust_errors    https://doc.rust-lang.org/error_codes/E%s.html
```

`xdg-open` will be called on the resulting URL.
