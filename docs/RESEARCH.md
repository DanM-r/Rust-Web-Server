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
      - [Status-line considerations](#status-line-considerations)
      - [Field-line considerations](#field-line-considerations)
      - [Body considerations](#body-considerations)
    - [HTTP Message Body Parsing](#http-message-body-parsing)
      - [Transfer-Encoding](#transfer-encoding)
      - [Status Codes](#status-codes)
      - [Content-Length](#content-length)
    - [HTTP Message Body Length](#http-message-body-length)
    - [Handling incomplete messages](#handling-incomplete-messages)
    - [Connection Management](#connection-management)
    - [Associating a response to a request](#associating-a-response-to-a-request)
    - [Persistence](#persistence)
    - [Retrying requests](#retrying-requests)
    - [Pipelining](#pipelining)
    - [Concurrency](#concurrency)
    - [Target URI](#target-uri)
  - [Fetch API](#fetch-api)

## Concepts

A few concepts are needed to be learned before starting this project, specially
how internally a web server and the http protocol work. To begin this
investigation, i will learn more about the HTTP protocol from the following
sources: [*Mobile Directory Number (MDN)*][1] and *Request For Comments
(MDN)* in the [RFC 9112][4], [RFC 9111][3] and the [RFC 9110][2] pages to gather
the neccesary technical information about how this protocol works internally.
As well, some other documentation like [the web server concept][5]

[1]: https://developer.mozilla.org/en-US/docs/Web/HTTP
[2]: https://datatracker.ietf.org/doc/html/rfc9110
[3]: https://datatracker.ietf.org/doc/html/rfc9111
[4]: https://datatracker.ietf.org/doc/html/rfc9112
[5]: https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/What_is_a_web_server

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

- **Transfer encoding**: lists coding names that can be applied to form the
message body.
- Server must be able to parse the chunked coding.
- Sender must not apply the chunked coding more than once to a body.
- Sender must apply chunked as the final message, if any other coding  `

### HTTP Message Body Parsing

#### Transfer-Encoding

Lists the transfer coding names to apply to the content. To delimit dynamically generated content.

- Always implement the *chunked* coding.
- Sender must not apply *chunked* more than once
- Sender must apply *chunked* as the final coding, if any other coding is
specified.
- Additional coding names can be included in the response.
- May be sent as a response to a HEAD or 304 GET
- Must not send in any 1xx or 204 or any 2xx of CONNECT
- Server with not implemented coding names should responde with 501
- Client must not send a request for a server that cannot handle version 1.1
- Server must reject or use the Transfer-Encoding only if the Content-Length is specified.
- If server or client receives a 1.0 message with Transfer encoding must treat the frame as faulty.
  
#### Status Codes

- 1xx: Informational, request received process continues
- 2xx: Successful, request successfully received, understood and accepted.
- 3xx: Redirection, further action required
- 4xx: Client error, request contains bad syntax or cannot be fulfilled.
- 5xx: Server error, server failed to fulfill an apparently valid request.

#### Content-Length

When a message does not contains the Transfer-Encoding.

### HTTP Message Body Length

1. Any response to HEAD, 204, 304 -> terminated by the first empty line after header fields.
2. Any 2xx to CONNECT -> empty line concldes the header fields, Transfer-Encoding and Content-Length should be ignored.
3. Transfer-Encoding and Content-Length -> Bad request
4. Transfer-Encoding and chunked final coding -> reading and decoding data until complete
5. Transfer-Encoding and no chnked final -> Bad request
6. Content-Length invalid -> Bad request or Bad Gateway or close connection
7. Content-Length and no Trasfer-Encoding -> decimal value defines length in octets
8. request and none above true -> 0 length
9. response -> number octets received prior server closing connection.

### Handling incomplete messages

May send an error response. An incomple message -> premature close of connection, chucked decoding fails or terminates in the middle of header section. An
incomplete message body -> zero-sized chunk not received. Valid Content-Length
incomplete -> size of message is less than the value given by content-length.

### Connection Management

Connection protocols are managed based on client configuration. Includes
maintaining the state of current connections, establishing a new connection,
or reusing an existing connection, processing messages received, detecting
connection failures and closing each connection. Shold be able to maintain many
connections in parallel and control request queues.

### Associating a response to a request

- No request Identifier
- Order of arrival.
- More than one response per request only happens for 1xx status codes.
- For multiple outsanding requests, client must hold a list of them.
- Close connection if any data is received when no outsanding requests were
active.

### Persistence

Default. Multiple requests over a same connection.

- If close connection option is present, the connection will not persist.
- for 1.1 or later, in 1.0 requires keep-alive option for non proxy and requests
- Server must read the entire body of request
- Proxy must not maintain a persistent connection with a 1.0

### Retrying requests

Anticipate the need to recover from asynchronous close events.

### Pipelining

Sending multiple requests without waiting for each response.

- Server processng in parallel if they all have safe methods.
- Same order of responses as the order requests have arrived.
- Should retry unanswered requests if connection closes before receiving all the
responses. Must not pipeline immediately after connection establishment.
- Must not pipeline after non-idempotent method.
- Intermediary may pipeline requests the pipelined requests it reeceives.

### Concurrency

- Client should limit the amount of simultaneous connections.
- No max connections
- Can cause congestion
- Reject if contents is abusive or DoS.

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
