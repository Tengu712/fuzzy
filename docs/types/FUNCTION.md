# function

## General

A function type is represented by an array listing `'symbol` values or function types that represent argument types.
In the REPL and this documentation, it is displayed as `@[argument1 argument2 ...]`.

## Functions

### `->`: `@['symbol]`

Defines a mutable variable or mutable function.
If the object takes the form `'typename:functionname`, it defines `functionname` on `'typename`.
Returns `()`.

```fuzzy
{ 1 + 2 } : [], -> 'f.   -- defines a mutable variable called f
{ 1 + 2 } : [], -> 't:f. -- defines a mutable function f on t
```

### `=>`: `@['symbol]`

Defines an immutable variable or immutable function.
If the object takes the form `'typename:functionname`, it defines `functionname` on `'typename`.
Returns `()`.

```fuzzy
{ 1 + 2 } : [], => 'f.   -- defines an immutable variable called f
{ 1 + 2 } : [], => 't:f. -- defines an immutable function f on t
```

### `@`: `@[ARGUMENT1 ARGUMENT2 ...]`

Evaluates the function with the objects as arguments.

```fuzzy
{ 1 + 2 } : [], -> 'f.
f @                               -- 3
{ #0 + #1 } : ['i32 'i32], -> 'f.
f @ 1 2                           -- 3
```
