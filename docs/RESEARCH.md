# Project Research

Here the project reaserch prior to start programming the web server lies.

## Table of Contents

- [Project Research](#project-research)
  - [Table of Contents](#table-of-contents)
  - [Concepts](#concepts)
  - [Hyper Text Transfer Protocol 1.1 (HTTP/1.1)](#hyper-text-transfer-protocol-11-http11)
    - [Components](#components)
    - [Connections](#connections)
      - [Flow from client standpoint](#flow-from-client-standpoint)
    - [HTTP Message Format](#http-message-format)
    - [HTTP Message Parsing](#http-message-parsing)
      - [Parsing considerations](#parsing-considerations)
      - [Version Considerations](#version-considerations)
      - [Request line considerations](#request-line-considerations)
      - [Target URI](#target-uri)
      - [Status-line considerations](#status-line-considerations)
  - [Fetch API](#fetch-api)

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

## Hyper Text Transfer Protocol 1.1 (HTTP/1.1)

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
All communication occurs over messages.

- **Requests**: messages from client, usually user-agent (web browser usually)
  or proxy, to server.
- **Responses**: messages from server to client.

![clientserver](./media/clientserver.png)

### Components

- **Client (user-agent)**: tool that acts on behalf of the user, mostly a web
browser, a bot or a program to debug applications.

  1. Sends a request to the server for the HTML document
  2. Sends additional requests to parse the other parts of the document.
  3. Browser combines resources to present the complete document.
  4. Scripts in the browser can fetch more resources.

  - Hypertext means some parts are links that can fetch another web page.

- **Web server**: serves the document, could be multiple servers doing that
job. Could share the same address.

- **Proxies**: Many other intermediate machines that work at different levels
of the TCP/IP stack, have a significant impact on performance. Can do numerous
functions:

  - caching
  - filtering
  - load balancing
  - authentication
  - logging

### Connections

Connections are at the transport layer, out of scope for the protocol. Uses TCP.
Before a request, a TCP conection must be established. Default behaviour is a
TCP connection for each request. With *pipelining* (Hard to implement) and
*persistent connections*. Persistent connections TCP connection can be
controlled using the `Connection` header.

#### Flow from client standpoint

1. Open a TCP Connection
2. Send an HTTP message
3. Read the response
4. Close or reuse connection.

### HTTP Message Format

Both request and response are messages, and the RFC defines clearly how a
message is composed of.

```HTTP
start-line CRLF
*( field-line CRLF )
CRLF
[ message-body ]
```

The only difference between HTML request and response is in its start-line. For
requests, the start-line is named **request-line**, and for responses it is
named **status-line**. Implementation limits what a server or a client can
expect: client a response, server a request.

- *request-line*: `method SP request-target SP HTTP-version` e.g. `GET / HTTP/
1.1`
  - *method*: operation the client wants to perform.
  - *request-line*: path of the resource to fetch.
  - *HTTP-version*: version of protocol
- *status-line*: `HTTP-version SP status-code SP [ reason-phraase ]` e.g.
`HTTP/1.1 200 OK`
  - *status-code*: indicates the request was succesful or not and why.
  - *reason-phrase*: `1*( HTAB / SP / VCHAR / obs-text )` a short description of
  the status code.
- *field-line*: `field-name ":" 1*( OWS / OS ) field-value 1*( OWS / OS )`
  
### HTTP Message Parsing

1. Start line into a structure
2. Each header field into a hash table by field name until empyt line
3. Based on parsed data, if body is expected proceed
4. Read as a stream until amount of octets equals message length or connection
is closed.

#### Parsing considerations

- Use ASCII encoding. Unicode encoding produces security vulnerabilities
- String based parsers can be used once the elements where extracted
- The line terminator is CRLF
- Invalid request if CR appears. Maybe replace CR with SP
- Invalid request with an extra CRLF at end of line
- Invalid request with trailling whitespaces between start-line and
first field-line.
- Bad request, if server receives a proper start-line but a badly formed http
grammar.

#### Version Considerations

- 1.1 compatible with 1.0 while ignoring newer features.
- Each intermediary must rewrite the HTTP version with their own.
- A server may respond with 1.0 a 1.1 request if client is suspected to
incorrectly implement 1.1.

#### Request line considerations

- Invalid if other whitespace octet or more than one SP.
- Not Implemented of server cannot process a request-line longer than it is
capable of.
- Servers at minimum implement a request-line of 8000 octets
- Default methods:

  | Method | Optional | Description |
  | --- | --- | --- |
  | GET | no | Get representation of resource |
  | HEAD | no | Get without transfering |
  | POST | yes | Processing of resource |
  | PUT | yes | Replace resource |
  | DELETE | yes | Remove resource |
  | CONNECT | yes | Establish tunnel to the server on target |
  | OPTIONS | yes | Communication options for resource |
  | TRACE | yes | Message loop-back test along resource |

- Some methods might not be implemented (Not-implemented) and some may not be
allowed (Method Not allowed).
- Bad request or Moved permanently if the target contains whitespaces. Respond
with properly formed target.
- Bad request if request doesnt contain a host header
- If authority component is given the host header must contain same value
- Bad request for any request with multiple host headers
- Target syntax:
  - origin-form = `absolute-path [ '?' query ]`, which the absolute-path is the
  the URI of the resource. If empty `/`.
  - absolute-form: the whole URL. Always send the host header but the server
  and proxy must ignore the header and instead use the target.
  - authority-form: `uri-host:port` only for CONNECT requests, for establishing
  connection to a proxy.
  - asterisk-form: `*` only for OPTIONS requests server-wide. The last proxy
  must send a request with the asterisk-form after receiving a absolute response

#### Status-line considerations

- May parse whitespace instead of SP, but like SP. (unsafe)
- status-code is a 3-digit integer.
- client should ignore the reason-phrase

#### Field-line considerations

Parsed using generic algorithm. Values are not parsed until a later stage of
message interpretation.

- Bad request if whitespace between field-name and colon.
- Proxies must remove any whitespace from a response message.
- Line folding should happen only within "message/http" container.
- Bad request or (pref) Obsolete if line folding is received in origin server.
- Bad gateway if line folding is received in proxy or gateway.
 
#### Body considerations

Signaled by `Content-Length` and `Transfer-Encoding` headers.

### Target URI

Is the target-request in absolute-form. Server must reconstruct the URI.

- If server's configuration implements a URI scheme, its implemented. Otherwise,
if the connection is secure: https, if not http.
- If authority, its also URI unless its invalid or empty which will be empty.
- If authority or asterisk, path+query will be empty, otherwise path+query is
URI.
- if scheme requires non-empty authority. server may reject message, interpret
through context or apply default values (unsafe).

```HTTP
scheme :// authority path [ query ]
```

Then the server should try see if the URI is valid or reaching an existing
resource that its willing to send a response.

## Fetch API

Allows making HTTP requests from JavaScript.
