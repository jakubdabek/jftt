#include "result.h"

#include <stdio.h>
#include <stdlib.h>


int yyparse(void);

void yyerror(const char *msg)
{
    printf("%s\n", msg);
}

void take_result(struct result *res)
{
    printf("%s\n= %d\n", res->polish, res->value);
    free(res->polish);
}

int main()
{
    // printf("-4/3=%d, 4/-3= %d, (int)-1.6=%d\n", -4/3, 4/-3, (int)-1.6);
    yyparse();
}
