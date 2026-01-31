# symbol

## General

A symbol type.
An identifier that starts with `'`.

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
'foo ! -- outputs foo
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
'foo !! -- outputs foo
        -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
'hello -> 'sym. -- defines a mutable variable called sym
sym             -- 'hello
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
'world => 'name. -- defines an immutable variable called name
name             -- 'world
```

### `%`: `@[]`

Evaluates the symbol.

```fuzzy
12 -> 'a. -- define a variable named a
'a %      -- treated as a
          -- 12
```

### `<`: `@['symbol]`

Checks if the subject is less than the object.

```fuzzy
'a < 'ab -- T
'2 < '10 -- ()
```

### `<=`: `@['symbol]`

Checks if the subject is less than or equal to the object.

```fuzzy
'a <= 'ab -- T
'2 <= '10 -- ()
```

### `>`: `@['symbol]`

Checks if the subject is greater than the object.

```fuzzy
'a > 'ab -- ()
'2 > '10 -- T
```

### `>=`: `@['symbol]`

Checks if the subject is greater than or equal to the object.

```fuzzy
'a >= 'ab -- ()
'2 >= '10 -- T
```

### `==`: `@['symbol]`

Checks if the subject is equal to the object.

```fuzzy
'a == 'a  -- T
'a == 'ab -- ()
```

### `!=`: `@['symbol]`

Checks if the subject is not equal to the object.

```fuzzy
'a != 'a  -- ()
'a != 'ab -- T
```
