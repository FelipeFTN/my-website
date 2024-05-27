package infra

import "github.com/FelipeFTN/my-website/config"

type Infra struct {
	Cache *Cache
}

func New(cfg *config.Config) *Infra {
	var infra Infra

	infra.Cache = NewCache(cfg)

	return &infra
}
