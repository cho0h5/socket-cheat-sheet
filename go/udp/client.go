package main

import (
        "log"
        "net"
       )

func main() {
    raddr, _ := net.ResolveUDPAddr("udp", ":8080")
    conn, _ := net.DialUDP("udp", nil, raddr)

    conn.Write([]byte("hello, server"))
    buffer := make([]byte, 1024)
    n, _, _ := conn.ReadFromUDP(buffer)
    log.Println("from server:", string(buffer[:n]))
}
