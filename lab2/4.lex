%option noyywrap

%x ERROR
%x ERRUNKNOWN

%{
#include <stdbool.h>

#define STACK_SIZE 1000

int ptr = -1;
int stack[STACK_SIZE];
char unknown = '\0';

const char begin_red[] = "\x1b[31m";
const char end_red[] = "\x1b[0m";

void execute(char op);
void finish();

void printerr(const char *message)
{
    fprintf(stderr, "%s%s%s\n", begin_red, message, end_red);
}

bool empty() { return ptr == -1; }

void push(int num);
int pop();

%}

%%

-?[0-9]+            push(atoi(yytext));
[+*/%^-]            execute(*yytext);
[[:blank:]]+        ;
<*>\n               finish();
.                   unknown = *yytext; BEGIN(ERRUNKNOWN);
<ERROR,ERRUNKNOWN>. ;

%%

void push(int num)
{
    if (++ptr < STACK_SIZE)
    {
        stack[ptr] = num;
    }
    else
    {
        printerr("Error: stack overflow");
        BEGIN(ERROR);
    } 
}

int pop()
{
    if (ptr >= 0)
    {
        return stack[ptr--];
    } 
    else
    {
        BEGIN(ERROR);
        return 0;
    }
}

int mypow(int a, int b)
{
    int res = 1;
    for ( ; b--; )
        res *= a;
    return res;
}

void execute(char op)
{
    int b = pop();
    if (empty())
    {
        printerr("Error: too few arguments");
        goto error;
    }

    int a = pop();

    switch(op)
    {
    case '+':
        push(a + b);
        break;
    case '-':
        push(a - b);
        break;
    case '*':
        push(a * b);
        break;
    case '/':
        if (b == 0)
        {
            printerr("Error: division by 0 is not permitted");
            goto error;
        }
        push(a / b);
        break;
    case '%':
        if (b == 0)
        {
            printerr("Error: division by 0 is not permitted");
            goto error;
        }
        push(a % b);
        break;
    case '^':
        if (b < 0)
        {
            printerr("Error: negative exponent not permitted");
            goto error;
        }
        push(mypow(a, b));
        break;
    }

    return;

error:
    BEGIN(ERROR);
}

void reset()
{
    ptr = -1;
    BEGIN(INITIAL);
}

void finish()
{
    if (YY_START == ERRUNKNOWN)
    {
        char buf[] = "Error: unknown character: ' '";
        buf[sizeof(buf) - 3] = unknown;
        printerr(buf);
    }
    else if (YY_START != ERROR)
    {
        if (empty())
        {
            printerr("Error: empty line");
        }
        else if (ptr > 0)
        {
            printerr("Error: too few operators");
        }
        else
        {
            printf("= %d\n", pop());
        }
    }

    reset();
}

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
