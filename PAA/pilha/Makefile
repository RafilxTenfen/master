# the compiler: gcc for C program, define as g++ for C++
CC = gcc

# compiler flags:
#  -g    adds debugging information to the executable file
#  -Wall turns on most, but not all, compiler warnings
CFLAGS  = -g -Wall -O3

default: aplicacao

pilha.o: pilha.c pilha_publico.h pilha_privado.h
	$(CC) $(CFLAGS) -c pilha.c -o pilha.o

aplicacao: aplicacao.c aplicacao.h pilha.o
	$(CC) $(CFLAGS) aplicacao.c pilha.o -o aplicacao

clean:
	rm -f aplicacao *.o core a.out *.*~ Makefile~

all: aplicacao