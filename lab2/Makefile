CC=gcc
FILES=1.out 2.out

all: $(FILES)

%.yy.c: %.lex
	flex -o $@ $^

%.out: %.yy.c
	$(CC) -o $@ $^ -lfl

clean:
	$(RM) $(FILES) $(FILES:.out=.yy.c)

.PHONY: all
.PRECIOUS: %.yy.c
