%{
    int line_count = 0;
    int word_count = 0;
%}

%%

^[[:blank:]]*\n     ;
\n                  ECHO; line_count++;
^[[:blank:]]+       ;
[[:blank:]]+$       ;
[ \t]+              fprintf(yyout, " ");
[^[:blank:]\n]+     ECHO; word_count++;
 
%%
 
int main(int argc, char *argv[])
{
    argc--; argv++;
    if (argc > 0)
        yyin = fopen(argv[0], "r");
    else
        yyin = stdin;
 
    if (argc > 1)
        yyout = fopen(argv[1], "w");
    else
        yyout = stdout;
 
    yylex();
    printf("lines: %d words: %d\n", line_count, word_count);
}

