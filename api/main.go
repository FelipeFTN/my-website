package main

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/server"
)

func main() {
	// Get config
	cfg := config.Get()

	// Run server
	server.Run(cfg)
}
