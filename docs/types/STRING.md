# string

## General

A type representing strings.
Supports the following escape characters:

| Notation | Meaning |
| --- | --- |
| `\\` | \\ |
| `\"` | " |
| `\r` | CR |
| `\n` | LF |
| `\t` | Tab character |
| `\0` | Null character |

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
"Hello, world!" ! -- outputs Hello, world!
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
"Hello, world!" !! -- outputs Hello, world!
                   -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
"hello" -> 'greeting. -- defines a mutable variable called greeting
greeting              -- "hello"
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
"world" => 'name. -- defines an immutable variable called name
name              -- "world"
```

### `#`: `@[]`

Returns the length of the string.
The return type is `'u32`.

```fuzzy
"hello" # -- 5
"" #      -- 0
```

### `^`: `@[]`

Returns the first character.
The return type is `'string`.
Returns `()` if the string is empty.

```fuzzy
"hello" ^ -- "h"
"" ^      -- ()
```

### `$`: `@[]`

Returns the last character.
The return type is `'string`.
Returns `()` if the string is empty.

```fuzzy
"hello" $ -- "o"
"" $      -- ()
```

### `@`: `@['i32]`

Returns the character at the specified index.
The return type is `'string`.
Returns `()` if the index is out of bounds.

```fuzzy
"hello" @ 3  -- "l"
"world" @ -1 -- "d"
"!" @ 100    -- ()
```

### `@<`: `@['i32 'string]`

Inserts a character at the specified index.
The object string must be a single character.

```fuzzy
"hello" @< 1 "x" -- "hxello"
```

### `@-`: `@['i32]`

Removes the character at the specified index.

```fuzzy
"hello" @- 1 -- "hllo"
```

### `$-`: `@[]`

Removes the last character.

```fuzzy
"hello" $- -- "hell"
```

### `$>`: `@['string]`

Concatenates the object to the end of the subject.

```fuzzy
"Hello, " $> "world!" -- "Hello, world!"
```

### `=@`: `@['string 'string]`

Replaces object1 with object2 in the subject.

```fuzzy
"Hello, world!" =@ "world" "fuzzy" -- "Hello, fuzzy!"
"foo bar baz" =@ "ba" "BA"         -- "foo BAr BAz"
```

### `<`: `@['string]`

Checks if the subject is less than the object.

```fuzzy
"" < ""    -- ()
"a" < "ab" -- T
"2" < "10" -- ()
```

### `<=`: `@['string]`

Checks if the subject is less than or equal to the object.

```fuzzy
"" <= ""    -- T
"a" <= "ab" -- T
"2" <= "10" -- ()
```

### `>`: `@['string]`

Checks if the subject is greater than the object.

```fuzzy
"" > ""    -- ()
"a" > "ab" -- ()
"2" > "10" -- T
```

### `>=`: `@['string]`

Checks if the subject is greater than or equal to the object.

```fuzzy
"" >= ""    -- T
"a" >= "ab" -- ()
"2" >= "10" -- T
```

### `==`: `@['string]`

Checks if the subject is equal to the object.

```fuzzy
"" == ""    -- T
"a" == "ab" -- ()
```

### `!=`: `@['string]`

Checks if the subject is not equal to the object.

```fuzzy
"" != ""    -- ()
"a" != "ab" -- T
```
