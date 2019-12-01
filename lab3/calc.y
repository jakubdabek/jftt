%{

#include "result.h"

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int yylex(void);
int yyerror(const char*);

int mypow(int a, int exp);
int mydiv(int a, int b);
int mymod(int a, int b);


void save_unary(struct result *a, char op, int value, struct result *result);
void save_binary(struct result *a, struct result *b, char op, int value, struct result *result);

void take_result(struct result*);

%}

/* bison declarations */
%define api.value.type {struct result}
%token NUM
%left '-' '+'
%left '*' '/' '%'
%precedence NEG   /* negation - unary minus */
%right '^'        /* exponentiation */

%destructor { free($$.polish); } exp

%%

input:
  %empty
| input line
;

line:
    '\n'
    | exp '\n'      { take_result(&$1); }
    | error '\n'    { ; }
    ;

exp:
    NUM
    | exp '+' exp           { save_binary(&$1, &$3, '+', $1.value + $3.value, &$$); }
    | exp '-' exp           { save_binary(&$1, &$3, '-', $1.value - $3.value, &$$); }
    | exp '*' exp           { save_binary(&$1, &$3, '*', $1.value * $3.value, &$$); }
    | exp '/' exp           { save_binary(&$1, &$3, '/', mydiv($1.value, $3.value), &$$); }
    | exp '%' exp           { save_binary(&$1, &$3, '%', mymod($1.value, $3.value), &$$); }
    | '-' exp  %prec NEG    { save_unary(&$2, '~', -$2.value, &$$);                 }
    | exp '^' exp           { save_binary(&$1, &$3, '^', mypow($1.value, $3.value), &$$); }
    | '(' exp ')'           { $$ = $2; }
    ;

%%

int mypow(int a, int exp)
{
    int res = 1;
    for ( ; exp--; )
        res *= a;
    return res;
}

int mydiv(int a, int b)
{
    div_t divmod = div(a, b);

    if (divmod.rem == 0)
        return divmod.quot;
    else if (divmod.rem < 0)
        return divmod.quot - 1;
    else
        return divmod.quot;
}

int mymod(int a, int b)
{
    div_t divmod = div(a, b);

    if (divmod.rem >= 0)
        return divmod.rem;
    else
        return b + divmod.rem;
}

void save_unary(struct result *a, char op, int value, struct result *result)
{
    char buf[64];
    snprintf(buf, 64, "%s %c", a->polish, op);

    result->value = value;
    result->polish = strndup(buf, 64);

    free(a->polish);
}

void save_binary(struct result *a, struct result *b, char op, int value, struct result *result)
{
    char buf[64];
    snprintf(buf, 64, "%s %s %c", a->polish, b->polish, op);

    result->value = value;
    result->polish = strndup(buf, 64);

    free(a->polish);
    free(b->polish);
}
