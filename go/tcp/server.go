package main

import (
        "net"
        "log"
       )

func main() {
    ln, _ := net.Listen("tcp", ":8080")
    defer ln.Close()

    conn, _ := ln.Accept()

    buffer := make([]byte, 1024)
    n, _ := conn.Read(buffer)
    log.Println("from client:", string(buffer[:n]))
    conn.Write([]byte("hello, client"))
}
