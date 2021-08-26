# Exercicios para aula 26/08

- 1. A sonda Phoenix foi enviada ao espaço pela agência espacial norte-americana em 4 de agosto de 2007 e, desde que pousou em Marte, no dia 25 de maio de 2008, envia fotos para a Terra. Uma foto transmitida tinha o tamanho de 8x10^6 bytes e, quando enviada, a distância entre os dois planetas era de 60 bilhões de metros (60x109m). Assumindo que o enlace de comunicação entre a sonda e a base da missão na Terra é de 128 kbps, que não há elementos intermediários, e que a velocidade de propagação do sinal é a velocidade da luz (3x108 m/s), quanto tempo, em segundos, se passou entre o início do envio da foto até ela ser recebida completamente na Terra?
  - tamanho da foto: 8x10^6 bytes
  - enlace: 128 kbps x10^3 = 128x10^3 bytes por segundo
  - Fórmula L/R = 500
    L - Tamanho do pacote 8x8x10^6 bytes
    R - largura de banda do enlace 128x10^3 bytes por segundo
    - Tempo de transmissão = 500 segundos
  - Fórmula d/s = 200
    - d comprimento do enlace físico 60x10^9 metros
    - s velocidade de propagação no meio (velocidade da luz) 3x10^8
    - Atraso de propagação = 200 segundos
  - Resposta D 700 segundos

- 2. R1. Qual e a diferenca entre um hospedeiro e um sistema final? Cite os tipos de sistemas finais. Um servidor Web e um sistema final?
R: Não há diferença entre hospedeiros e sistemas finais, na realidade, eles são dispositivos que estão nas “bordas” da internet. Atualmente existem vários tipos de sistemas finais, desde os tradicionais computadores até smartphones, televisores, entre outros dispositivos eletrônicos. Um servidor Web é um sistema final, porque ele está na borda e recebe/envia informação para outros sistemas conectados a internet.
- 3. Qual e a vantagem de uma rede de comutacao de circuitos em relacao a uma de comutacao de pacotes? Quais sao as vantagens da TDM sobre a FDM em uma rede de comutação de circuitos?
R:A comutação de circuitos é mais adequada a serviços de tempo real, é um circuito dedicado Numa rede de comutação de pacotes cada pacote tem que ser recebido integralmente por cada comutador antes de ser retransmitido, o que provoca atraso. No TDM a transmissão é digital (bits). Portanto pode haver correção de erros a cada estágio da transmissão (em cada comutador ou multiplexador TDM). Na TDM, como são alocados fatias de tempo para cada elemento que transmite, quando há silêncio em um determinado elemento, a fatia de tempo alocada a ele pode ser usada por outro, o que não é o caso no FDM onde a freqüência está alocada todo o tempo a cada elemento.
