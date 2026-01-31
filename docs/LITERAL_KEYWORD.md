# Literal & Keyword

## Literal

| Type | Pattern | Example |
| --- | --- | --- |
| Number | `-?(\d+\.?\d*\|\.d+)((i\|u)(8\|16\|32\|64\|128)\|f(32\|64))?` | `12`, `-35i64`, `.2f32` |
| String | `".*"` | `"Hello, world!"`, `"dq\"lf\n"` |
| Symbol | `'.+` | `'foo`, `'symbol` |
| Argument | `#\d+` | `#12` |
| Directive | `/.+` | `/exit` |

## Keyword

| Notation | Meaning |
| --- | --- |
| `T` | True value |
| `##` | Self-reference |
