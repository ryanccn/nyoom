package config

import (
	"log"

	"github.com/spf13/viper"
)

type Userchrome struct {
	Name     string `mapstructure:"name"`
	CloneURL string `mapstructure:"clone_url"`
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
	data = append(data, c)

	viper.Set("userchromes", data)

	err := viper.WriteConfig()
	if err != nil {
		log.Fatalln(err)
	}
}
