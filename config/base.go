package config

import (
	"log"
	"os"
	"path"

	"github.com/pelletier/go-toml/v2"
	"github.com/ryanccn/nyoom/utils"
)

type UserchromeConfig struct {
	Key   string `toml:"key"`
	Value string `toml:"value"`
	Raw   bool   `toml:"raw"`
}

type Userchrome struct {
	Name     string             `toml:"name"`
	CloneURL string             `toml:"clone_url"`
	Configs  []UserchromeConfig `toml:"configs"`
}

type Config struct {
	Profile     string       `toml:"profile"`
	Userchromes []Userchrome `toml:"userchromes"`
}

func getConfigFile() string {
	dir, yop := os.LookupEnv("XDG_CONFIG_DIR")

	if !yop {
		home, err := os.UserHomeDir()
		if err != nil {
			log.Fatalln(err)
		}

		dir = path.Join(home, ".config")
	}

	path := path.Join(dir, "nyoom.toml")
	return path
}

func ReadConfig() Config {
	configPath := getConfigFile()

	if !utils.Exists(configPath) {
		return Config{}
	}

	f, err := os.ReadFile(configPath)
	if err != nil {
		log.Fatal(err)
	}

	var config Config
	err = toml.Unmarshal(f, &config)
	if err != nil {
		log.Fatal(err)
	}

	return config
}

func WriteConfig(config Config) error {
	configPath := getConfigFile()

	bs, err := toml.Marshal(config)
	if err != nil {
		return err
	}

	return os.WriteFile(configPath, bs, 0655)
}
