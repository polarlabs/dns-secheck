
# DNS Security Check

## Tests

0. Check if the server status is okay.

1. Send arbitrary data to the server on port 53 (raw TCP / UDP exfiltration)
   1. via UDP
   2. via TCP

If test 1 passes, port 53 is open for arbitrary data.

2. Send a DNS request to the server on port 53
   1. via UDP
   2. via TCP

If test 1 fails and test 2 passes, packets on port 53 are being filtered for DNS protocol.




# TCP

On the server side:

socat TCP-LISTEN:53,bind=[LOCAL_IP],fork,reuseaddr STDOUT

This will start a TCP server on port 53 which allows simultaneous 
connections and print any input to STDOUT.

Connect to the server with:

netcat [SERVER_IP] 53

<- This can be done multiple times.

Now, type something in to the clients' side terminal. This will appear 
on the server side on STDOUT.

# UDP

socat UDP-LISTEN:53,bind=[LOCAL_IP],fork,reuseaddr STDOUT

netcat -u [SERVER_IP] 53

<- This can be done multiple times.


# Ping / Pong

socat TCP-LISTEN:53,bind=93.177.64.153,fork,reuseaddr SYSTEM:'while read x; do echo "$x" >&2; [ "$x" = "ping" ] && echo pong; done'

netcat 93.177.64.153 53

socat - TCP:93.177.64.153:53


socat UDP-RECVFROM:53,bind=93.177.64.153,fork,reuseaddr SYSTEM:'while read x; do echo "$x" >&2; [ "$x" = "ping" ] && echo pong; done'




# Testsetup

echo -ne "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 15\r\n\r\n{\"status\":\"ok\"}" > response.txt

socat TCP-LISTEN:53,bind=93.177.64.153,fork,reuseaddr SYSTEM:'while read x; do echo "$x" >&2; [ "$x" = "ping" ] && echo pong; done' & 
socat UDP-RECVFROM:53,bind=93.177.64.153,fork,reuseaddr SYSTEM:'while read x; do echo "$x" >&2; [ "$x" = "ping" ] && echo pong; done' &
socat TCP-LISTEN:80,bind=93.177.64.153,fork,reuseaddr SYSTEM:'cat response.txt'



# Architecture

UDP socket
│
multiplexer
│
parse DNS with Hickory
│
build DNS response
│
send response

# Build

    cargo build --release --target x86_64-pc-windows-gnu