package main

import (
        "log"
        "net"
        "time"
       )

func main() {
    raddr, _ := net.ResolveUDPAddr("udp", ":8080")
    conn, _ := net.DialUDP("udp", nil, raddr)

    buffer := make([]byte, 1024)

    start := time.Now()
    conn.Write([]byte("echo echo echo"))
    n, _, _ := conn.ReadFromUDP(buffer)
    elapsed := time.Since(start)

    log.Println(elapsed)
    log.Println("return:", string(buffer[:n]))
}
