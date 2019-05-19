[![Build Status](https://travis-ci.com/KevinCybura/K_lang.svg?branch=master)](https://travis-ci.com/KevinCybura/K_lang)


```
program          : [[statement | expression] Delimiter ? ]*;
statement        : [declaration | definition];
declaration      : Extern prototype;
definition       : Def prototype expression;
prototype        : Ident OpeningParenthesis [Ident Comma ?]* ClosingParenthesis;
expression       : [binary_expr | unary_expr];
binary_expr      : [unary_expr (Op unary_expr)* ];
unary_expr       : [( "!" | "-")? primary_expr];
primary_expr     : [Ident | Number | call_expr | parenthesis_expr];
call_expr        : Ident OpeningParenthesis [expression Comma ?]* ClosingParenthesis;
parenthesis_expr : OpeningParenthesis expression ClosingParenthesis;
```
