package main

import (
        "net"
       )

func main() {
    ln, _ := net.Listen("tcp", ":8080")
    defer ln.Close()

    buffer := make([]byte, 1024)
    for {
        conn, _ := ln.Accept()

        conn.Read(buffer)
        conn.Write([]byte("hello, client"))
    }
}
