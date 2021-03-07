package main

import (
        "net"
        "log"
       )

func main() {
    laddr, _ := net.ResolveUDPAddr("udp", ":8080")
    conn, _ := net.ListenUDP("udp", laddr)

    buffer := make([]byte, 1024)
    n, addr, err := conn.ReadFromUDP(buffer)
    if err != nil {
        log.Println(err)
    }
    log.Println("from client:", string(buffer[:n]))
    conn.WriteToUDP([]byte("hello, client"), addr)
}
