package services

import (
	"context"
	"sort"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/integrations"
	"github.com/FelipeFTN/my-website/mocks"
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
	// Mock repositories for development
	repos = mocks.ListRepositories()

	// If the environment is not development, fetch repositories from Github
	// NOTE: Github API has a rate limit of 60 requests per hour,
	// completely trash. I can't test this code without hitting the rate limit
	// So, I'm going to mock the response for development.
	if self.cfg.Server.Environment != "development" {
		repos, err = self.intgr.Github.ListRepositories(ctx)
		if err != nil {
			return repos, err
		}
	}

	// Sort repositories by stargazers count
	// I want to show the most popular repositories first
	sort.Slice(repos, func(i, j int) bool {
		return repos[i].StargazersCount > repos[j].StargazersCount
	})

	// This will remove some repositories based on some filters
	for i := 0; i < len(repos); i++ {
		if repos[i].StargazersCount == 0 && !isPinned(repos[i].Name) {
			repos = remove(repos, i)
			continue
		}
		if repos[i].Language == "" {
			repos = remove(repos, i)
			continue
		}
	}

	return repos, nil
}

func (self *Website) CollabRepositories(ctx context.Context) (repos []models.RepositoryResponse, err error) {
	return mocks.ListCollabRepositories(), nil
}

// This function removes the element at index i from slice
// In this case, it will remove repositories
func remove(slice []models.RepositoryResponse, s int) []models.RepositoryResponse {
	return append(slice[:s], slice[s+1:]...)
}

// This function checks if a repository is pinned
// This means that the repository will always be shown
// in the website, regardless of the stargazers count.
// The pinned repositories are defined in the config file
func isPinned(n string) bool {
	p := config.PinnedRepositories
	for _, v := range p {
		if v.Name == n {
			return true
		}
	}
	return false
}
