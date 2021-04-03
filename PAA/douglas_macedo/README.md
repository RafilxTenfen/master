# PILHA TDA
* Implementação de uma pilha com tamanho dinâmico e capacidade mínima para 5 elementos.

* Quando alcançado o topo, duplica sua capacidade.

* Quando desempilhar (POP) e sua capacidade for menor que a metade do tamanho, libera uma parte da memória que está em utilização. 

## PARA COMPILAR
* Recomendo utilizar o makefile. Exemplo "$ make".

## PARA EXECUTAR O EXERCÍCIO
* Foi incluído como exemplo um arquivo txt (lista_palavras.txt) com uma série de palavras. Este arquivo é utilizado no script 'main', fornecendo a lista de palavras para eventual empilhamento.

### MAKEFILE:
* ```make pilha```
		Compila somente a pilha.	

* ```make```
		Compila tudo.

* ```make clean```
		Limpa o diretório
