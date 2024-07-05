package mocks

import "github.com/FelipeFTN/my-website/models"

func ListRepositories() []models.RepositoryResponse {
	return []models.RepositoryResponse{
		{
			Name:            "my-website",
			RepoOwner:       "FelipeFTN",
			Description:     "My personal website",
			StargazersCount: 8,
			ForksCount:      2,
			Language:        "Rust",
		},
		{
			Name:            "my-website-api",
			RepoOwner:       "FelipeFTN",
			Description:     "My personal website API",
			StargazersCount: 10,
			ForksCount:      4,
			Language:        "Go",
		},
		{
			Name:            "repository-with-a-very-long-name-that-should-be-truncated",
			RepoOwner:       "FelipeFTN",
			Description:     "This is a repository with a very long name that should be truncated",
			StargazersCount: 0,
			ForksCount:      0,
			Language:        "JavaScript",
		},
	}
}
