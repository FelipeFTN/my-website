package services

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/integrations"
)

type Service struct {
	cfg     *config.Config
	Website *Website
}

func New(cfg *config.Config, intgr *integrations.Integration) *Service {
	service := new(Service)

	service.cfg = cfg
	service.Website = NewWebsite(cfg, intgr)

	return service
}
