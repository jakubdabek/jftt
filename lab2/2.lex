
WS                  [[:blank:]\n]+

COMMENT_BEGIN       "<!--"
COMMENT_END         "-->"
COMMENT             {COMMENT_BEGIN}([^-]|-[^-])*{COMMENT_END}

CDStart             "<![CDATA["
CData               ([^\]]|"]"[^\]]|"]"{2,}[^\]>])*
CDEnd               "]"{2,}">"
CDSect              {CDStart}{CData}{CDEnd}

NameChar            [[:alnum:]._:-]
Name                [[:alpha:]_:]{NameChar}*

    /* specification says that it should be [^"<] below */
AttValue            \"[^"]*\"|\'[^']*\'
Attribute           {Name}={AttValue}
ETag                '</'{Name}{WS}?'>'

%x TAG
%x SCRIPT
%x SCRIPTTAG
%x STR

%{
    int tag_start;
%}

%%

{CDSect}            fprintf(yyout, "##CDATA##");
{COMMENT}           fprintf(yyout, "##COMMENT##");

"<script"                                   { ECHO; BEGIN(SCRIPTTAG); }
"<"{Name}                                   { ECHO; BEGIN(TAG); }
<TAG,SCRIPTTAG>({WS}{Attribute})*{WS}?      ECHO;
<TAG,SCRIPTTAG>"/"?">"                     { 
                                                ECHO; 
                                                if (YY_START == TAG)
                                                    BEGIN(INITIAL);
                                                else
                                                    BEGIN(SCRIPT);
                                            }
<SCRIPT>"</script"{WS}?>                    { ECHO; BEGIN(INITIAL); }

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
}

