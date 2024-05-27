package infra

import (
	"time"

	"github.com/FelipeFTN/my-website/config"
)

type Cache struct {
	cfg   *config.Config
	TTL   time.Duration
	cache map[string]cacheData
}

type cacheData struct {
	Value string
	TTL   time.Time
}

func NewCache(cfg *config.Config) *Cache {
	cacheTTL, err := time.ParseDuration(cfg.Cache.TTL)
	if err != nil {
		panic(err)
	}

	return &Cache{
		cfg:   cfg,
		TTL:   cacheTTL,
		cache: make(map[string]cacheData),
	}
}

func (self *Cache) Get(key string) (string, bool) {
	data, ok := self.cache[key]
	if !ok {
		return "", false
	}

	if data.TTL.Before(time.Now()) {
		delete(self.cache, key)
		return "", false
	}

	return data.Value, true
}

func (self *Cache) Set(key, value string) {
	self.cache[key] = cacheData{
		Value: value,
		TTL:   time.Now().Add(self.TTL),
	}
}
