%option noyywrap

%top {
    #include <stdbool.h>
}

%{
    bool leave_comments = false;

    void line_comment();
    void multi_comment();

    const char begin_red[] = "\x1b[31m";
    const char end_red[] = "\x1b[0m";
%}

%x STR
%x POSSIBLECOMMENT
%x POSSIBLEESCAPED
%x LINECOMMENT
%x BANGLINECOMMENT
%x TRIPLECOMMENT
%x BANGMULTICOMMENT
%x MULTICOMMENT
%x JAVADOCCOMMENT

HexDigit        [0-9a-fA-F]

EscapeSingle    ["'abefnrtv\\]
EscapeOctal     [0-9]{3}
EscapeHex       x{HexDigit}+
Escapeu         u{HexDigit}{4}
EscapeU         U{HexDigit}{8}
EscapeInvalid   [^"'abefnrtuUvx0-9]
StrEscape       \\({EscapeOctal}|{EscapeSingle}|{EscapeHex}|{Escapeu}|{EscapeU}|{EscapeInvalid})

%%

[[:blank:]]*#include.*  ECHO;

\"                      ECHO; BEGIN(STR);
<STR>\"                 ECHO; BEGIN(INITIAL);
<STR>{StrEscape}        ECHO;
<STR>[^\\"]+            ECHO;

\/                                      BEGIN(POSSIBLECOMMENT);  yymore();
<POSSIBLECOMMENT,POSSIBLEESCAPED>\\\n   BEGIN(POSSIBLEESCAPED);  yymore();
<POSSIBLECOMMENT,POSSIBLEESCAPED>"/"    BEGIN(LINECOMMENT);      yymore();
<POSSIBLECOMMENT>"/!"                   BEGIN(BANGLINECOMMENT);  yymore();
<POSSIBLECOMMENT>"//"                   BEGIN(TRIPLECOMMENT);    yymore();
<POSSIBLECOMMENT,POSSIBLEESCAPED>"*"    BEGIN(MULTICOMMENT);     yymore();
<POSSIBLECOMMENT>"*!"                   BEGIN(BANGMULTICOMMENT); yymore();
<POSSIBLECOMMENT>"**"                   BEGIN(JAVADOCCOMMENT);   yymore();
<POSSIBLECOMMENT>.                      ECHO; BEGIN(INITIAL);

<LINECOMMENT,BANGLINECOMMENT,TRIPLECOMMENT>\n           line_comment();
<LINECOMMENT,BANGLINECOMMENT,TRIPLECOMMENT>\\\n         yymore();
<LINECOMMENT,BANGLINECOMMENT,TRIPLECOMMENT>[^\\\n]+     yymore();
<LINECOMMENT,BANGLINECOMMENT,TRIPLECOMMENT>.            yymore();

<MULTICOMMENT,BANGMULTICOMMENT,JAVADOCCOMMENT>\*+\/     multi_comment();
<MULTICOMMENT,BANGMULTICOMMENT,JAVADOCCOMMENT>\*+[^*/]  yymore();
<MULTICOMMENT,BANGMULTICOMMENT,JAVADOCCOMMENT>[^*]+     yymore();

%%

void line_comment()
{
    switch (YY_START)
    {
    case BANGLINECOMMENT:
    case TRIPLECOMMENT:
        if (leave_comments)
        {
            ECHO;
        }
        break;
    default:
        fprintf(yyout, "\n");
        break;
    }

    // fprintf(yyout, begin_red);
    // ECHO;
    // fprintf(yyout, end_red);
    // fprintf(yyout, "#!# line_comment %d #!#\n", YY_START);
    BEGIN(INITIAL);
}

void multi_comment()
{
    switch (YY_START)
    {
    case BANGMULTICOMMENT:
    case JAVADOCCOMMENT:
        if (leave_comments)
        {
            ECHO;
        }
        break;
    default:
        break;
    }

    // fprintf(yyout, begin_red);
    // ECHO;
    // fprintf(yyout, end_red);
    // fprintf(yyout, "#!# multi_comment %d #!#", YY_START);
    BEGIN(INITIAL);
}

int main(int argc, char *argv[])
{
    argc--; argv++;

    if (argc > 0 && strcmp(argv[0], "-leave") == 0)
    {
        leave_comments = true;
        argc--; argv++;
    }

    if (argc > 0)
        yyin = fopen(argv[0], "r");
    else
        yyin = stdin;
 
    if (argc > 1)
        yyout = fopen(argv[1], "w");
    else
        yyout = stdout;
 
    yylex();
}
