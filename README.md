# iLisp

## Grammar

```md
atom ::= <digit> | <char> | <boolean>

s_expression ::= atom | "(" <s_expression> "." <s_expression> ")"

list ::= "(" "list"  <s_expression> ")"
```
