# tempos <- data.frame(
#   n = c(1:20, 22),
#   media = c(
#     0.5,
#     1.0,
#     0.5,
#     1.5,
#     2.0,
#     2.0,
#     2.5,
#     1.5,
#     1.3,
#     0.5,
#     1.0,
#     1.5,
#     2.0,
#     1.0,
#     0.2,
#     2.5,
#     0.5,
#     1.0,
#     1.5,
#     2.0,
#     20.0
#   ),
#   desvio = c(
#     0.100,
#     0.300,
#     1.000,
#     0.525,
#     2.000,
#     0.300,
#     0.750,
#     1.500,
#     1.600,
#     0.075,
#     0.350,
#     0.225,
#     0.700,
#     1.000,
#     0.400,
#     0.875,
#     0.150,
#     0.250,
#     0.375,
#     0.600,
#     0.001
#   ),
#   distrib = c(
#     'n',
#     'n',
#     'u',
#     'n',
#     'e',
#     'n',
#     'n',
#     'e',
#     'u',
#     'n',
#     'n',
#     'n',
#     'n',
#     'e',
#     'u',
#     'n',
#     'n',
#     'n',
#     'n',
#     'n',
#     'n')
#   )

# le os tempos para cada ID (em ms por compatibilidade com o codigo C)
tempos <- read.table("tempos-cgim.txt", head=T)
# converte os tempos para segundos
tempos$media <- tempos$media / 1000
tempos$desvio <- tempos$desvio / 1000

func <- function(id) {
  param <- tempos[tempos$n == id, ]
  # se o id for inexistente, atribui valores default
  if (nrow(param) == 0) {
    param = data.frame(n=id, media=1, desvio=0.2, distrib='n')
  }
  if (param$distrib == 'n')
    tempo.sleep <- rnorm(1, mean=param$media, sd=param$desvio)
  else if ((param$distrib == 'e') && (param$media != 0))
    tempo.sleep <- rexp(1, rate=1/param$media)
  else if (param$distrib == 'u')
    tempo.sleep <- runif(1, param$media, param$desvio)
  else
    tempo.sleep <- 0
  Sys.sleep(tempo.sleep)
}
