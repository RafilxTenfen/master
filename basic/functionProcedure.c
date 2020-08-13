#include <stdio.h>
#include <stdlib.h>

int funcao(int par1, float par2)
{
  int var_local;
  var_local = par1 * (int)par2;
  return var_local;
}

void procedimento(int par1, float par2)
{
  int var_local;
  var_local = par1 * (int)par2;
  printf("(procedimento) var_local = %d\n", var_local);
}

int main(int argc, char **agrv)
{ // main function
  if (argc != 3)
  {
    printf("Erro, numero incorreto de parametros\n");
    return 1; // inform deu ruim
  }

  printf("(main) retorno funcao = %d\n", funcao(atoi(agrv[1]), atof(agrv[2])));
  procedimento(atoi(agrv[1]), atof(agrv[2]));

  return 0;
}