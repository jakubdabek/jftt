#include "test/*asdf*/header.h"
#include <stdio.h>
    #include <foo/*bar*/baz.h>

int grumple();

/** \brief Java style Doc String - Foo function */
int foo();

int bar(); /**< Bar function */

/// .NET Style Doc String
int g_global_var = 1;

int asdf; // wow
int qwerty; // wowwww

/* Hello
/* World
// */
int baz();
// */

/*! Global variable
 *  ... */
volatile int g_global;

//! Main
int main(int argc, const char *argv[])
{
    printf("/* fo\o \\\" \x69\r\n\t\v\'\"\e/*bar");
    //*/ bar();

    // \
    continuation \
    /*
    baz();
    /*/
    foo();
    //*/

/\
\
\
/*
    grumple();
/*/
    foo();
//*/

    return 1;
}

// wwww /*ww
/*ggggggggggggggggggg */

//! jjj \
ababab \gg
/\
/\
/\
hhhh
/\
/!llllllll\
hhh

void xd() { qwer(); }
