# Basics

## TLS

Transport Security Layer provide end-to-end security in the internet.

TLS was originally designed to work on top of a TCP/IP
stack and thus, it is connection oriented.


public key infrastructure (PKI)
Certificate Revocation Lists (CRLs)

TLS handshake with Elliptic Curve Diffie Hellman (ECDH). After
the negotiation with the hello messages, the server delivers its certificate. The
key exchange is then initiated by the server with the “ServerKeyExchange”
message that contains the ECDH key material, so the client can derive an
ephemeral key. The “ServerKeyExchange” contains also a signature with the
server key that verifies with the certificate delivered in the “Certificate” message. The client can derive an ephemeral key and deliver it to the server
using the “ClientKeyExchange” so both endpoints can derive a master key
according to DH. Alike the handshake presented in Fig. 1, no encrypted key
is delivered during the handshake enabling forward secrecy

DTLS - Traduzido do inglês-O Datagram Transport Layer Security é um protocolo de comunicação que fornece segurança para aplicativos baseados em datagramas, permitindo que eles se comuniquem de uma maneira projetada para impedir a interceptação, adulteração ou falsificação de mensagens.


RC4 era o algoritmo simétrico de criptografia de fluxo mais usado no software e era utilizado nos protocolos mais conhecidos, como Secure Socket Layers e WEP. 
Later on, it was discovered the possibility to recover plain text after observing big
TLS traffic and due to that, RC4 was permanently forbidden
in TLS.

When it comes to server certificates, in 2010 there were
more than 16.2 million servers listening at port 443 (HTTP
over TLS default port) but just 11.3 million (38%) were able
to respond to a SSL/TLS handshake and only 4.3 million had a
valid certificate. The rest, over the 60%, used either malformed
or unverifiable certificates. According to the EFF, some server
certificates had a valid signature but were signed with keys
from CAs known to be compromised time ago [75] whereas
others were issued to subject names as “localhost” or even IP
addresses.

The verifiable structure consists of a Merkle Tree (MT) [90].
Such a tree contains a hash of an object subject to verification
in every leaf. The existence of an object in the tree and the
order in which it was added to the tree can be verified by
means of the MT. To accomplish that, parent nodes in a MT
contain a hash that combines the hashes of their children and
continues until the root, that contains a hash combining the
hash of every descendant. In this way, any change in either
the content or the order of the leafs, alters the value of the
root

HTTP Public Key Pinning Protocol (HPKP) [114] lets client
detect when a trust or certificate chain has changed unexpectedly. HPKP defines an HTTP header that lets the UA to learn
which SubjectPublickKey structures should be present
in the certificate chain in future TLS connections with the
same server. The objective is to avoid MITM attacks based on
compromised certificates and it should be used in conjunction
with HSTS

