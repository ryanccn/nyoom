package lib

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path"

	"github.com/ryanccn/nyoom/config"
	"github.com/ryanccn/nyoom/utils"
)

func Switch(chrome config.Userchrome, profile string) {
	tempDir, err := os.MkdirTemp(os.TempDir(), "nyoom")
	if err != nil {
		log.Fatalln(err)
	}

	fmt.Println("Cloning repository")
	cloneCmd := exec.Command("git", "clone", "--depth=1", chrome.CloneURL, tempDir)
	err = cloneCmd.Run()
	if err != nil {
		log.Fatalln(err)
	}

	fmt.Println("Copying chrome directory")
	newChromeDir := path.Join(profile, "chrome")

	if utils.Exists(newChromeDir) {
		os.RemoveAll(newChromeDir)
	}
	os.Mkdir(newChromeDir, 0755)

	err = utils.CopyDirectory(path.Join(tempDir, "chrome"), newChromeDir)
	if err != nil {
		log.Fatalln(err)
	}

	fmt.Printf("Switched to %s!\n", chrome.Name)
}