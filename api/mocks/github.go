package mocks

import "github.com/FelipeFTN/my-website/models"

func ListRepositories() []models.RepositoryResponse {
	return []models.RepositoryResponse{
		{
			Name:            "my-website",
			Description:     "My personal website",
			StargazersCount: 8,
			ForksCount:      2,
		},
		{
			Name:            "my-website-api",
			Description:     "My personal website API",
			StargazersCount: 10,
			ForksCount:      4,
		},
		{
			Name:            "repository-with-a-very-long-name-that-should-be-truncated",
			Description:     "This is a repository with a very long name that should be truncated",
			StargazersCount: 0,
			ForksCount:      0,
		},
	}
}
