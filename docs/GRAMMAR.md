# Grammar

## Comment

Everything from `--` to the end of the line is treated as a comment.

## Sentence

A Fuzzy sentence consists of 0 or 1 subject, 0 or 1 verb, and 0 or more objects.
Subjects and objects are either sentences or values.
Each component is separated by spaces or newlines.

For example, the following sentence:

```fuzzy
2 * 3 + 4
```

Is parsed into this tree:

- Subject: 2
- Verb: *
  - Subject: 3
  - Verb: +
    - Subject: 4

The Fuzzy interpreter evaluates from the innermost sentence outward.
Therefore, the entire sentence above evaluates to 14.
However, in conventional sense, `2 * 3 + 4` should evaluate to 10.
Fuzzy provides `,` and `;` to manipulate sentence structure.

| Symbol | Meaning |
| --- | --- |
| `,` | Makes the current hierarchy at this point the subject |
| `;` | Makes the entire sentence at this point the subject |

Using `,`, you can write the following to evaluate to 10:

```fuzzy
2 * 3, + 4
```

`;` is useful when the hierarchy becomes deep:

```fuzzy
-- To take 2 * 3 + 4 as a subject, you'd need two commas,
2 * 3 + 4,, !!
-- but with ; you can take the whole thing as a subject.
2 * 3 + 4; !!
```

If no subject exists, it evaluates to `()`.

```fuzzy
, !> { "hello" !! } -- hello
```

Fuzzy terminates the sentence at the point where the subject doesn't have the following verb.
You can also explicitly terminate a sentence with `.`.

```fuzzy
-- If subject foo can take bar as a verb, foo bar forms a sentence.
foo bar
-- If subject foo bar can take baz as a verb, foo bar baz forms a sentence.
foo bar baz
-- By indicating sentence end with ., baz is treated as a subject.
foo bar. baz
```

## Block

Fuzzy has three types of blocks.
Blocks collect 0 or more sentences and become subjects.

| Type | Delimiter | Evaluation Result |
| --- | --- | --- |
| Immediate block | `()` | Result of the last sentence |
| Deferred block | `{}` | Result of the last sentence |
| Array block | `[]` | Array collecting all sentences |

```fuzzy
-- Immediate block behavior
()     -- ()
(1 2)  -- 2
(1 2.) -- ()

-- Deferred block behavior
{} %     -- ()
{1 2} %  -- 2
{1 2.} % -- ()

-- Array block and deferred block behavior
[]     -- []
[1 2]  -- [1 2]
[1 2.] -- [1 2 ()]
```
