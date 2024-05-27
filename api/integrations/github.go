package integrations

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/infra"
	"github.com/FelipeFTN/my-website/models"
)

type Github struct {
	cfg   *config.Config
	cache *infra.Cache
}

func NewGithub(cfg *config.Config, cache *infra.Cache) *Github {
	return &Github{
		cfg:   cfg,
		cache: cache,
	}
}

func (self *Github) ListRepositories(ctx context.Context) ([]models.RepositoryResponse, error) {
	// Check if the data is in the cache
	c, exist := self.cache.Get("github_repos")
	if exist {
		var repos []models.RepositoryResponse
		if err := json.Unmarshal([]byte(c), &repos); err != nil {
			return nil, err
		}
		fmt.Println("Data fetched from cache")

		return repos, nil
	}

	// If not, fetch from the API
	url := fmt.Sprintf("https://api.github.com/users/%s/repos", self.cfg.Github.Username)

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return nil, err
	}

	req.Header.Set("Accept", "application/vnd.github.v3+json")
	req.Header.Set("X-GitHub-Api-Version", "2022-11-28")
	req.Header.Set("User-Agent", "FelipeFTN")
	req.Header.Set("Authorization", self.cfg.Github.Token)

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("API request failed with status code %d", resp.StatusCode)
	}

	var repos []models.RepositoryResponse
	if err := json.NewDecoder(resp.Body).Decode(&repos); err != nil {
		return nil, err
	}

	// Save the data in the cache
	data, err := json.Marshal(repos)
	if err != nil {
		return nil, err
	}
	self.cache.Set("github_repos", string(data))

	return repos, nil
}
