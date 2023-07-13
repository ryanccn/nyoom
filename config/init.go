package config

import (
	"log"
	"os"
	"path"

	"github.com/ryanccn/nyoom/utils"
	"github.com/spf13/viper"
)

func makeEmpty() {
	dir, yop := os.LookupEnv("XDG_CONFIG_DIR")

	if !yop {
		home, err := os.UserHomeDir()
		if err != nil {
			log.Fatalln(err)
		}

		dir = path.Join(home, ".config")
	}

	config := path.Join(dir, "nyoom.toml")

	if !utils.Exists(config) {
		os.WriteFile(config, []byte{}, 0655)
	}
}

func Init() {
	makeEmpty()

	viper.SetConfigName("nyoom")
	viper.SetConfigType("toml")
	viper.AddConfigPath("$XDG_CONFIG_DIR")
	viper.AddConfigPath("$HOME/.config")

	if err := viper.ReadInConfig(); err != nil {
		if _, ok := err.(viper.ConfigFileNotFoundError); !ok {
			log.Fatalln("Error when reading config", err)
		}
	}
}
