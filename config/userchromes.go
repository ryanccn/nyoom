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

func SetUserChromeConfig(name string, key string, value string, raw bool) error {
	config := ReadConfig()

	for idx, chrome := range config.Userchromes {
		if chrome.Name == name {
			alreadyHasKey := false

			for idx, config := range chrome.Configs {
				if config.Key == key {
					chrome.Configs[idx] = UserchromeConfig{Key: key, Value: value, Raw: raw}
					alreadyHasKey = true
					break
				}
			}

			if !alreadyHasKey {
				chrome.Configs = append(chrome.Configs, UserchromeConfig{Key: key, Value: value, Raw: raw})
				config.Userchromes[idx] = chrome
			}

			err := WriteConfig(config)
			return err
		}
	}

	return fmt.Errorf("no userchrome with name %s found", name)
}

func UnsetUserChromeConfig(name string, key string) error {
	config := ReadConfig()

	for uIdx, chrome := range config.Userchromes {
		if chrome.Name == name {
			found := false

			for cIdx, config := range chrome.Configs {
				if config.Key == key {
					chrome.Configs[cIdx] = chrome.Configs[len(chrome.Configs)-1]
					chrome.Configs = chrome.Configs[:len(chrome.Configs)-1]
					found = true
					break
				}
			}

			if !found {
				return fmt.Errorf("config key %s not found", key)
			}

			config.Userchromes[uIdx] = chrome

			err := WriteConfig(config)
			return err
		}
	}

	return fmt.Errorf("no userchrome with name %s found", name)
}
