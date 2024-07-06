package config

import (
	"github.com/spf13/viper"
)

// Config struct
type Config struct {
	Server struct {
		Environment string `mapstructure:"environment"`
		Port        string `mapstructure:"port"`
		Mode        string `mapstructure:"mode"`
	} `mapstructure:"server"`
	Github struct {
		Username string `mapstructure:"username"`
		Token    string `mapstructure:"token"`
	} `mapstructure:"github"`
	Cache struct {
		TTL string `mapstructure:"ttl"`
	} `mapstructure:"cache"`
}

type Server struct {
	Environment string `mapstructure:"environment"`
	Port        string `mapstructure:"port"`
	Mode        string `mapstructure:"mode"`
}

type Github struct {
	Username string `mapstructure:"username"`
	Token    string `mapstructure:"token"`
}

type Cache struct {
	TTL string `mapstructure:"ttl"`
}

// Get config
func Get() *Config {
	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.AddConfigPath("./config")
	viper.AutomaticEnv()

	var cfg Config
	if err := viper.ReadInConfig(); err != nil {
		panic(err)
	}

	cfg = Config{
		Server: Server{
			Environment: viper.GetString("ENVIRONMENT"),
			Port:        viper.GetString("PORT"),
			Mode:        viper.GetString("MODE"),
		},
		Github: Github{
			Username: viper.GetString("GITHUB_USERNAME"),
			Token:    viper.GetString("GITHUB_TOKEN"),
		},
		Cache: Cache{
			TTL: viper.GetString("CACHE_TTL"),
		},
	}
	// err := viper.Unmarshal(&cfg)
	// if err != nil {
	// 	panic(err)
	// }

	return &cfg
}
