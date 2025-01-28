package integrations

import (
	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/infra"
)

type Integration struct {
	Github *Github
}

func New(cfg *config.Config, infra *infra.Infra) *Integration {
	integration := new(Integration)

	integration.Github = NewGithub(cfg, infra.Cache)

	return integration
}
