# Resumo

- [Artigo](https://www.en-trust.at/papers/Brunner20a.pdf) 

- Revocation List: Como seria atualizado a lista de keys revogadas em uma blockchain ? 
  - Seria apenas necessário incluir uma key em uma nova transação?
  - Caso insira uma nova transação, como seria facilmente recuperado depois ? sem ter de percorrer todos os blocos ja criados

Distributed Hash Table (DHT)

- OnChain: Salvar todo o conteudo dentro da blockchain = ruim
- OffChain: Salvar apenas a hash na blockchain e os dados do certificado em uma DHT = bacana, should be public

Web of Trust (WoT) > hierarchically

(Al-Bassam, 2017) Premissioned Revocation Ethereum Custom WoT only Hash Public DHT NotUpdatable NotPrivacy NoIncentive Complexity NoCost

Revocation: Support for revoking certificates in a
PKI is a must. Revocation is a key feature of PKIs as
this is the only way to ensure the long-term validity
of a certain certificate.

[Al-Bassam](https://dl.acm.org/action/showFmPdf?doi=10.1145%2F3055518)
[Al-Bassam-open](https://sci-hub.tw/10.1145/3055518.3055530)

[SCPKI](http://www0.cs.ucl.ac.uk/staff/m.albassam/publications/scpki-bcc17.pdf): A Smart Contract-based PKI

Pretty Good Privacy (PGP) is a data encryption and decryption standard that does not use CAs to verify the authenticity of public keys. Instead, it offers a feature that
allows individuals to sign other individuals’ public keys to
certify their authenticity. This creates a web-of-trust model
that can navigated to determine the authenticity of public
keys belonging to individuals that have no pre-shared secret
with each other.

One such blockchain-based system for smart contracts is
Ethereum [10]. The Ethereum white paper describes smart
contracts as “complex applications involving having digital
assets being directly controlled by a piece of code implementing arbitrary rules

The primary proposition of SCPKI is to write such a smart
contract with functionality for the operation of a public key
infrastructure and identity management system, where public keys and identity attributes are stored on the blockchain
and can be managed by the smart contract.

Due to the expensive gas costs associated with Ethereum
storage, SCPKI is designed to allow users to store large attribute 
data (such as PGP keys) off the blockchain (such as
on IPFS) to save costs, while maintaining the authenticity
of the data by providing a cryptographic hash of the data
with the attribute on the blokchain.

The RevokeSignature function only revokes signatures if
the Ethereum address calling it is also the signer of the signature to be revoked; the signature is checked to ensure that
the signer matches the sender of the transaction calling the
function. The procedure for this is shown in Algorithm 1.

Future Works

Privacy. The system is only suitable for the publishing of attributes that the user wishes to make public (such
as degree awards). It is not suitable for the publishing of
more private identity attributes such as personal address
as all attributes can be viewed by anyone in the system
and there is no access control. Future iterations of the
system may address this by adding functionality to publish “zero-knowledge” attributes for verification of privately
shared data that the user distributes and has control over.