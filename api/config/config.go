package config

import "github.com/spf13/viper"

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
