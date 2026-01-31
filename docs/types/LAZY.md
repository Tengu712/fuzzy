# \{\}

## General

A type representing a deferred evaluation block.
Can be viewed as a collection of strings.

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
{ 1 + 2 } ! -- outputs {}
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
{ 1 + 2 } !! -- outputs {}
             -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
{ 1 + 2 } -> 'block. -- defines a mutable variable called block
block                -- { 1 + 2 }
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
{ "hello" !! } => 'printer. -- defines an immutable variable called printer
printer                     -- { "hello" !! }
```

### `#`: `@[]`

Returns the length of the deferred block.
The return type is `'u32`.

```fuzzy
{} #        -- 0
{ 1 + 2 } # -- 3
```

### `^`: `@[]`

Returns the first element.
The return type is `'string`.
Returns `()` if the deferred block is empty.

```fuzzy
{} ^        -- ()
{ 1 + 2 } ^ -- "1"
```

### `$`: `@[]`

Returns the last element.
The return type is `'string`.
Returns `()` if the block is empty.

```fuzzy
{} $        -- ()
{ 1 + 2 } $ -- "2"
```

### `@`: `@['i32]`

Returns the element at the specified index.
The return type is `'string`.
Returns `()` if the index is out of bounds.

```fuzzy
{ 1 + 2 } $ 1   -- "+"
{ 1 + 2 } $ -1  -- "2"
{ 1 + 2 } @ 100 -- ()
```

### `@@`: `@['i32 'string]`

Replaces the element at the specified index with the object.

```fuzzy
{ 1 + 2 } @@ 1 "-" -- { 1 - 2 }
```

### `@<`: `@['i32 'string]`

Inserts an element at the specified index.

```fuzzy
{ 1 2 } @@ 1 "+" -- { 1 + 2 }
```

### `@-`: `@['i32]`

Removes the element at the specified index.

```fuzzy
{ 1 + 2 } @@ 1 -- { 1 2 }
```

### `$-`: `@[]`

Removes the last element.

```fuzzy
{ 1 + 2 } $- -- { 1 + }
```

### `$>`: `@['_]`

Appends an element to the end.

```fuzzy
{ 1 + } $> "2" -- { 1 + 2 }
```

### `%`: `@[]`

Evaluates the deferred block.

```fuzzy
{ 1 + 2 } % -- 3
```

### `%%`: `@['{}]`

Repeatedly evaluates the object until the subject returns `()`.

```fuzzy
5 -> 'i.
{ i > 0 } %% {
  i !
  i - 1, -> 'i.
}
-- outputs 54321
```

### `:`: `@['[]]`

Casts the subject to the function type indicated by the object.
The object must be an array that satisfies the following requirements:

- All leaf elements must be of type `'symbol`
- Each element represents an argument type
- If an element is an array, it represents a function type

```fuzzy
{ #0 @ #1 #2 } : [['i32 'i32] 'i32 'i32] -- a function that takes a function with 2 'i32 arguments and 2 'i32 arguments
```

### `==`: `@['{}]`

Checks if the subject is equal to the object.

```fuzzy
{ 1 + 2 } == { 1 + 2 } -- T
{ 1 + 2 } == { 2 + 1 } -- ()
```

### `!=`: `@['{}]`

Checks if the subject is not equal to the object.

```fuzzy
{ 1 + 2 } != { 1 + 2 } -- ()
{ 1 + 2 } != { 2 + 1 } -- T
```
