## Securing Microservices and Microservice Architectures: A Systematic Mapping Study

- Zero trust models

- Access Control - Authorization:techniques used to check users’ permissions for accessing specific MSAresources or data.

- Auditing:techniques applied at runtime for discovering security gaps and may:  (1) subsequently initiateappropriate measures or (2) simply report security breaches to relevant supervisory authority.

- Mitigation:techniques that limit the damage of attacks when they appear.  Mitigation techniques can beintegrated into existing mircoservice-based systems.

- Prevention:techniques that try to stop attacks from happening in the first place. Prevention techniques need tobe considered when developing new mircoservice-based systems.

- authentication and authorization techniques is defensible: Bad solution

#### MTD

Most, if not all, proposals for mitigation are Moving Target Defense-based solutions (MTD) [68]. The idea behindMTD is to continuously perform transformation of system components and configurations preventing attackers fromacquiring knowledge about target systems to be used to initiate harmful attacks.  This includes, periodically updateor restart microservices, IP shuffling, and live migration of microservices. Specifically, the authors of P21 proposeddeception through live cloning and sandboxing of suspicious containers respecting the same network overloading andperformance to deceive attackers.

Prevention proposals are the most diverse techniques.  They varies from using physical computing devices such asHardware Security Module (HMS), powerful techniques and technologies such as encryption and Blockchain intoadopting software design decisions such as using secure programming languages and smart contracts.

Nice - Due to the lower rate of mitigation techniques and their applicability to existing microservice-based systems, weadvocate more research studies on mitigation techniques


Use of HSML: BAD