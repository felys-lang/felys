# Felys Grammar

This is the Felys grammar file - notation only.

## Item

```text
fn -> 'fn' ident '(' ','.(ident ':' ty)* ')' ('->' ty)? block

struct -> \
    \ 'struct' ident '{' ','.(ident ':' ty)+ '}'
    \ 'struct' ident '(' ','.ident+ )'
    \ 'struct' ident

impl -> 'impl' ident '{' (method \ fn)* '}'

method -> 'fn' ident '(' 'self' (',' ident ':' ty)* ')' ('->' ty)? block
```

## Statement

```text
stmt -> \
    \ expr ';'
    \ expr
    \ ';'

block -> '{' stmt* '}'
```

## Expression

```text
expr -> \
    \ assign
    \ tuple
    \ block
    \ 'break' expr?
    \ 'continue'
    \ 'if' block ('else' expr)?
    \ 'loop' block
    \ 'return' expr?
    \ 'while' expr block

assign -> \
    \ pat '=' expr
    \ pat '+=' expr
    \ pat '-=' expr
    \ pat '*=' expr
    \ pat '/=' expr
    \ pat '%=' expr

tuple -> \
    \ '(' ','.expr+ ')'
    \ disjunction

disjunction -> \
    \ disjunction 'or' conjunction
    \ conjunction

conjunction -> \
    \ conjunction 'and' inversion
    \ inversion

inversion -> \
    \ 'not' inversion
    \ equality

equality -> \
    \ equality '==' comparison
    \ equality '!=' comparison
    \ comparison

comparison -> \
    \ comparison '>=' term
    \ comparison '<=' term
    \ comparison '>' term
    \ comparison '<' term
    \ term

term -> \
    \ term '+' factor
    \ term '-' factor
    \ factor

factor -> \
    \ factor '*' unary
    \ factor '/' unary
    \ factor '%' unary
    \ unary

unary -> \
    \ '+' unary
    \ '-' unary
    \ evaluation
    
evaluation -> \
    \ evaluation '(' ','.expr* ')'
    \ evaluation '.' ident
    \ primary

primary -> \
    \ path
    \ lit
    \ '(' expr ')'
    
path -> \
    \ path '::' ident
    \ ident
```

## Pattern

```text
pat -> \
    \ ident '{' ','.pat+ '}'
    \ ident '(' ','.pat+ ')'
    \ '(' ','.pat+ ')'
    \ ident
    \ lit
    \ '_'
    
ident -> (alphabetic \ '_') (alphanumeric \ '_')*
```

## Typing

```text
ty -> \
    \ 'float'
    \ 'int'
    \ 'str'
    \ 'bool'
    \ 'fn' '(' ','.ty* )' ('->' ty)?
    \ ident
```

## Literal

```text
lit -> \
    \ float
    \ int
    \ str
    \ bool

float -> \
    \ '0' '.' digit+
    \ digit+ '.' digit+

int -> \
    \ '0x' hex+
    \ '0o' oct+
    \ '0b' bin+
    \ '0' !digit
    \ digit+

bool -> \
    \ 'true'
    \ 'false'

str -> '"' (!'"' char)* '"'
```
