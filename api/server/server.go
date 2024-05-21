package server

import (
	"github.com/gin-gonic/gin"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/controllers"
	"github.com/FelipeFTN/my-website/server/middlewares"
)

func Run(cfg *config.Config, ctrl *controllers.Controller) {
	// Set server mode
	gin.SetMode(cfg.Server.Mode)

	// Initiate server
	server := gin.Default()

	// Middlewares
	server.Use(middlewares.Cors())

	// Routes
	Routes(server, ctrl)

	// Run server
	server.Run(":" + cfg.Server.Port)
}

func Routes(server *gin.Engine, ctrl *controllers.Controller) {
	// Route v1 should slowly disappear
	v1 := server.Group("v1")
	{
		// Internal Routes
		v1.GET("/health", Health)

		// Middlewares
		v1.Use()

		// Main Routes
		v1.GET("/my_repositories", ctrl.Website.MyRepositories)
	}
}

func Health(c *gin.Context) {
	c.JSON(200, gin.H{
		"status": true,
		"data":   "health successfully",
		"code":   200,
	})
}
