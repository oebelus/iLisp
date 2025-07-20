# iLisp

## Grammar

```md
number ::= <integer> | <float>
atom ::= <number> | <string> | <boolean>

s_expression ::= atom | "(" <s_expression> "." <s_expression> ")"
```

The second expression would be in terms of parsers combinator:

```md
atom | "(" zero_or_more(<s_expression>) ")"
```
