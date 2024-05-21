package services

import (
	"context"

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

// TODO: Change the response type later for a specific struct
func (self *Website) MyRepositories(ctx context.Context) (repos []integrations.RepositoryResponse, err error) {
	repos, err = self.intgr.Github.ListRepositories(ctx)
	if err != nil {
		return repos, err
	}

	return repos, nil
}
