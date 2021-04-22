CC=gcc
FLAGS=-Wall -g

default: app

pilha.o: pilha.c pilha_interface.h pilha_privado.h
	$(CC) $(FLAGS) -c pilha.c -o pilha.o

fila.o: fila.c fila_interface.h fila_privado.h
	$(CC) $(FLAGS) -c fila.c -o fila.o

fila_pilha.o: fila_pilha.c fila_pilha_interface.h fila_pilha_privado.h
	$(CC) $(FLAGS) -c fila_pilha.c -o fila_pilha.o

app: main.c main.h pilha.o fila.o fila_pilha.o  
	$(CC) $(FLAGS) main.c pilha.o fila.o fila_pilha.o -o app

.PHONY: clean

clean:
	rm -f *.o app