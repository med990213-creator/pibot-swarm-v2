package main

import (
	"fmt"
	"net"
	"time"
)

// PiEdge - النسخة الخفيفة من سرب Pi للأجهزة الضعيفة
func main() {
	fmt.Println("🥧 Pi-Edge Core v1.0 - Sovereign AI (Go Version)")
	fmt.Println("-----------------------------------------------")

	target := "127.0.0.1" // الهدف الافتراضي للفحص
	ports := []int{22, 80, 443, 3000, 8080, 8888}

	fmt.Printf("🔍 Starting light-recon on %s...\n", target)

	for _, port := range ports {
		scanPort(target, port)
	}

	fmt.Println("\n✅ Recon completed by Pi-Edge.")
}

func scanPort(ip string, port int) {
	address := fmt.Sprintf("%s:%d", ip, port)
	conn, err := net.DialTimeout("tcp", address, 2*time.Second)

	if err != nil {
		// Port is closed or filtered
		return
	}
	conn.Close()
	fmt.Printf("🎯 [OPEN] Port %d detected!\n", port)
}
