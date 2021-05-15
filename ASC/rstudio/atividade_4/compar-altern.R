#' Gera um data frame para representação gráfica de ICs.
#'
#' A função processa um data frame onde cada coluna representa uma variável diferente, 
#' e gera um outro data frame que permite plotar os ICs das variáveis.
#' @param df Data frame de entrada, onde cada coluna é uma variável diferente.
#' @param conf.level Nível de confiança para o cálculo do IC.
#' @param pairs Indica se as variáveis devem ser tomadas aos pares ou não.
#'
#' @return Retorna um data frame com as seguintes colunas:
#' \itemize{
#'   \item x.  Posição no eixo x.
#'   \item nome.var.  Nome da variável (coluna do data frame de entrada).
#'   \item media.  Média das observações.
#'   \item ic.  Largura do intervalo de confiança.
#'   \item grupo.  Indica o agrupamento das variáveis.
#' }
geraIC <- function(df, conf.level=0.95, pairs=TRUE) {
  media <- c()
  ic <- c()
  grupo <- c()
  xpos <- c()
  i <- 1
  gr <- 1
  x <- 1
  for (nc in colnames(df)) {
    tt <- t.test(df[, nc], conf.level=conf.level)
    media[i] <- as.numeric(tt$estimate)
    ic[i] <- media[i] - tt$conf.int[1]
    grupo[i] <- gr
    xpos[i] <- x
    
    ## quando os ICs estão aos pares, muda o grupo e introduz um espaço no eixo
    ## x a cada 2 colunas
    if (pairs & (i %% 2 == 0)) {  
      gr <- gr + 1    
      x <- x + 1
    }
    x <- x + 1
    i <- i + 1
  }
  return(data.frame(x=xpos, nome.var=colnames(df), media, ic, grupo=as.factor(grupo)))
}


#' Gera um plot dos intervalos de confiança de diversas variáveis.
#'
#' @param df Data frame de entrada, com as seguintes colunas:
#' \itemize{
#'   \item x.  Posição no eixo x.
#'   \item nome.var.  Nome da variável (coluna do data frame de entrada).
#'   \item media.  Média das observações.
#'   \item ic.  Largura do intervalo de confiança.
#'   \item grupo.  Indica o agrupamento das variáveis.
#' }
#' @param usa.grupo Indica se a coluna grupo deve ser usada ou não.
#' @param show Indica se o gráfico deve ser plotado ou não.
plotaIC <- function(df, usa.grupo=TRUE, show=TRUE) {
  require(ggplot2)
  if (usa.grupo) {
    graf.ic <- ggplot(df, aes(x=x, y=media, colour=grupo)) + 
      geom_point(size=2) + 
      geom_errorbar(aes(ymin=media-ic, ymax=media+ic), width=0.1) + 
      scale_x_continuous(breaks=df$x, labels=df$nome.var, name="") + 
      scale_y_continuous(name="IC para a média") + 
      theme_bw() + 
      theme(legend.position = "none", panel.grid.minor.x=element_blank(), 
            panel.grid.major.x=element_blank())
  } else {
    graf.ic <- ggplot(df, aes(x=x, y=media)) + 
      geom_point(size=2) + 
      geom_errorbar(aes(ymin=media-ic, ymax=media+ic), width=0.1) + 
      scale_x_continuous(breaks=df$x, labels=df$nome.var, name="") + 
      scale_y_continuous(name="IC para a média") + 
      theme_bw() + 
      theme(legend.position = "none", panel.grid.minor.x=element_blank(), 
            panel.grid.major.x=element_blank())
  }
  if (show)
    graf.ic
  return(graf.ic)
}
