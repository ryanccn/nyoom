package lib

import (
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/ryanccn/nyoom/config"
	"github.com/ryanccn/nyoom/utils"
)

func getProfileUserJsPath(profile string) string {
	arkenfoxPath := filepath.Join(profile, "user-overrides.js")

	if utils.Exists(arkenfoxPath) {
		return arkenfoxPath
	}
	return filepath.Join(profile, "user.js")
}

const startLine = "/** nyoom-managed config; do not edit */"
const endLine = "/** end of nyoom-managed config */"

func applyUserJs(configs []config.UserchromeConfig, profile string) {
	path := getProfileUserJsPath(profile)

	bytes, err := os.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(bytes), "\n")

	startIdx, endIdx := -1, -1

	for idx, line := range lines {
		if line == startLine {
			startIdx = idx
		} else if line == endLine {
			endIdx = idx
		}
	}

	completeConfigs := append([]config.UserchromeConfig{{Key: "toolkit.legacyUserProfileCustomizations.stylesheets", Value: "true", Raw: true}}, configs...)

	addedLines := []string{}
	for _, config := range completeConfigs {
		actualValue := config.Value
		if !config.Raw {
			actualValue = "\"" + actualValue + "\""
		}

		addedLines = append(addedLines, "user_pref(\""+config.Key+"\", "+actualValue+");")
	}

	var newContent []string
	if startIdx != -1 && endIdx != -1 {
		newContent = append(append(lines[:startIdx+1], addedLines...), lines[endIdx:]...)
	} else {
		newContent = append(append(append(lines, startLine), addedLines...), endLine)
	}

	if newContent[len(newContent)-1] != "" {
		newContent = append(newContent, "")
	}

	os.WriteFile(path, []byte(strings.Join(newContent, "\n")), 0655)
}
