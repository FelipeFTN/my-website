package controllers

import (
	"fmt"

	"github.com/gin-gonic/gin"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/services"
)

type Website struct {
	cfg *config.Config
	svc *services.Website
}

func NewWebsite(cfg *config.Config, svc *services.Website) *Website {
	return &Website{
		cfg: cfg,
		svc: svc,
	}
}

func (self *Website) MyRepositories(c *gin.Context) {
	ctx := c.Request.Context()

	repos, err := self.svc.MyRepositories(ctx)
	if err != nil {
		fmt.Println(err)
		c.JSON(503, gin.H{
			"message": "Error fetching repositories",
			"error":   err.Error(),
			"status":  false,
		})
		return
	}

	c.JSON(200, gin.H{
		"message": "Repositories fetched successfully",
		"status":  true,
		"data":    repos,
	})
}
