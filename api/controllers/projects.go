package controllers

import (
	"fmt"

	"github.com/gin-gonic/gin"
)

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

func (self *Website) CollabRepositories(c *gin.Context) {
	ctx := c.Request.Context()

	repos, err := self.svc.CollabRepositories(ctx)
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
