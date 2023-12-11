package main

import (
	"fmt"
	"log"
	"net"

	api "github.com/wilmacedo/streaming/go/proto"
	"google.golang.org/grpc"
)

func main() {
	LoadDatabase("database.json")

	lis, err := net.Listen("tcp", ":8000")
	if err != nil {
		log.Fatalf("cannot create listener: %s", err)
	}
	serverRegistrar := grpc.NewServer()
	service := &APIServer{}

	api.RegisterAPIServer(serverRegistrar, service)

	fmt.Println("🚀 Running gRPC server at port localhost:8000")
	err = serverRegistrar.Serve(lis)
	if err != nil {
		log.Fatalf("impossible to serve: %s", err)
	}
}
