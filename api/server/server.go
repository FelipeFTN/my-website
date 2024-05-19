package server

import (
	"github.com/gin-gonic/gin"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/server/middlewares"
)

func Run(cfg *config.Config) {
	// Set server mode
	gin.SetMode(cfg.Server.Mode)

	// Initiate server
	server := gin.Default()

	// Middlewares
	server.Use(middlewares.Cors())

	// Routes
	Routes(server)

	// Run server
	server.Run(":" + cfg.Server.Port)
}

func Routes(server *gin.Engine) {
	// Ping endpoint
	server.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})
}
