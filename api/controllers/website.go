package controllers

import (
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
