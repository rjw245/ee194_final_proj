all: simple

simple.o:
	gcc -c simple.c

libsimple.a: simple.o
	ar rcs libsimple.a simple.o

simple: simple.o libsimple.a
	rustc simple.rs

clean:
	rm simple.o libsimple.a simple
