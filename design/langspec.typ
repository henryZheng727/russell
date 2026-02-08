== Syntax
Comments start from a `//` and occur until a newline character.

Russell has the following primitive data:
```
<integer> ::= [0-9]+
<float>   ::= [0-9]+.[0-9]+
<bool>    ::= true
            | false
<id>      ::= [a-z]+[a-z0-9_]*
```

Russell has the following keywords:
```
echo
else
fn
if
let
match
read
return
then
typedef
```

...and the following type keywords:
```
u64
i64
f64
bool
```

Russell has the following reserved symbols:
```
!
!=
&&
(
)
*
+
,
-
/
;
<
<=
==
>
>=
{
|>
||
}
```

== Grammar
A Russell program is a list of definitions, and obeys the following grammar.
```
<defn> ::= typedef <id> { <id> ( <binding> , ... ) , ... };
         | fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }

<stmnt> ::= let <id> = <exp>;
          | read <type> <id>;
          | echo <type> <exp>;
          | return <exp>;

<exp> ::= <integer>
        | <float>
        | <bool>
        | <id>
        | fn ( <binding> ) -> <exp>
        | - <exp>
        | ! <exp>
        | <exp>(<exp>)
        | <exp> + <exp>
        | <exp> - <exp>
        | <exp> * <exp>
        | <exp> / <exp>
        | <exp> |> <exp>
        | <exp> < <exp>
        | <exp> <= <exp>
        | <exp> > <exp>
        | <exp> >= <exp>
        | <exp> == <exp>
        | <exp> != <exp>
        | <exp> && <exp>
        | <exp> || <exp>
        | if <exp> then <exp> else <exp>
        | ( <exp> )
        | match <exp> { <id> ( <binding> , ...) -> <exp> , ... }

<type> ::= u64
         | i64
         | f64
         | bool
         | <type> -> <type>

<binding> ::= <id> : <type>
```
