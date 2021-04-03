CC=gcc
CFLAGS=-g

default: main

pilha: Pilha.c Pilha_interface.h
	$(CC) $(CFLAGS) -c Pilha.c -o Pilha.o

main: Pilha_interface.h Pilha.o main.c
	$(CC) $(CFLAGS) main.c  Pilha.o -o main
	
all: main

clean:
	rm -f main
	rm *.o                
