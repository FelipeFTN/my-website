package models

type RepositoryResponse struct {
	Name            string `json:"name"`
	RepoOwner       string `json:"repo_owner,omitempty"`
	Description     string `json:"description"`
	StargazersCount int    `json:"stargazers_count"`
	ForksCount      int    `json:"forks_count"`
	Language        string `json:"language"`
	URL             string `json:"html_url"`
}
