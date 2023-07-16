package presets

import (
	"embed"
	"log"
	"strings"

	"github.com/pelletier/go-toml/v2"
	"github.com/ryanccn/nyoom/config"
)

//go:embed *.toml
var presets embed.FS

func ListPresets() []string {
	list, err := presets.ReadDir(".")
	if err != nil {
		log.Fatal(err)
	}

	ret := []string{}
	for _, f := range list {
		if f.Type().IsRegular() && strings.HasSuffix(f.Name(), ".toml") {
			fName := f.Name()
			ret = append(ret, fName[:len(fName)-5])
		}
	}

	return ret
}

func GetPreset(name string) config.Userchrome {
	bytes, err := presets.ReadFile(name + ".toml")
	if err != nil {
		log.Fatal(err)
	}

	var data config.Userchrome
	err = toml.Unmarshal(bytes, &data)
	if err != nil {
		log.Fatal(err)
	}

	return data
}
