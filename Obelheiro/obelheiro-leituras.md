# Obelheiro Leituras - material-rafael - DDoS

###### tags: `master`

- Ip fixo: 45.182.161.96

### Ideias

- Colocar IP fixo para testar de fato e conseguir colocar a mão na massa

- Alguns servidores são utilizados como amplificadores em ataques por terem algumas de suas "barreiras" de segurança desabilitadas ou mal configuradas
- Talvez a gnt possa ter algo do tipo em que testamos quais as formas de barreiras de segurança são as mais eficazes para evitar que o seu servidor vire um amplificador

- Automatizar a parte de deploy x recolher dados e logs talvez utilizando kubernets Daemon em que cada node poderia ser uma das máquinas
- Dashboard para visualizar dados com Grafana & Prometheus

##### Root path
### [2017 - 1000 days of UDP amplification DDoS attacks](https://drive.google.com/file/d/1puJAP5_wRV580Tn26qqrxhU8GQN42pLW/view?usp=sharing)
- O estudo apresenta um honeypot chamado de Hopscotch, utilizado para determinar ataques de amplificação com UDP e caracterizar esses ataques, juntamente com as vítimas. Hopscotch é um pequeno programa em C (3000 linhas), desenvolvido para suportar protocolos UDP, após períodos de testes, foi adicionado demais funcionalidades para auxiliar como SQLMon, SSDP e MS SQL. Os dados do estudo foram recolhidos com a análise de 65 refletores de tamanho médio do período de julho de 2014 a Janeiro de 2017, os pesquisadores recolheram dados com uma média de 1450 scanners maliciosos por dia, através de todos os protocolos UDP. Nesse período também, foi observado 5.18 milhões de ataques envolvendo mais de 3 trilhões de pacotes. Algumas análises interessantes foi que a proporção de ataques utilizando DNS diminuiu com o tempo, já a proporção de ataques utilizando NTP aumentou.
<!--
- If  the  IP  addresses  used  for  malicious  scanning  changeregularly then it is harder to block or investigate them

- Over  the  observation  period 3 years,  the  median  estimatedtotal  number  of  attacks  per  day  for  CHARGEN  is  488,  forDNS is 1 930, for NTP is 4 130, and for SSDP is 972
 -->
### [2014 - Amplification Hell: Revisiting Network Protocols for DDoS Abuse](https://drive.google.com/file/d/1cYuonglli0Mc3jOmip_ap0oxpQaxofN9/view?usp=sharing)

- Inicia explicando ataques DRDoS e um pouco dos protocolos UDP e como eram especificados os modelos de ataques observados: em que o atacante envia requisições  para amplificadores com o IP da vítima como a origem do pacote, assim os amplificadores enviam potencialmente multiplos do tamanho original da resposta para a vítima. Algumas peculiaridades foram descritas no documento, como que leva em média 100000 BitTorrent amplificadores, e scaneando para os demias protocolos como SNMP, NTP ou DNS leva menos de 4 minutos. Além disso, eles obteram dados que apontam que cerca de 1 milhão de usuários finais foram comprometidos. A conclusão aponta que pelo menos 14 UDP based protocols ou implementação de serviços sucetiveis a serem comprometidos como amplificadores podem ser fácilmente quebrados, a não ser que admnistradores de rede estejam preparados para ataques DRDoS.
-
<!-- - For network services, we assume that they run on their standardized UDP port and we scan the Internet for available amplifiers. To save resources and limit the noise of our scans, we scan a random sample of 106 IP addresses out of the set of all 2.6 billion advertized IPv4 addresses. We then extrapolate the total number of amplifiers from the number of amplifiers found during our partial scan. In our experiments, we used a synchronous 150 MBit/s Internet connection for scanning, and each scan finished in 65 seconds on average. Implementing an optimized and complete scanner is out of the scope of this work. As shown by Durumeric et al. [8], an attacker could speed up this process and complete a /0 IPv4 scan for one protocol in less than two hours by using a 1 Gb/s uplink and a more efficient scanner implementation. Similarly, Dainotti et al. observed botnets that complete /0 stealthy scans [6]. This shows that, in practice, scanning is no burden to attackers.
    - PERGUNTA: como funciona um scanner para os atacantes?
        - through peer list exchanges, a process often referred to as crawling.
        - studied the source code of popular protocol implementations (BitTorrent)
        - obtained an initial set of ten bootstrapping peers from public resources or the protocol implementations.
        - then iteratively query the set of known peers to retrieve their neighboring peers
        - include only peers that at least once responded to our requests
- In the specific case of DNS, we measure two types ofamplifiers. First, we scan the Internet for open resolvers, i.e.,public resolvers that serve recursive name lookups to anyclient. Second, we give lower bounds for the numberof authoritative name servers. We cannot scan for authoritativename servers, though, as we do not know the zones a server isauthoritative for.
    - PERGUNTA: Qual a forma utilizada para atacantes para selecionar seus amplificadores
- DNS:Name lookups (e.g.,AorMXrecords), the tra-ditional use-case of DNS, have low amplification rates. Tra-ditionally, the size of UDP replies was limited to 512 bytesand DNS switched to TCP communication for larger replies.However, many DNS servers adopted the DNS extensions(EDNS0) that allow for UDP responses of up to 4096 bytes.Attackers may abuseANYrequest with EDNS0, for which aserver returns all known DNS record types for a given domain.
- Darknet traffic
- 14 UDP-based network protocols or service implemen-tations are susceptible to amplification abuse

 -->

### [2016 - Who Gets the Boot? Analyzing Victimization by DDoS-as-a-Service](https://drive.google.com/file/d/1aJDS9x0WcUShtl-LSzsfDEqJ3gMBUt_d/view?usp=sharing)


Nessa pesquisa, foram utilizados cerca de 115795 IP vitimas capturadas em um período de 2 anos (2014-2015), através de vários honeypots. Uma das partes mais impactantes apresentadas na pesquisa é que existem muitas vítimas de webhosting, em que o maior número de vítimas de ataques DDOs são sites relacionados a jogos como principalmente Minecraft, apresentando que é tecnicamente fácil contratar ataques DDOs, nem sempre com foco em lucros ou algo assim, mas sim por quase rixa de jogos, viganças, etc... Outro valor absurdo apresentado pelo trabalho é que o preço para comprar um ataque DDoS amplificado pode ser tão baixo quanto um dolar, sendo realizado através de serviços como booter em que normalmente implica em acesso ao serviço por um período de tempo, atrelado a vários níveis de preço, mas por exemplo um ataque de apenas 10 minutos ja é o suficiente para tirar um jogador de uma partida. Por fim, os dados apresentados explicam que a comodidade de realizar ataques DDoS leva os atacantes a focar os ataques a usuários finais, ou seja, provavelmente pessoas que o atacante "conheça", seja apenas virtualmente, mas que ja tenha tido um contato e deseja atrapalhar o jogo que aquele usuário esteja utilizando no momento, o que muitas vezes pode parecer inofensivo, mas em todos os locais é considerado como crime.

<!-- - also uses AmpPot
- comprar um ataque custa $1!!
- The most frequently attacked UDP ports by a large margin include ports 80 and 8080,that are more likely to be open and accessible through firewalls

- We  have  now  established  that  the  majority  of  victims  reside  in  broadband providers and that the majority of these victims are access nodes. In other words,home routers are typically the most affected devices. It suggests that the actualtarget is a regular home user behind that router
    - Pergunta: Isso acontece mesmo quando voce tem IP dinamico. Imagino que sim, porém não tão frequente
    - O provedor não teria como realizar algum filtro referente a isso, pois todo o trafego passa por ele, correto ?


- Incrivel a quantidade de ataques que acontecem contra servidores de jogos ex.: Minecraft, que imagino que na maioria das vezes por vingança

- Ataques ciberneticos são considerados crimes, se um honeypot chegar a auxiliar em um ataque, pode ser classificado como crime ? claro que para fins acadêmicos e de pesquisa, deve ser justificavel

> ISP Internet Service Provider -->

### [2015 - AmpPot: Monitoring and Defending Against Amplification DDoS Attacks](https://drive.google.com/file/d/1btrLs2arem5QWrTanyEX4EkfU047Q4cF/view?usp=sharing)

A pesquisa apresenta um honeypot chamado de AmpPot que rastreia ataques de amplificação. Esse honeypot AmpPot imita serviços de ataque de amplificação ouvindo portas UDP que provavelmente serão atacadas, mais especificiamente os seguintes protocolos: QOTD (17), CharGen (19), DNS (53), NTP (123), NetBIOS (137), SNMP (161) and SSDP (1900), plus MSSQL (1434) and SIP (5060/5061), além disso o honeypot tem 3 modos de uso que devem ser informados ao iniciar, **Emulated** que responde uma resposta pré-gerada de acordo com o protocolo utilizado, **Proxied** que torna o honeypot em um proxy que encaminha as requisições que de fato implementao o protocolo vulnerável e **Agnostic** que responde bytes aleatórios 100x o tamanho da requisição, que esse ultimo modo assume que os atacantes não se importam com respostas válidas, somente no tamanho dos pacotes. Uma das descobertas interessantes apresentadas no estudo é que 96% dos ataques capturados, analisados e identificados provém de uma única fonte, além de que 79% das vítimas foram atacadas somente uma vez! 11% duas e somente 0.81% das vítimas foram atacadas repetidamente (mais de 10x)

<!--
- find that the vast majority of attacks areshort-lived and most victims are attacked only once
    - Pergunta: se a maioria dos ataques acontecem apenas uma vez, quanto tempo a vítima fica em média indisponível
        - tirando os ataques menores de 300seg, 90%  of  the  attacks  last  at  most  onehour. Only 1.4% of the attacks last longer than 4 hours. This shows thatattackers quickly move on to attack other victims.
        - 79% of  the  victims were attacked  onlyonce; a further 11% were attacked twice. 0.81% of the victims were at-tacked more than 10 times

- AmpPot -> nome do honeypot, a novel honeypot to capture amplification DDoSattacks
- In ourlater  deployment  of  the  honeypots,  we  received  only  four  emails  fromattack victims, which we responsibly answered. After our clarification,none of the victims claimed that we caused damage
    - Pergunta: Já enviaram algum email para vocês ?
- Dentro de 24 horas os honeypots começam a ser atacados, porem tiveram uma grande escala após 5 dias
    - Foi observado o mesmo padrão ?


-  Real-time Attack Monitoring, que é um broker ou dashboard para visualizar em tempo real os ataques que estão sendo realizados ja pensaram em fazer algo assim ?

- Honeypot Detection: We leave it open to future work to explore how we couldincrease the stealthiness ofAmpPot
 -->

### [2017 - Millions of Targets Under Attack: a MacroscopicCharacterization of the DoS Ecosystem](https://drive.google.com/file/d/1rF0xH8ammiAKkMF91ApPBOvXpBNM8Wsr/view?usp=sharing)

Esse documento analisou e correlacionou quatro conjunto de dados, dois contendo eventos com diferentes características, um deles contém ataques e o outro contém logs de eventos em que foram utilizados amplificadores honeypots. O terceiro conjunto de dados foi derivado the DNS ativos e entre outras informações mapeadas de IPs e domínios, o quarto conjunto de dados rastreia quais sites terceirizam a proteção de DDoS Protection Services. Além disso exemplificam metodologias e extensões para DPS (DDoS Protection Services). Algumas descobertas bacanas apresentadas pela pesquisa é que 64% dos sites foram hospedados em endereços IP que são alvos de ataques, com uma média de que 3% deles eram alvos diariamente. Por fim, basicamente um framework foi apresentado para auxiliar na caracterização de ataques DoS de um modo sistemático em que foi possível correlacionar dados de diferentes fontes de dados e com uma diversidade de informações para serem conectadas e atreladas entre si. O framework não trabalha com dados em realtime, mas isso pode ser aboradado como um trabalho futuro.

<!--
- Diversion is usually implementedthrough the DNS or through the Border Gateway Protocol (BGP)

- Randomly spoofed attacks against Web ports are more in-tense.Given the prominent presence of Web ports (i.e., 80 & 443)

- The first-most and second-most countries to which joint targetsgeolocate are the US and China, with24.4%and20.4%, respectively.France comes third (9.5%) and Germany fourth (6.5%)
    - Isso seria devido ao fato de que la possui uma maior concentração de hosts a serem atacados ?

- Overall, the three most frequently attacked larger parties thatwe identify over the two-year period are, in order,GoDaddy,GoogleCloud, andWix.

- O que um DPS faz de fato para proteger, bloqueia certas portas, configura melhor o server ?

> BGP Border Gateway Protocol is a standardized exterior gateway protocol designed to exchange routing and reachability information among autonomous systems on the Internet.

> darknet -> A dark net or darknet is an overlay network within the Internet that can only be accessed with specific software, configurations, or authorization, and often uses a unique customized communication protocol.

> DDoS Protection Services (DPS) -->

## DDoS

### [2014 - Taming the 800 Pound Gorilla: The Rise and Decline of NTP DDoS Attacks](https://drive.google.com/file/d/14_bQFD0B4w_P8GbMonVGni_R0zV_b1rD/view?usp=sharing)

A pesquisa de 2014 apresenta que ataques DDoS utilizando o protocolo NTP(Network Time Protocolo) cresceram drasticamente e existem alguns servidores chamados de "mega amplifiers" que chegam a responder GB de dados em um único pacote, o protocolo NTP é utilizado para sincronização de relógios entre sistemas de computadores. Em mais um artigo, se apresenta que entre os milhares de IP's vítimas de ataques DDoS, a maioria aparenta ser de jogadores de jogos online.
Para verificar servidores NTP uma organização chamada de OpenNTPProject passa um scan em todos os endereços de IPv4 mandando um pacote NTP que requisita um servidor alvo, retornando assim uma lista de resultados e capturando todos as respostas dos pacotes.
As 3 portas mais utilizadas para atacar as vítimas são 80(TCP. HTTP), 123(NTP) e 3074(XBox Live), entre as 20 portas mais utilizadas, 10 são relacionadas a jogos como Steam Half-life (27015), Minecraft (25565) entre outras...
Após inúmeros servidores NTP vulneráveis serem notificados para verificarem o porque de seus pacotes de respostas chegarem a GB além de atualizarem softwares e barreiras de segurança, foi possível observar uma melhora na redução de tráfegos de ataque, o que mostra que em algumas networks é possível melhorar rapidamente e diminuir a quantidade de ataques digamos mais "fracos".


### [2016 - Identifying the Scan and Attack Infrastructures Behind Amplification DDoS Attacks](https://drive.google.com/file/d/1fOO0mhgPa_jXf8MOC9H5RjnEzmTE5DO4/view?usp=sharing)
A pesquisa apresenta tecnicas para descobrir as estruturas utilizadas por tras dos ataques DDoS. Uma das tecnicas consistem em que foi desenvolvido nos scanners uma metoodologia que consegue identificar ataques de amplificação, permitindo assim linkar ataques subsequentes ao de volta ao atacante. Para utilizar essa técnica de reconhecer o atacante, é possível fazer isso pois ao realizar o scan da rede em busca de amplificadores, os scanners não conseguem mascarar seus endereços IP, e em um segundo passo é testado se a infraestrutura disposta pelos honeypots e o scan é também utilizada para iniciar ataques. Assim, é possível observar que origens possuem distâncias similares (em termos de pulos de rede) entre sensores distribuídos globalmente, usando triangulação, então foi possível linkar scanners para os ataques de origens baseado em contagem de pulos de rede com cerca de 98%+ de confiança. Durante o período de 23 semanas entre 2015 ~ 2016 foram realizados 1.351.852 ataques de amplificação, e o framework utilizado para coletar os dados e honeypots foi do AmpPot. Para fazer a triangulação eles utilizaram a contagem o TTL time-to-live dos pacotes enviados aos honeypots.

### [2021 - BGPeek-a-Boo: Active BGP-based Traceback for Amplification DDoS Attacks](https://drive.google.com/file/d/1lVZnQULfhGVB4UObRIESQGvodbModIAz/view?usp=sharing)

Essa pesquisa, apresenta uma outra forma de rastrear o IP de origem do atacante, mas diferente das anteriores que necessitavam de um conhecimento previo dos atacantes marcando os scanners e se utilizando o TTL, dessa vez foi proposto o BGPEEK-A-BOO que não requere cooperação de routeadores ou conhecimento prévio da origem de potencialmente atacantes, em que basicamente eles se utilizam da rede de provedor do atacante, utilizando o protocolo Border Gateway Protocol (BGP) para identificar o Autonomous System (AS) que emite trafego de ataque modificado. Algo extramamente bacana, é que utilizando de BGP Poisoning para ataques DDoS e o seu algoritmo de "549" steps, com menos de 4 dias é possível encontrar a localização do IP do atacante, já em uma segunda abordagem utilizando grafos e apenas "98.5" steps algo que significa um pouco mais de um dia.

<!-- "Steps" de algoritmos -->

## Honeypots


### Honeypots for Distributed Denial of Service Attacks

- [Link](https://drive.google.com/file/d/1zlswx5qx0rr2l7LKxCbZZDZPsORzYXOu/view?usp=sharing)
##### Resenha
A obra inicia com uma introdução a DoS e DDoS e uma abertura a DRDoS mas lá ainda não é chamado assim, apenas apresenta refletores em sua estrutura. Após dar exemplos de caracteriticas e definições de ataques de negação de serviço, explora as defesas a esses ataques, categorizando em 2 tipos de abordagens: Mitigação e identificação da fonte do ataque, contudo apresenta que nenhuma das abordagens até o momento é aplicavel no mundo real... Então a ideia do documento é apresentar uma outra abordagem cacpaz de defender a rede operacional com uma grande probabilidade à ataques ja conhecidos além de novas possíveis variantes, além de gravar informações do atacante para uma possível ação legal contra ele. O termo Honeynet é introduzido como uma melhoria ao tradicional honeypots utilizados para detecção de ataques, as principais diferenças estão em que a honeynet não é um único sistema, mas uma rede de multiplos sistemas, outra diferença é que todos os sistemas com honeynet são de fato sistemas em ambiente de produção. Após isso, a abordagem conta que a honeynet irá enganar o atacante, retornando o que ele espera que aconteceria com a sua requisição, além de que após reconhecer um ataque iria avisar a rede e se adaptar a ele. Por fim, o maior problema que eu vejo é o passo utilizado para identificar um ataque utilizando assinaturas the ataques DDoS, pois além do método ter pontos ruins, como mudar a assinatura para um atacante pode representar um byte de diferença em sua requisição, e essa abordagem cai em um problema em que até a detecção do ataque, ele de fato segue com o ataques DDoS em um ambiente de produção...

### Honeypots: Catching the Insider Threat

- [Link](https://drive.google.com/file/d/1aLidlwtDGy4l-EoLoXvRhOYBSOKAWq5X/view?usp=sharing)
##### Resenha

Esse documento teve um foco diferente das demais para o uso de honeypots, ao inves de buscar servir de isca para ataques DoS, tem um foco em ataques que buscam recolher dados, o que em alguns casos pode ser ainda pior. Além de explicar o que são honeypots, da uma nova noção em que honeypots não necessáriamente precisam ser programas ou softwares, mas podem ser um arquivo, login e senha (naturalmente chamado de honeytokens), alguns conceitos importantes sobre honeypots foram apresentados, que geram pequenas amostras de dados, comparado aos milhares de logs gerados por outras aplicações, a redução de falso positivos, pois qualquer comunicação com um honeypot é por definição não autorizado, encontra falso negativos, pois como qualquer comunicação com o honeypot é suspeita, qualquer atividade como o honeypot é uma anomalia e por fim que honeypots utilizam poucos recursos. Um ótimo exemplo para honeytokens é apresentado: Em um hospital, estão tentando verificar o funcionário que está acessando dados privados dos pacientes, então um registro com nome de celebridade ficticio é adicionado ao banco de dados, se alguém acessar esse registro e entrar para verificar seus dados, é possível determinar que o funcionário que fez isso não estava apenas olhando os dados para tirar conclusões medicas, pois o paciente ali nunca foi registrado, esse é um exemplo de detecção de um intruso ou agente infiltrado na organização.

- "A honeypot is an information system resource whose value lies in unauthorized or illicit use of that resource. "


### A Hybrid Honeypot Architecture for Scalable Network Monitoring

- [Link](https://drive.google.com/file/d/11Ep8kntk0QNh6xe0_yU6IOsEmunwJ_or/view?usp=sharing)
##### Resenha

A pesquisa apresenta um modo hibrido entre honeypots de baixa e alta interatividade para monitorar os ataques, basicamente a ideia apresentada é que é inviavel ter apenas honeypots de alta interatividade, pois o custo pode ser elevado, então um modo hibrido em que as requisições são recebidas por honeypots de baixa interatividade e caso a assinatura recebida ou de alguma forma é identificado que é necessário alta interatividade como TCP o proxy redireciona a requisição para esse honeypot de alta interatividade (que é uma VM). Contudo, alguns problemas com essa abordagem podem acontecer, como os sensores podem não reconhecer que o ataque necessita de alta interatividade para recolher demais informações, e assim podendo perder informações, claro que o lado positivo é uma possível economia nos gastos de servidores/armazenamento. Por fim, a pesquisa mostrou uma abordagem diferente das demais para caracterização e mitigação de ataques worm ou ataques automatizados utilizando uma solução hibrida de honeypots de alta e baixa interatividade.

- Ataques worms são tipos de malware ou softwares maliciosos que podem se espalhar rapidamente pela internet.


### Detecting Targeted Attacks Using Shadow Honeypots

- [Link](https://drive.google.com/file/d/1nVsqaPOhNxUf_4zwxeHEaESubtdwoHL9/view?usp=sharing)
##### Resenha
Esse documento, apresenta uma arquitetura chamada de shadow honeypot combinando sistemas de detecção de anomalia e honeypots, a arquitetura é a seguinte: são 3 principais componentes, um mecanismo de filtro, um array de sensores de detecção de anomalias e o shadow honeypot, que valida as predições de detecção de anomalias. O framework apresentado como shadow honeypot tem uma implementação bem específica em modificar códigos em C para utilizarem certo mecanismo de funções de alocação de memória como "pmalloc()". Por fim, o documento apresentou uma abordagem para lidar com ataques combinando honeypots com sistemas de detecção de anomalias, a principal vantagem dessa arquitetura é o filtro de falso positivos que ocorrem no seguinte processo: O array de sistemas de detecção de anomalias que monitora e classifica todo o trafego para uma rede protegida, o tráfego que apresenta anomalia é então processado pelo shadow honeypot um programa instrumentado de modo protegido com 'pmalloc', ataques auferidos ao shadow honeypot são detectados e pegos antes de infectarem o estado da aplicação, isso permite o sistema a implementar regras que façam valer a pena a troca de desempenho ao risco de ameças.


### Leurre.com: on the Advantages of Deploying a Large Scale Distributed Honeypot Platform

- [Link](https://drive.google.com/file/d/1lJu4xO97K3w-Df1_GbxE3rEAOqheJOIC/view?usp=sharing)
##### Resenha
Esse projeto Leurre.com tem o foco de identificar padrões em locais específicos para pegar informações mais precisas, com o desenvolvimento e implantação de honeypots distribuídos por 6 meses, e encontraram fatos peculiares, que em um mesmo pais como a França, 3 plataformas foram implementadas, 2 em ambientes de redes acadêmicas de universidades distintas tem ataques e até portas muito similares, já a terceira plataforma implantada em uma rede da industria possui padrão de ataques bem diferente das demais. Além disso, como a implantação dos honeypots distribuídos foi global, foi possível observar que USA e China estão no top 3 países atacantes, possívelmente pela alta quantidade de poder computacional nesses países. Por fim, muitos pontos apresentados no documento apontavam que era um estudo mais aprofundado para obter melhores entendimentos e informações sobre os ataques, então talvez em um proximo estudo, mas fui acessar leurre.com e não obtive sucesso para encontrar maiores informações.

- A maioria dos ataques são originados de SO windows 80 ~ 95 %

### The Honeynet Project: Data Collection Tools, Infrastructure, Archives and Analysis

- [Link](https://drive.google.com/file/d/10gfNW3Z5VSxKmNyxrrUc937FxUaes2u6/view?usp=sharing)
##### Resenha

O projeto honeynet, incia apresentando as ferramentas para coleta de dados de ataques, ferramentas para análise, incrivelmente inúmeras ferramentas são feitas ou possuem suporte para o Windows. Além de também apresentar formas de infraestrutura para a coleta de dados, que podem ser de vários tipos: Ad hoc, coordenado centralmente; honeypots de baixa ou alta interatividade, honeypots virtuais ou fisicos; honeypots desligam totalmente uma vez que comprometidos, ou continuam suas operações; Geograficamente locais or remotamente implantado; Coleta de dados para uso privado, limitado ou disponível para auxiliar em conjunto de dados compartilhados. Basicamente, o documento apresenta as tecnologias e infraestruturas utilizadas no projeto honeypot e apresenta sua busca por mais organizações e adoção da industria para auxiliar no processo de captação de dados de ataques.

### The Leurre.com Project: Collecting Internet Threats Information using a Worldwide Distributed Honeynet

- [Link](https://drive.google.com/file/d/12mUxVdXWNLqa0AtYjQD29Dw4fpMmTP0-/view?usp=sharing)
##### Resenha
Dessa vez o projeto leurre.com apresentou um documento em que contem vários honeypots distribuídos e que armazenam seus dados em um banco de dados centralizado, além de os honeypots serem distribuídos em vários locais geograficamente, também conseguiram abrangir diferentes ambientes como educacional, governamental e de setores privados. Outra parte interessante abordada, é como é o formado do banco de dados, os modelos de entidade e relacionamento, e como esses dados foram gerados pelos honeypots, utilizando diferentes ferramentas como para buscar o sistema operacional do atacante utilizando Disco e p0f para buscar se o ataque veio de uma maquina windows, linux... e também que algumas vezes, faz sentido ter os dados duplicados em tabelas diferentes para facilitar queries, utilizando bancos não relacionais isso acontece bastante. Foi apresentado com bastante detalhes as ferramentas e implementações utilizadas para reconhecer informações como qual malware foi utilizado, além é claro do ScriptGen apresentado que permitiu uma maior interação do honeypot com o atacante para abstrair maiores informações sobre o mesmo. Enfim, o projeto leurre v2 foi projetado para coletar dados estruturados de ataques que acontecem de modo comun na internet, com o principal objetivo de descobrir e caracaterizar novas tendências e tecnicas de ataques, e a melhor parte é dar a possibilidade de outros pesquisadores consultarem os seus dados recolhidos para fazerem pesquisas e tirarem demais conclusões.

### Comparison of Empirical Data from Two Honeynets and a Distributed Honeypot Network

- [Link](https://drive.google.com/file/d/15Yeh0XaEQVhaW-1bf296DSzUhZEYqV39/view?usp=sharing)
##### Resenha
A pesquisa apresenta a comparação de 2 honeynets e 1 rede de honeypots distribuidos, as 2 honeynets são de alta interação, um em um ambiente de pequena e média empresa e outro de um ambiente comparativo, e o outro uma rede de honeypots distribuídos com cerca de 150 honeypots conhecidos como Leurre, no período de Maio/2007 até Julho/2007. O documento apresenta algumas peculiaridades de diferenças e similiaridades entre eles. Como que o número total de atacantes era diferente nas três honeynets. Leurre com os honeypots distribuídos teve o maior média do número de pacotes por honeypot e também a maior quantidade de IPs distintos atacados. Além disso, novamente foi apontado que a maioria dos ataques teve a origem a partir dos Estados Unidos, e os Chineses também estão próximos ficando no top 5, algumas diferenças também acontecem no realizar dos ataques, em que a maioria dos atacantes parecem buscar a maior quantidade de hosts na rede distribuída da Leurre, e nos outros 2 honeynets acontece que eles descobrem poucos hosts inicialmente e após um período a quantidade de hosts iniciais aumenta. Algo muito interessante observado, foi que a persistencia dos atacantes (o número de apcotes que um atacante lança contra a rede) parece seguir a lei do poder: um pequeno montante de atacantes para cada rede é responsável pela maioria do trafego malicioso nas redes. Algo possível de comparar também devido ao utilizar de ambientes distintos foi que inicialmente parte dos dados para a rede corporativa e a rede de pequenas e médias empresas (honeynets) sugere que para um atacante que ataca uma dessas honeynets tem chance (raiz  quadrada de 10) de atacar a outra honeynet.



### A Survey: Recent Advances and Future Trends in Honeypot Research
- [Survey](https://drive.google.com/file/d/1y7MB93shzs2kfjnrTmYsduWua4HYvmyF/view?usp=sharing)


### Distributed Honeynet System Using Gen III Virtual Honeynet
- [Link](https://drive.google.com/file/d/1b_Synq4ooRzZsYlAsO_iRQObpdC4_OfI/view?usp=sharing)
O documento apresenta a utilizacao de uma unica máquina para virtualizar vários hosts e definir ip específicos ou faixas de ip para cada um dos hosts virtualizados (utilizando até diferentes sistemas operacionais), eles coletaram um grande número de dados, e essa forma sem duvida facilita a coleta de dados, pois estao tecnicamente todos utilizando o mesmo storage, isso fica inviavel a partir da utilizacao de honeypots distribuidos em inumeras máquinas. O documento não possui foco em ataques de negação de serviço, mas sim em qualquer tipo de ataque, com vários dados que foram apresentados, o foco maior foi em binários maliciosos inseridos ao host, e poucos detalhes de ataques DDoS foram demonstrados. Por fim, parece uma abordagem inteligente, caso esteja sendo buscado heterogenidade nos hosts contando sistemas operacionais distribuições, etc e tenha poucos recursos para instanciar uma máquina de cada tipo desejado.



### Honeyeye: A Network Traffic Collection Framework for Distributed ICS Honeynets

- [Link](https://drive.google.com/file/d/1q7fPLtMnmJi4JWQchwuUxHtRxoFWIIpT/view?usp=sharing)
##### Resenha

O documento, apresenta um framework implementado em java para coletar dados de trafego da rede e armazenar esses dados localmente ou enviar para um banco de dados remoto. Além de capturar dados da rede o framework "honeyeye" também computa e processa os dados binários recolhidos da rede para dados mais legíveis e converter para formatos desejados como json. O objetivo do framework é prover dados legíveis para administradores de rede e para mecanismos de segurança da rede decidirem como reagir aos trafegos registrados que podem ser ataques e afins.


#### Observations

- Definition of attacks
"We use the same conservative definition of an attack as Kramer et al., who define an attack as a stream of at least 100 consecutive packets from the same source to the same port without gaps longer than one hour."

- Traceback is complex