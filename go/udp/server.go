package main

import (
        "net"
        "log"
       )

func main() {
    laddr, _ := net.ResolveUDPAddr("udp", ":8080")
    conn, _ := net.ListenUDP("udp", laddr)

    buffer := make([]byte, 1024)
    n, _ := conn.Read(buffer)
    log.Println("from client:", string(buffer[:n]))
    conn.Write([]byte("hello, client"))
}
