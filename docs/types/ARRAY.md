# []

## General

A type representing an array.
Created using array blocks.
Arrays can contain values of any type.

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
[1 2 3] ! -- outputs [1 2 3]
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
[1 2 3] !! -- outputs [1 2 3]
           -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
[1 2 3] -> 'arr. -- defines a mutable variable called arr
arr              -- [1 2 3]
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
[4 5 6] => 'nums. -- defines an immutable variable called nums
nums              -- [4 5 6]
```

### `#`: `@[]`

Returns the length of the array.
The return type is `'u32`.

```fuzzy
[] #      -- 0
[1 2 3] # -- 3
```

### `^`: `@[]`

Returns the first element.
Returns `()` if the array is empty.

```fuzzy
[] ^           -- ()
[1 2 3] ^      -- 1
['foo "bar"] ^ -- 'foo
```

### `$`: `@[]`

Returns the last element.
Returns `()` if the array is empty.

```fuzzy
[] $           -- ()
[1 2 3] $      -- 3
['foo "bar"] ^ -- "bar"
```

### `@`: `@['i32]`

Returns the element at the specified index.
Returns `()` if the index is out of bounds.

```fuzzy
[4 5 6] @ 1   -- 5
[4 5 6] @ -1  -- 6
[4 5 6] @ 100 -- ()
```

### `@@`: `@['i32 '_]`

Replaces the element at the specified index with the object.

```fuzzy
[4 5 6] @@ 1 "hey" -- [4 "hey" 6]
```

### `@<`: `@['i32 '_]`

Inserts an element at the specified index.

```fuzzy
['foo "baz"] @< 1 "bar" -- ['foo "bar" "baz"]
```

### `@-`: `@['i32]`

Removes the element at the specified index.

```fuzzy
['foo "bar" "baz"] @- 1 -- ['foo "baz"]
```

### `$-`: `@[]`

Removes the last element.

```fuzzy
['foo "bar" "baz"] $- -- ['foo "bar"]
```

### `$>`: `@['_]`

Appends an element to the end.

```fuzzy
['foo "bar"] $> "baz" -- ['foo "bar" "baz"]
```

### `|>`: `@['symbol]`

Defines a user-defined type.
The subject must be an array that satisfies the following requirements:

- All leaf elements must be of type `'symbol'`
- Odd-numbered symbols represent member names
- Odd-numbered symbols must have a visibility prefix `:` or `::`
- Even-numbered elements represent member types
- If an even-numbered element is an array, it represents a function type

The defined user-defined type cannot be redefined until the scope is exited.

```fuzzy
[':foo 'i32. '::bar ['i32]] |> 'newtype. -- defines a user-defined type newtype
                                         -- with a public member foo of type 'i32
                                         -- and a private member bar of type @['i32]
```

### `:`: `@['symbol]`

Casts the subject to the user-defined type indicated by the object.
The subject must be an array that satisfies the following requirements:

- Odd-numbered symbols represent member names
- Odd-numbered symbols must have a visibility prefix `:` or `::`
- Even-numbered elements are member values

```fuzzy
[':foo 12 '::bar {#0 !!} : ['i32]] : 'newtype, -> 'var. -- defines a variable var of type 'newtype
var:foo                                                 -- 12
var:bar @ 1                                             -- error because bar is private
{ ##::bar @ 1 } : [], -> 'newtype:baz.                  -- define a function on newtype
var baz.                                                -- to access bar
                                                        -- outputs 1
```

### `==`: `@['[]]`

Checks if the subject is equal to the object.

```fuzzy
[1 + 2 "hoge"] == [2 + 1 "hoge"] -- T
[1 + 2 "hoge"] == [1 + 2 "hOge"] -- ()
```

### `!=`: `@['[]]`

Checks if the subject is not equal to the object.

```fuzzy
[1 + 2 "hoge"] != [2 + 1 "hoge"] -- ()
[1 + 2 "hoge"] != [1 + 2 "hOge"] -- T
```
