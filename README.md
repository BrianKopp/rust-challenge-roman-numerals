# Roman Numerals

## Problem Statement

Given an integer, write a function to return its roman numeral representation.

## Background

Roman numerals is a system which uses characters instead of
arabic numerals to represent characters. E.g. `1` is represented `I`,
`2` is represented `II`, `10` is represented `X`.

### Rules

Symbols in equal or decreasing value are additive. E.g.
`VI=V+I=5+1=6`.

Symbols in increasing value are subtractive, e.g.
`IX=X-I=10-1=9`.

Do not repeat characters more than 3 times, unless the character
is `M` for `1000`, since that is the highest number that
will be represented.

### Symbols

* `I` - `1, one`
* `V` - `5, five`
* `X` - `10, ten`
* `L` - `50, fifty`
* `C` - `100, one-hundred`
* `D` - `500, five-hundred`
* `M` - `1000, one-thousand`
