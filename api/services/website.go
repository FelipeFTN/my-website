package services

import (
	"context"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/integrations"
	"github.com/FelipeFTN/my-website/models"
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

func (self *Website) MyRepositories(ctx context.Context) (repos []models.RepositoryResponse, err error) {
	repos, err = self.intgr.Github.ListRepositories(ctx)
	if err != nil {
		return repos, err
	}

	return repos, nil
}
