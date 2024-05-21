package main

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/controllers"
	"github.com/FelipeFTN/my-website/integrations"
	"github.com/FelipeFTN/my-website/server"
	"github.com/FelipeFTN/my-website/services"
)

func main() {
	// Get config
	cfg := config.Get()

	// Integrations
	integrations := integrations.New(cfg)

	// Service
	svc := services.New(cfg, integrations)

	// Controllers
	ctrl := controllers.New(cfg, svc)

	// Run server
	server.Run(cfg, ctrl)
}
