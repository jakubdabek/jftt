SHELL:=/bin/bash
CC=gcc
# CFLAGS=-Wall -pedantic -std=c11
LDFLAGS=-lfl -lm
FILES=1.out 2.out 3.out 4.out

all: $(FILES)

%.yy.c: %.lex
	flex -o $@ $^

%.out: %.yy.c
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

test: $(FILES)
	diff <(./1.out l2z1a.txt 2>/dev/null) l2z1a.r.txt
	diff <(./1.out l2z1b.txt 2>/dev/null) l2z1b.r.txt
	diff <(./4.out test-calc.txt 2>/dev/null) test-calc.out.txt

clean:
	$(RM) $(FILES) $(FILES:.out=.yy.c)

.PHONY: all clean test
.PRECIOUS: %.yy.c
