SHELL:=/bin/bash
CC=gcc
CFLAGS=-Wall -pedantic -std=gnu11
LDFLAGS=-lfl -lm
FILES=main.o calc.yy.o calc.tab.o

all: main

main: $(FILES)

main.o: result.h

%.tab.c %.tab.h: %.y
	bison -d $^

%.yy.c: %.lex %.tab.h
	flex -o $@ $<

%.out: %.yy.c
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

test: main
	diff <(./main < test1.txt) test1.out.txt
	diff <(./main < test2.txt) test2.out.txt
	diff <(./main < test3.txt) test3.out.txt

clean:
	$(RM) main *.o *.tab.h *.tab.c *.yy.c

.PHONY: all clean test
.PRECIOUS: %.yy.c %.tab.h
