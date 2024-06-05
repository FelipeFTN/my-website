package mocks

import "github.com/FelipeFTN/my-website/models"

func ListCollabRepositories() []models.RepositoryResponse {
	return []models.RepositoryResponse{
		{
			Name:            "commitgpt",
			Description:     "CommitGPT is a command-line tool that generates a commit message based on the changes in the git diff, following the conventional commits standard.",
			StargazersCount: 9,
			ForksCount:      2,
			Language:        "Rust",
			URL:             "https://github.com/loadfms/commitgpt",
		},
		{
			Name:            "emoji-selector-for-gnome",
			Description:     "This extension provide a popup menu with some emojis ; clicking on an emoji copies it to the clipboard. ",
			StargazersCount: 469,
			ForksCount:      76,
			Language:        "JavaScript",
			URL:             "https://github.com/maoschanz/emoji-selector-for-gnome",
		},
		{
			Name:            "unsplash",
			Description:     "Every run a new wallpaper for you! 🌄",
			StargazersCount: 0,
			ForksCount:      0,
			Language:        "Go",
			URL:             "https://github.com/loadfms/unsplash",
		},
	}
}
