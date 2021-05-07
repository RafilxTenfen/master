#' Gera números aleatórios até encontrar 171 diversas vezes.
#' 
#' Executa até encontrar \code{noc} ocorrências do valor 171 em uma sequência de 
#' números aleatórios inteiros entre 1 e \code{nmax}. 
#' 
#' @param noc Número de ocorrências.
#' @param nmax Limite superior dos números aleatórios.
#' @param verbose Imprime o tempo de execução quando \code{TRUE}.
#' @param debug Imprime informações de depuração.
#' @return Retorna o tempo de execução em segundos.
#' 
rand171 <- function(noc=20, nmax=50000, verbose=TRUE, debug=FALSE) {
  tini <- Sys.time()
  v <- 1:nmax
  cont <- 0
  neval <- 0
  while (cont < noc) {
    neval <- neval + 1
    x <- sample(v, 1)
    if (x == 1)
      cont <- cont + 1
  }
  tfim <- Sys.time()
  texec <- difftime(tfim, tini, units="secs")
  if (debug) 
    cat(sprintf("noc=%d\tnmax=%d\tneval=%d\n", noc, nmax, neval))
  if (verbose) 
    cat(sprintf("tempo de execucao=%.3f s\n", texec))
  
  return(as.numeric(texec))
}
