package integrations

import "github.com/FelipeFTN/my-website/config"

type Integration struct {
	Github *Github
}

func New(cfg *config.Config) *Integration {
	integration := new(Integration)

	integration.Github = NewGithub(cfg)

	return integration
}
