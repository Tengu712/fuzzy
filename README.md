# Fuzzy, a joke programming language.

Fuzzy is a kind of joke programming language.

- Infix notation
- Built-in function names consisting only of symbols
- Obscure grammar uniquely determined by somewhat strong typing

```fuzzy
-- FizzBuzz. --
1 -> 'i.
{ i <= 100 } %% {
    i % 3, == 0 >> { "Fizz" ! }
    i % 5, == 0 >> { "Buzz" ! }
    i % 3, == 0 || i % 5, == 0; >> { "" !! } !> { i !! }
    i + 1, -> 'i.
}
```
