# iLisp

This is an implementation of a simple Lisp interpreter in Rust. It consists of a parser and an interpreter.

The parser used is a parser combinator which I implemented too, but only for strings.

## Grammar

```md
atom ::= <number> | <string> | <boolean>
s_expression ::= atom | "(" <s_expression> "." <s_expression> ")"
```

## Features

- Binary Operations:

```lisp
(* (+ 5 4) 2) (+ 3 1)
```

- Unary Operations:

```lisp
(- 5)
(! true)
(! false)
```

- Logical Operators:

```lisp
(| (true false))
(& (true false))
```

- Comparison:

```lisp
(> 5 4)
(== 5 5)
(<= 8 4)
(<= (-5) 4)
```

- Conditional:

```lisp
(if (> 5 4) (+ 5 4) (- 5 4))
```

- Function definition:

```lisp
(define mul (x y) (* x y))
(define add (x y) (+ x y))
```

- Function calls:

```lisp
(add 5 (mul 5 4))
```

## To-Dos

- [ ] Format
- [ ] Float
- [ ] REPL
- [ ] More tests
