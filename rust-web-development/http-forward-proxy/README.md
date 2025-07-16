Step 1

In this step your goal is to create a minimal forward proxy. When it’s done you can test it using curl like this:

% curl --proxy "http://localhost:8989" "http://httpbin.org/ip"

In this case we’re telling curl to send the request via our proxy server that is running locally on port 8989.

If you remember from the build your own curl coding challenge, curl will normally send a request that looks like this (I’m only showing the fields we’re interested in):

GET /ip HTTP/1.1
Host: eu.httpbin.org

When we tell it to use the proxy, the request changes and instead looks like this:

GET http://httpbin.org/ip HTTP/1.1
Host: eu.httpbin.org
Proxy-Connection: Keep-Alive

You can configure your proxy server however you like, logging as much or as little as you like, but this is the output I see from mine (running in another terminal):

% ./ccproxy
Starting proxy server on 127.0.0.1:8989
Request made. Target: httpbin.org:443 Client: 127.0.0.1:56157

To make the proxy server work you will need to do the following:

    Have your proxy server start up and listen for connections. In my case I decided to listen on port 8989.

    When a request is received parse it to extract the target host.

    Create a new socket / connection to the target server.

    Forward the request, minus the hop by hop headers.

    Change the GET request.

    Add the ‘X-Forwarded-For’ header.

    Read the response from the target server and set the correct response headers before,

    Sending the response to the client.

You can find more about the headers in RFC 7230 and on the MDN website.
Step 2

In this step your goal is to refuse to proxy a request to domain on the banned list. In other words like many of the old school corporate proxies used to, you should stop access to some sites that are deemed inappropriate/unsafe/nsfw.

I suggest your proxy reads a list of forbidden hosts from a forbidden-hosts.txt file and refuses to server any requests for them. Here’s how that looked for me, when I added Facebook to the ban list:

% curl --proxy "http://localhost:8989" -v "http://facebook.com/"
*   Trying 127.0.0.1:8989...
* Connected to localhost (127.0.0.1) port 8989 (#0)
> GET <http://facebook.com/> HTTP/1.1
> Host: facebook.com
> User-Agent: curl/8.1.2
> Accept: */*
> Proxy-Connection: Keep-Alive
>
< HTTP/1.1 403 Forbidden
< Content-Type: text/plain; charset=utf-8
< X-Content-Type-Options: nosniff
< Content-Length: 34
<
Website not allowed: facebook.com

Step 3

In this step your goal is to refuse to proxy a web page if certain content appears on the page. Again your proxy could read a list of banned words from a banned-words.txt file and refuse to serve the page if any of those words appear in the response.

For a test I’ve banned the word ‘dummy’ then tried:

% curl --proxy "http://localhost:8989" -v "http://dummyjson.com/"
*   Trying 127.0.0.1:8989...
* Connected to localhost (127.0.0.1) port 8989 (#0)
> GET http://dummyjson.com/ HTTP/1.1
> Host: dummyjson.com
> User-Agent: curl/8.1.2
> Accept: */*
> Proxy-Connection: Keep-Alive
>
< HTTP/1.1 403 Forbidden
< Content-Type: text/plain; charset=utf-8
< X-Content-Type-Options: nosniff
< Content-Length: 43
<
Website content not allowed.

Step 4

In this step your goal is to log all the traffic going through the proxy. For example your proxy server might log like this:

% ./ccproxy
Starting proxy server on 127.0.0.1:8989
Client: 127.0.0.1:62448 Request URL: http://httpbin.org/ip
127.0.0.1:62448   200 OK
Client: 127.0.0.1:62454 Request URL: http://httpbin.org/ip
127.0.0.1:62454   200 OK

Ideally you want to log this to a file and optionally to standard out. You should probably include a date and timestamp - I’ve removed them for clarity here.
Step 5

In this step your goal is to handle TLS so you can also proxy HTTPS requests. So far we’ve only looked at proxying HTTP requests, but most of the Internet is now using HTTPS for very good privacy reasons. We want our proxy server to support that too.

Here’s what happens if we don’t:

% curl --proxy "http://localhost:8989" "https://httpbin.org/ip"
curl: (56) CONNECT tunnel failed, response 400

So we want to extend the proxy server to support HTTPS, which would result in the request succeeding:

% curl --proxy "http://localhost:8989" "https://httpbin.org/ip"
{
  "origin": "82.61.42.63"
}

The error message gives us a hint of what we should do. we need to create a tunnel and support the CONNECT method request. You can see what curl is doing with the -v option:

% curl --proxy "http://localhost:8989" -v "https://httpbin.org/ip"
*   Trying 127.0.0.1:8989...
* Connected to localhost (127.0.0.1) port 8989 (#0)
* CONNECT tunnel: HTTP/1.1 negotiated
* allocate connect buffer
* Establish HTTP proxy tunnel to httpbin.org:443
> CONNECT httpbin.org:443 HTTP/1.1
> Host: httpbin.org:443

The key bit here is the CONNECT line: CONNECT httpbin.org:443 HTTP/1.1 which is asking the proxy to create a connection to the specified host and port. If that connection is established then the proxy will send back a 200 response letting the client know that a tunnel has now been established between the client and the end host.

To implement the tunnel a proxy will need to read data sent to it by the client and forward it to the end server, and in return, read data sent by the end server and forward it to the client. Curl or another client can then send TCP traffic from the client to the end server, including TLS traffic via the tunnel.

Once you have this working you can adjust your computer to route all your web traffic through your proxy - if that works, congratulations you’ve built a forward proxy server.

Going Further

To get even more out of this challenge consider two things:

    The connection between your client and your proxy server is not running over TLS - how would you extend the proxy server to provide a secure connection?

    If a proxy server provides TLS termination between your client and the end server then so could another proxy server. That proxy server could be malicious or could just be insecure. So be careful when browsing the web through a machine that uses a proxy server even if the connection is over HTTPS it might not be secure.
