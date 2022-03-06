---
title: "Exercicios"
output: html_document
---

```{r setup, include=TRUE}
temposCPU = c(0.74, 0.43, 0.24, 2.24, 262.08, 8960, 4720, 19740, 7360, 22440, 28560)
temposCPU.sort = sort(temposCPU)
temposCPU.mean = mean(temposCPU.sort, trim=0.25)
temposCPU.amplitude = IQR(temposCPU.sort)
temposCPU.variancia = var(temposCPU.sort)
temposCPU.desvioPadrao = sd(temposCPU.sort)
CV <- function(x) { sd(x) / mean(x) }

```

## Exercícios – Análise Exploratória de Dados
- 1 [Jain, 12.10mod]Os tempos de CPU em milissegundos para 11 cargas de trabalho em um pro-cessador são os seguintes:
  - a) Que medida de tendência central deveria ser usada para representar esses tempos, e porquê? Obtenha o valor dessa medida para o conjunto.
    - R: Média Truncada (Aparada) em 25 % pois existem máximas e mínimas muito distantes. Valor: `r temposCPU.mean`
  - b) Determine  a  amplitude,  a  variância,  o  desvio  padrão  e  o  coeficiente  de  variação  para  oconjunto.
    - R: Amplitude `r temposCPU.amplitude`
    - Variância `r temposCPU.variancia`
    - Desvio Padrão `r temposCPU.desvioPadrao`
    - Coeficiencia de Variação `r CV(temposCPU.sort)`

```{r}
taxaExecucao = c(3, 5, 6, 12)
hmean <- function (x, na.rm=TRUE) { 1/mean(1/x, na.rm=na.rm) }
```
- 2 Um analista mensurou o desempenho de um sistema com 4 programas distintos e obteve a tabelaabaixo, que mostra a taxa de execução para cada programa. Qual a taxa média de execução (emMIPS) para esse sistema?
 - `r hmean(taxaExecucao)`

```{r}
tempoExecucao = read.table("texec.txt", header = TRUE)
tempoExecucao.max = max(tempoExecucao$te)
tempoExecucao.min = min(tempoExecucao$te)
moda <- function(x) {
  x <- na.omit(x)
  ux <- unique(x)
  tab <- tabulate(match(x, ux))
  ux[tab == max(tab)]
}

tempoExecucao.moda = moda(tempoExecucao$te)
tempoExecucao.mean = mean(tempoExecucao$te)
tempoExecucao.median = median(tempoExecucao$te)
tempoExecucao.cv = CV(tempoExecucao$te)

```
- 3 O arquivo de dadostexec.txtcontém medições de tempo de execução de umbenchmarkemum sistema. Determine
- (a)  o maior e o menor tempo de execução;
  - R: Maior: `r tempoExecucao.max`, Menor: `r tempoExecucao.min`
- (b)  o tempo de execução mais frequente;
  - R: Moda: `r tempoExecucao.moda`
- (c)  o tempo médio de execução;
  - R: Média: `r tempoExecucao.mean`
- (d)  a mediana do tempo de execução;
  - R: Mediana: `r tempoExecucao.median`
- (e)  o coeficiente de variação do tempo de execução.
  - R: CV `r tempoExecucao.cv`

- Qual medida de tendência central (média, mediana, moda) você usaria para resumir o tempo deexecução dobenchmark?
- R: a média

```{r}
clientSrv = read.table("rtt.txt", header = TRUE)
clientSrv.vazao = 1000 / (min(clientSrv$RTT) + 25)
clientSrv.hmean = hmean(clientSrv$RTT)
```
- 4 O arquivo de dadosrtt.txtcontém medições de RTT (round-trip time) entre um cliente e umservidor na Internet. Suponha que o processamento de uma requisição leva 25 ms no servidor, eque a resposta a uma requisição exige apenas um RTT.
- a) Qual a vazão máxima, em requisições por segundo, que o cliente pode esperar da aplicaçãoem questão?  Considere que o cliente envia uma requisição imediatamente após receber aresposta da requisição anterior.
- R: `r clientSrv.vazao` requisições por segundo
- b) Qual o tempo de resposta típico da aplicação para esse cliente?  Considere como típico ovalor que o cliente poderia esperar ao executar a aplicação.
- R: `r clientSrv.hmean`


```{r}
banda = read.table("lbanda.txt", header = TRUE)
banda.vazao.transmissao.max = max(banda$tx.kbps)
banda.vazao.recepcao.max = max(banda$rx.kbps)

banda.vazao.transmissao.tipic = moda(banda$tx.kbps)
banda.vazao.recepcao.tipic = moda(banda$rx.kbps)

banda.vazao.transmissao.dispersion.diff = diff(range(banda$tx.kbps))
banda.vazao.recepcao.dispersion.diff =  diff(range(banda$rx.kbps))

banda.vazao.dispersion.txt = 'Transmissão'
if (banda.vazao.transmissao.dispersion.diff > banda.vazao.recepcao.dispersion.diff) {
  banda.vazao.dispersion.txt = 'Recepção'
}

banda.taxa.transmissao.tipic = moda(banda$tx.pps)
banda.taxa.recepcao.tipic = moda(banda$rx.pps)

banda.vazao.transmissao.sum = sum(banda$tx.kbps)
banda.vazao.recepcao.sum = sum(banda$rx.kbps)

```
- 5.  O arquivo de dadoslbanda.txtcontém métricas de desempenho coletadas em um enlace derede, em intervalos de 2 s. As colunas do arquivo são as seguintes:
- (a)  Quais as vazões máximas de transmissão e recepção?
- R: Transmissão `r banda.vazao.transmissao.max`, Recepção `r banda.vazao.recepcao.max`
- (b)  Quais as vazões típicas de transmissão e recepção?
- R: Transmissão `r banda.vazao.transmissao.tipic` e Recepção `r banda.vazao.recepcao.tipic`
- (c)  Qual dos sentidos (transmissão ou recepção) possui vazão mais estável? (Use um índice dedispersão apropriado para embasar sua resposta.)
- R: Transmissão `r banda.vazao.transmissao.dispersion.diff` e Recepção `r banda.vazao.recepcao.dispersion.diff`, o mais estável é `r banda.vazao.dispersion.txt`
- (d)  Quais as taxas típicas de transmissão e recepção de pacotes?
- R: Transmissão `r banda.taxa.transmissao.tipic` e Recepção `r banda.taxa.recepcao.tipic`
- (e)  Estime o total de bytes transmitidos e recebidos durante a medição.
- R: Transmissão `r banda.vazao.transmissao.sum` e Recepção `r banda.vazao.recepcao.sum`

```{r}
sgdb = read.table("sgbd-tps.txt", header = TRUE)
sgdb.s1.sum = sum(sgdb$S1)
sgdb.s2.sum = sum(sgdb$S2)
sgdb.s3.sum = sum(sgdb$S3)

sgdb.sum.txt = "S1"
if (sgdb.s2.sum > sgdb.s1.sum) {
  sgdb.sum.txt = "S2"
}
if (sgdb.s3.sum > sgdb.s2.sum && sgdb.s3.sum > sgdb.s1.sum) {
  sgdb.sum.txt = "S3"
}


```

- 6.  Três aplicações cliente-servidor usando banco de dados foram submetidas ao mesmobenchmark,que foi executado 200 vezes com cada uma. O arquivo de dadossgbd-tps.txtcontém o númerode transações por segundo (tps) obtido em cada execução, para cada uma das aplicações (S1, S2,S3).  Supondo que obenchmarkseja representativo da carga de trabalho aplicada pelos usuáriosem ambiente de produção, determine qual das três aplicações apresenta o melhor desempenho.
- Melhor desempenho é `r sgdb.sum.txt`
  - S1 `r sgdb.s1.sum`
  - S2 `r sgdb.s2.sum`
  - S3 `r sgdb.s3.sum`


```{r}
globo = read.table("globo.txt", header = TRUE)
globo.tipic = moda(globo$X143)
globo.carga601.length = length(globo$X143[globo$X143 < 600])

globo.carga.percentage = (globo.carga601.length * 100) / length(globo$X143)
globo.carga.percentage
globo.carga.txt = "Não"
if (globo.carga.percentage > 98) {
  globo.carga.txt = "Sim"
}

```

- 7.  Um administrador de rede decidiu medir o tempo de carga da página principal de um portalInternet; os tempos observados, em milissegundos, estão no arquivo de dadosglobo.txt.
- (a)  Qual o tempo típico de carga da página?
- R: `r globo.tipic`
- (b)  Espera-se que o tempo de carga seja inferior a 600 ms em pelo menos 98% dos casos.  Issoestá ocorrendo na prática?
- R: `r globo.carga.txt`, a porcentagem de carga inferior a 600 ms é `r globo.carga.percentage`

```{r}
pacotes = read.table("pacotes.txt", header = TRUE)


#mean(pacotes$X125, trim=0.2)
```

- 8.  O arquivo de dados pacotes.txtcontém os tamanhos de 10.000 pacotes de dados observadosem uma rede. Descreva esse conjunto de dados utilizando medidas de tendência central.
- Média Aritimética: `r mean(pacotes$X125)`
- Média Truncada 20%: `r mean(pacotes$X125, 0.2)`
- Média Harmômica: `r hmean(pacotes$X125)`
- Observando os dados, e possivel visualizar uma grande dispersao dos dados, uma diferença bem grande entre os máximos e mínimos

```{r}
trbd = read.table("trbd.txt", header = TRUE)
trbd.tipic = moda(trbd$tr.bd)
trbd.dispersao = diff(range(trbd$tr.bd))
trbd.cv = CV(trbd$tr.bd)
trbd.min = min(trbd$tr.bd)
trbd.vazao = 1000 / trbd.min
```

- 9.  Um DBA mediu o tempo de resposta de um SGDB para 500 consultas.   O arquivo de dadostrbd.txtcontém os tempos de resposta observados, em milissegundos.
- (a)  Qual o tempo de resposta típico desse SGBD?
- R: `r trbd.tipic`
(b)  Descreva a variabilidade do tempo de resposta usando uma medida de dispersão apropri-ada.
- R: Possui uma dispersão max - min grande de: `r trbd.dispersao`, porém levando em consideração o coeficiente de variação que utiliza a média e o desvio padrão fica em torno de `r round(trbd.cv * 100, 2)`%
(c)  Qual a vazão máxima que pode ser esperada desse SGBD, supondo que as consultas sejamrepresentativas?
- R: `r trbd.vazao` consultas / segundo

```{r}
usuarios = read.table("usuarios.txt", header = TRUE)
usuarios.max = max(usuarios$users)
usuarios.max.which = which(usuarios$users == usuarios.max)
usuarios.max.line = usuarios[usuarios.max.which, ]

usuarios.min = min(usuarios$users)
usuarios.min.which = which(usuarios$users == usuarios.min)
usuarios.min.line = usuarios[usuarios.min.which, ]
#usuarios
usuarios.tipic = moda(usuarios$users)
usuarios.bigger260 = usuarios[usuarios$users > 260, ]
```

- 10.  Um servidor deintranetde uma empresa foi monitorado durante o horário de expediente, quevai das 8h às 18h. O arquivo de dadosusuarios.txtcontém o número de usuários que acessa-ram o servidor a cada minuto. Deseja-se saber as seguintes informações:
- (a)  Qual o número máximo e o número mínimo de usuários? Em que horários ocorreram essesacessos?
  - Máximo `r usuarios.max` na hora `r usuarios.max.line$horario`
  - Mínimo `r usuarios.min` na hora `r usuarios.min.line$horario`
- (b)  Qual o número típico de usuários que acessam o servidor?
  - R: `r usuarios.tipic`
- (c)  Uma  análise  de  desempenho  constatou  que  o  tempo  de  resposta  do  servidor  se  mostraaceitável para um máximo de 260 usuários por minuto.  Em que horários esse tempo deresposta ficou acima do considerado aceitável?
  - R: Nos horários: `r usuarios.bigger260$horario`



```{r}
consultas = read.table("aed-tcons.dat", header = TRUE)

#hist(consultas$S1)
plot(ecdf(consultas$S1), xlab="tempo (s)", panel.first=grid(),main="S1")
plot(ecdf(consultas$S2), xlab="tempo (s)", panel.first=grid(),main="S2")
plot(ecdf(consultas$S3), xlab="tempo (s)", panel.first=grid(),main="S3")
plot(ecdf(consultas$S4), xlab="tempo (s)", panel.first=grid(),main="S4")

consultas.s4.summary = summary(consultas$S4)
consultas.s4.quatile = quantile(consultas$S4, c(0.9, 0.95))

consultas.s1.sort = sort(consultas$S1)
consultas.s1.mediana = median(consultas.s1.sort)
consultas.s1.media = mean(consultas$S1)
consultas.s1.media.truncada = mean(consultas$S1, trim=0.05)

consultas.s3.sort = sort(consultas$S3)
consultas.s3.mediana = median(consultas.s3.sort)
consultas.s3.media = mean(consultas$S3)
consultas.s3.media.truncada = mean(consultas$S3, trim=0.05)

```

- 11.  O arquivo de dadosaed-tcons.datcontém medições do tempo de execução (em ms) de consul-tas em quatro SGBDs diferentes (S1, S2, S3 e S4).
- (a)  Determine o formato das distribuições dos tempos de consultas dos quatro SGBDs.
  - S1 assimétrico a esquerda
  - S2 bimodal simétrico
  - S3 unimodal simétrico
  - S4 assimétrico a direita

- (b)  Calcule o 3oquartil, o 90oe o 95opercentis dos tempos de consulta de S4.
  - 3 quartil `r consultas.s4.summary[5]`
  - 90 e 95 % dos tempos até `r consultas.s4.quatile`

- (c)  Calcule a mediana, a média aritmética e média truncada a 5% para os tempos de consultade S1 e S3. Como o truncamento afeta a média de cada sistema?
  - S1 Mediana `r consultas.s1.mediana`, Média Aritimética `r consultas.s1.media`, Média Truncada 5% `r consultas.s1.media.truncada`
  - S3 Mediana `r consultas.s3.mediana`, Média Aritimética `r consultas.s3.media`, Média Truncada 5% `r consultas.s3.media.truncada`
  - O truncamento afeta bastante apenas as consultas registradas no S1 então nesse caso, truncando apenas 5% dos maximos/minimos mudou muito o resultado na média, significando que possui muitos casos com tempos de execução com máximos bem altos ou/e mínimas bem baixas

- (d)  Os gráficos abaixo mostram a função de distribuição acumulada empírica dos tempos deconsulta dos quatro SGBDs. Associe cada gráfico ao sistema correspondente.
  - ECDF 1 -> S3
  - ECDF 2 -> S4
  - ECDF 3 -> S2
  - ECDF 4 -> S1

- 12.  O gráfico abaixo mostra oboxplotde um conjunto de dados. A partir do gráfico, estime os valores(mínimo,  máximo,  1oe  3oquartis,  mediana)  para  o  conjunto,  e  determine  o  formato  de  suadistribuição.
- R: Mínimo: 200, máximo: 400, outliers: 180 e 600, 1 quartil: 200, 3 quartil: 400, mediana: 300  formato: unimodal simétrica

## Including Plots

You can also embed plots, for example:

```{r pressure, echo=FALSE}
plot(pressure)
```

Note that the `echo = FALSE` parameter was added to the code chunk to prevent printing of the R code that generated the plot.