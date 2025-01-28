package services

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/integrations"
)

type Website struct {
	cfg   *config.Config
	intgr *integrations.Integration
}

func NewWebsite(cfg *config.Config, intgr *integrations.Integration) *Website {
	return &Website{
		cfg:   cfg,
		intgr: intgr,
	}
}
