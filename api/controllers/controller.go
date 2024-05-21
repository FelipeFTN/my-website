package controllers

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/services"
)

type Controller struct {
	cfg     *config.Config
	Website *Website
}

func New(cfg *config.Config, svc *services.Service) *Controller {
	controller := new(Controller)

	controller.cfg = cfg
	controller.Website = NewWebsite(cfg, svc.Website)

	return controller
}
