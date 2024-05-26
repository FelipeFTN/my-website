package config

import "github.com/FelipeFTN/my-website/models"

// array of models.RepositoryResponse
// with fixed repositories list
var PinnedRepositories = []models.RepositoryResponse{
	{Name: "Kernel"},
	{Name: "Computer-Science"},
	{Name: "repository-with-a-very-long-name-that-should-be-truncated"},
}
