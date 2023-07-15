package config

import (
	"log"

	"github.com/spf13/viper"
)

type Userchrome struct {
	Name     string           `mapstructure:"name" toml:"name"`
	CloneURL string           `mapstructure:"clone_url" toml:"clone_url"`
	Config   UserchromeConfig `mapstructure:"config" toml:"config"`
}

func GetUserChromes() []Userchrome {
	var data []Userchrome

	if !viper.IsSet("userchromes") {
		return []Userchrome{}
	}

	err := viper.UnmarshalKey("userchromes", &data)
	if err != nil {
		log.Fatalf("Unable to decode into struct, %v", err)
	}

	return data
}

func AddUserChrome(c Userchrome) {
	data := GetUserChromes()

	for _, ce := range data {
		if ce.Name == c.Name {
			log.Fatalf("Userchrome with name %s already exists!", c.Name)
		}
		if ce.CloneURL == c.CloneURL {
			log.Fatalf("Userchrome with clone URL %s already exists: %s", c.CloneURL, ce.Name)
		}
	}

	data = append(data, c)

	viper.Set("userchromes", data)

	err := viper.WriteConfig()
	if err != nil {
		log.Fatalln(err)
	}
}
