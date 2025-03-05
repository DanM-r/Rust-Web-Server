# Project Research

Here the project reaserch prior to start programming the web server lies.

## Table of Contents

- [Project Research](#project-research)
  - [Table of Contents](#table-of-contents)
  - [Concepts](#concepts)
    - [Hyper Text Transfer Protocol 1.1 (HTTP/1.1)](#hyper-text-transfer-protocol-11-http11)

## Concepts

A few concepts are needed to be learned before starting this project, specially
how internally a web server and the http protocol work. To begin this
investigation, i will learn more about the HTTP protocol from the following
sources: [*Mobile Directory Number (MDN)*][1] and *Request For Comments
(MDN)* in the [RFC 9112][4], [RFC 9111][3] and the [RFC 9110][2] pages to gather
the neccesary technical information about how this protocol works internally.

[1]: https://developer.mozilla.org/en-US/docs/Web/HTTP
[2]: https://datatracker.ietf.org/doc/html/rfc9110
[3]: https://datatracker.ietf.org/doc/html/rfc9111
[4]: https://datatracker.ietf.org/doc/html/rfc9112

### Hyper Text Transfer Protocol 1.1 (HTTP/1.1)
A stateless, application-level, resquest/response protocol with extensible
semantics and self descriptive messages for trasnmitting hypertext or hypermedia
documents like HTML. It is for:

- Web browser to web server communication.
- Machine-to-machine communication.
- Programmatic access to API.

Follows the client-server model, a distributed architecture that partitions
tasks and workloads between servers and service requesters between clients. A
client opens a connection to make a request and waits for the server response
to arrive. A **complete document** is constructed by resources like text, video,
images, scripts, etc. Rely on the TCP transport protocol and the TLS-encryption.

- **Requests**: messages from client, usually user-agent (web browser usually) or
  proxy, to server. 
- **Responses**: messages from server to client.

![clientserver](./media/clientserver.png)

