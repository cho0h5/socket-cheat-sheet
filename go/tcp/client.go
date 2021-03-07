package main

import (
        "log"
        "net"
       )

func main() {
    conn, _ := net.Dial("tcp", ":8080")

    conn.Write([]byte("hello, server"))
    buffer := make([]byte, 1024)
    n, _ := conn.Read(buffer)
    log.Println("from server:", string(buffer[:n]))
}
