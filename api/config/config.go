package config

import "github.com/spf13/viper"

// Config struct
type Config struct {
	Server struct {
		Port string `mapstructure:"port"`
		Mode string `mapstructure:"mode"`
	} `mapstructure:"server"`
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

	err := viper.Unmarshal(&cfg)
	if err != nil {
		panic(err)
	}

	return &cfg
}
