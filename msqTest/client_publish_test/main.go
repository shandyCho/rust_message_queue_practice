package main

import (
	"encoding/json"
	"fmt"
	"net"
	"strconv"
	"sync"
	"time"
)

func main() {
	conn, err := net.Dial("tcp", "192.168.28.117:8080")
	fmt.Println("Connecting to server...")
	if err != nil {
		fmt.Println("Connection error:", err)
		panic(err)
	}
	// defer connectionClose(conn)

	var wg sync.WaitGroup
	wg.Add(1)

	go tcpConnect(conn, &wg)
	go tcpConnect(conn, &wg)
	go tcpConnect(conn, &wg)
	go tcpConnect(conn, &wg)
	go tcpConnect(conn, &wg)
	wg.Wait()
}

func tcpConnect(c net.Conn, wg *sync.WaitGroup) {

	for i := 1; i <= 100000; i++ {
		send := make(map[string]interface{})
		send["senderAddress"] = "127.0.0.1:80"
		sendData := make(map[string]interface{})
		sendData["cmd"] = "test"
		sendData["time"] = time.Now().Unix()
		sendData["name"] = "from go" + strconv.Itoa(i)
		send["data"] = sendData
		fmt.Println(send)
		jsonBytes, err := json.Marshal(send)
		jsonBytes = append(jsonBytes, '\n')
		fmt.Println("Prepared data")
		if err != nil {
			fmt.Println("JSON Marshal error:", err)
			panic(err)
		}

		fmt.Println("Sending data...")
		_, err = c.Write(jsonBytes)
		fmt.Println("Sent data:", string(jsonBytes))
		if err != nil {
			fmt.Println("Failed to write data : ", err)
			break
		}
		// time.Sleep(1 * time.Second)
	}
	connectionClose(c)

	wg.Done()
}

func connectionClose(conn net.Conn) {
	fmt.Println("Closing connection...")
	conn.Close()
}
