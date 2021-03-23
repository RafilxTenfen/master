# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall -O3

default: aplicacao

Stack.o: Stack.c Stack_Public.h Stack_Private.h
	$(CC) $(CFLAGS) -c Stack.c -o Stack.o

aplicacao: aplicacao.c Stack.o
	$(CC) $(CFLAGS) aplicacao.c Stack.o -o aplicacao

clean:
	rm -f aplicacao *.o core a.out *.*~ Makefile~

all: aplicacao