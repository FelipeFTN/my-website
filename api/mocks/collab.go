package mocks

import "github.com/FelipeFTN/my-website/models"

func ListCollabRepositories() []models.RepositoryResponse {
	return []models.RepositoryResponse{
		{
			Name:            "dunst",
			RepoOwner:       "dunst-project",
			Description:     "A lightweight and customizable notification-daemon",
			StargazersCount: 5230,
			ForksCount:      362,
			Language:        "C",
			URL:             "https://github.com/dunst-project/dunst",
		},
		{
			Name:            "commitgpt",
			RepoOwner:       "loadfms",
			Description:     "CommitGPT is a command-line tool that generates a commit message based on the changes in the git diff, following the conventional commits standard.",
			StargazersCount: 9,
			ForksCount:      2,
			Language:        "Go",
			URL:             "https://github.com/loadfms/commitgpt",
		},
		{
			Name:            "emoji-selector-for-gnome",
			RepoOwner:       "maoschanz",
			Description:     "This extension provide a popup menu with some emojis ; clicking on an emoji copies it to the clipboard. ",
			StargazersCount: 469,
			ForksCount:      76,
			Language:        "JavaScript",
			URL:             "https://github.com/maoschanz/emoji-selector-for-gnome",
		},
		{
			Name:            "unsplash",
			RepoOwner:       "loadfms",
			Description:     "Every run a new wallpaper for you! 🌄",
			StargazersCount: 0,
			ForksCount:      0,
			Language:        "Go",
			URL:             "https://github.com/loadfms/unsplash",
		},
		{
			Name:            "fastcompmgr",
			RepoOwner:       "tycho-kirchner",
			Description:     "A fast and lightweight compositor manager for X11",
			StargazersCount: 35,
			ForksCount:      1,
			Language:        "C",
			URL:             "https://github.com/tycho-kirchner/fastcompmgr",
		},
	}
}
