package integrations

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/FelipeFTN/my-website/config"
	"github.com/FelipeFTN/my-website/models"
)

type Github struct {
	cfg *config.Config
}

func NewGithub(cfg *config.Config) *Github {
	return &Github{cfg: cfg}
}

func (self *Github) ListRepositories(ctx context.Context) ([]models.RepositoryResponse, error) {
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

	return repos, nil
}
