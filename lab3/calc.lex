%option noinput
%option nounput

%{
    #include "calc.tab.h"
    #include "result.h"

    #include <string.h>
%}

%x COMMENT

Number  [0-9]+

%%

"\n"                return '\n';
"+"                 return '+';
"-"                 return '-';
"*"                 return '*';
"/"                 return '/';
"%"                 return '%';
"^"                 return '^';
"("                 return '(';
")"                 return ')';
{Number}            yylval.value = atoi(yytext); yylval.polish = strdup(yytext); return NUM;

"\\\n"              ;
[[:blank:]]+        ;

^"#"                BEGIN(COMMENT);
<COMMENT>"\n"       BEGIN(INITIAL);
<COMMENT>.          ;
<COMMENT>"\\\n"     ;

.                   return *yytext;

%%
