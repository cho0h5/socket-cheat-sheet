package main

import (
        "log"
        "net"
        "time"
       )

func main() {
    conn, _ := net.Dial("tcp", ":8080")

    buffer := make([]byte, 1024)

    start := time.Now()
    conn.Write([]byte("echo echo echo"))
    n, _ := conn.Read(buffer)
    elapsed := time.Since(start)

    log.Println(elapsed)
    log.Println("return:", string(buffer[:n]))
}
