package config

import (
	"fmt"
	"log"
)

func GetUserChromes() []Userchrome {
	return ReadConfig().Userchromes
}

func AddUserChrome(c Userchrome) {
	config := ReadConfig()

	for _, ce := range config.Userchromes {
		if ce.Name == c.Name {
			log.Fatalf("Userchrome with name %s already exists!", c.Name)
		}
		if ce.CloneURL == c.CloneURL {
			log.Fatalf("Userchrome with clone URL %s already exists: %s", c.CloneURL, ce.Name)
		}
	}

	config.Userchromes = append(config.Userchromes, c)

	err := WriteConfig(config)
	if err != nil {
		log.Fatalln(err)
	}
}
