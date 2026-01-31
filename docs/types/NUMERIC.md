# numeric

## General

Numeric types include the following 3 categories with 12 types in total:

| Category | Types |
| --- | --- |
| Signed integers | `'i8`, `'i16`, `'i32`, `'i64`, `'i128` |
| Unsigned integers | `'u8`, `'u16`, `'u32`, `'u64`, `'u128` |
| Floating-point numbers | `'f32`, `'f64` |

## Functions

### `!`: `@[]`

Outputs to standard output without a newline.
Returns the subject.

```fuzzy
12 !     -- outputs 12
1.2f32 ! -- outputs 1.2
```

### `!!`: `@[]`

Outputs to standard output with a newline.
Returns the subject.

```fuzzy
12 !     -- outputs 12
         -- with newline
1.2f32 ! -- outputs 1.2
         -- with newline
```

### `->`: `@['symbol]`

Defines a mutable variable.
Returns `()`.

```fuzzy
42 -> 'num. -- defines a mutable variable called num
num         -- 42
```

### `=>`: `@['symbol]`

Defines an immutable variable.
Returns `()`.

```fuzzy
3.14f32 => 'pi. -- defines an immutable variable called pi
pi              -- 3.14
```

### `:`: `@['symbol]`

Performs type casting.

```fuzzy
42 : 'f64      -- 42.0
3.14f32 : 'i32 -- 3
```

### `+`: `@[SAME-AS-SUBJECT]`

Returns the sum of the subject and the object.
The object must be of the same type as the subject.

```fuzzy
1 + 2         -- 3
3.f32 + .5f32 -- 3.5
1 + 2.0f32    -- error
```

### `-`: `@[SAME-AS-SUBJECT]`

Returns the difference of the subject and the object.
The object must be of the same type as the subject.

```fuzzy
1 - 2         -- -1
3.f32 - .5f32 -- 2.5
1 - 2.0f32    -- error
```

### `*`: `@[SAME-AS-SUBJECT]`

Returns the product of the subject and the object.
The object must be of the same type as the subject.

```fuzzy
1 * 2         -- 2
3.f32 * .5f32 -- 1.5
1 * 2.0f32    -- error
```

### `/`: `@[SAME-AS-SUBJECT]`

Returns the quotient of the subject and the object.
The object must be of the same type as the subject.

```fuzzy
1 / 2         -- 0
3.f32 / .5f32 -- 6
1 / 2.0f32    -- error
```

### `%`: `@[SAME-AS-SUBJECT]`

Returns the remainder of the subject and the object.
Only defined for integer types, and the object must be of the same type as the subject.

```fuzzy
10 % 3       -- 1
10f32 % 3f32 -- error
```

### `<`: `@[SAME-AS-SUBJECT]`

Checks if the subject is less than the object.
The object must be of the same type as the subject.

```fuzzy
1 < 2       -- T
2 < 2       -- ()
3f32 < 2f32 -- ()
```

### `<=`: `@[SAME-AS-SUBJECT]`

Checks if the subject is less than or equal to the object.
The object must be of the same type as the subject.

```fuzzy
1 <= 2       -- T
2 <= 2       -- T
3f32 <= 2f32 -- ()
```

### `>`: `@[SAME-AS-SUBJECT]`

Checks if the subject is greater than the object.
The object must be of the same type as the subject.

```fuzzy
1 > 2       -- ()
2 > 2       -- ()
3f32 > 2f32 -- T
```

### `>=`: `@[SAME-AS-SUBJECT]`

Checks if the subject is greater than or equal to the object.
The object must be of the same type as the subject.

```fuzzy
1 >= 2       -- ()
2 >= 2       -- T
3f32 >= 2f32 -- T
```

### `==`: `@[SAME-AS-SUBJECT]`

Checks if the subject is equal to the object.
The object must be of the same type as the subject.

```fuzzy
1 == 2       -- ()
2 == 2       -- T
3f32 == 2f32 -- ()
```

### `!=`: `@[SAME-AS-SUBJECT]`

Checks if the subject is not equal to the object.
The object must be of the same type as the subject.

```fuzzy
1 != 2       -- T
2 != 2       -- ()
3f32 != 2f32 -- T
```
