package main

import (
        "net"
       )

func main() {
    laddr, _ := net.ResolveUDPAddr("udp", ":8080")

    buffer := make([]byte, 1024)

    conn, _ := net.ListenUDP("udp", laddr)

    _, addr, _ := conn.ReadFromUDP(buffer)
    conn.WriteToUDP(buffer, addr)
}
