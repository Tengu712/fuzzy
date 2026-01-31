# bool

## General

A type representing boolean values. It has only the following two values:

| Notation | Meaning |
| --- | --- |
| `T` | True |
| `()` | False |

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
T !  -- outputs T
() ! -- outputs ()
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
T !!  -- outputs T
      -- with newline
() !! -- outputs ()
      -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
T -> 'a. -- defines a mutable variable called a
a        -- T
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
T => 'b. -- defines an immutable variable called b
b        -- T
```

### `~`: `@[]`

Returns the negation.

```fuzzy
T ~  -- ()
() ~ -- T
```

### `&&`: `@['bool]`

Returns the logical AND of the subject and the object.

```fuzzy
T && T   -- T
T && ()  -- ()
() && T  -- ()
() && () -- ()
```

### `||`: `@['bool]`

Returns the logical OR of the subject and the object.

```fuzzy
T || T   -- T
T || ()  -- T
() || T  -- T
() || () -- ()
```

### `>>`: `@['{}]`

Evaluates the deferred block object when the subject is `T`.
Returns the subject.

```fuzzy
1 == 1 >> { "true" !! } -- outputs true
                        -- T
0 == 1 >> { "true" !! } -- outputs nothing
                        -- ()
```

### `!>`: `@['{}]`

Evaluates the deferred block object when the subject is `()`.
Returns the subject.

```fuzzy
1 == 1 !> { "false" !! } -- outputs nothing
                         -- T
0 == 1 !> { "false" !! } -- outputs false
                         -- ()
```

### `<`: `@['bool]`

Checks if the subject is less than the object.

```fuzzy
T < T   -- ()
T < ()  -- ()
() < T  -- T
() < () -- ()
```

### `<=`: `@['bool]`

Checks if the subject is less than or equal to the object.

```fuzzy
T <= T   -- T
T <= ()  -- ()
() <= T  -- T
() <= () -- T
```

### `>`: `@['bool]`

Checks if the subject is greater than the object.

```fuzzy
T > T   -- ()
T > ()  -- T
() > T  -- ()
() > () -- ()
```

### `>=`: `@['bool]`

Checks if the subject is greater than or equal to the object.

```fuzzy
T >= T   -- T
T >= ()  -- T
() >= T  -- ()
() >= () -- T
```

### `==`: `@['bool]`

Checks if the subject is equal to the object.

```fuzzy
T == T   -- T
T == ()  -- ()
() == T  -- ()
() == () -- T
```

### `!=`: `@['bool]`

Checks if the subject is not equal to the object.

```fuzzy
T != T   -- ()
T != ()  -- T
() != T  -- T
() != () -- ()
```
