package cmd

import (
	"fmt"
	"log"

	"github.com/ryanccn/nyoom/config"
	"github.com/ryanccn/nyoom/lib"
	"github.com/ryanccn/nyoom/presets"
	"github.com/spf13/cobra"
)

var userchromeListCmd = &cobra.Command{
	Use:   "list",
	Short: "List userchromes",
	Run: func(cmd *cobra.Command, args []string) {
		data := config.GetUserChromes()

		if len(data) == 0 {
			fmt.Println("No userchromes available! Use `nyoom add` to add one")
		} else {
			for _, chrome := range data {
				fmt.Printf("%s - %s\n", chrome.Name, chrome.CloneURL)
			}
		}
	},
}

var userchromeAddCmd = &cobra.Command{
	Use:   "add <name> <Git clone URL>",
	Short: "Add a new userchrome",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		name, cloneURL := args[0], args[1]

		config.AddUserChrome(config.Userchrome{Name: name, CloneURL: cloneURL})
	},
}

var userchromePresetCmd = &cobra.Command{
	Use:   "preset <name>",
	Short: "Add a new userchrome from a preset",
	Args:  cobra.MaximumNArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		if len(args) == 0 {
			for _, c := range presets.ListPresets() {
				fmt.Printf("- %s\n", c)
			}
			return
		}

		presetName := args[0]
		config.AddUserChrome(presets.GetPreset(presetName))

		fmt.Printf("Added userchrome %s from preset!\n", presetName)
		fmt.Printf("Run `nyoom switch %s` to switch to this userchrome.\n", presetName)
	},
}

var userchromeSwitchCmd = &cobra.Command{
	Use:   "switch <name>",
	Short: "Switch to a userchrome",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		cfg := config.ReadConfig()

		for _, chrome := range cfg.Userchromes {
			if chrome.Name == args[0] {
				profile := cfg.Profile
				lib.Switch(chrome, profile)
				return
			}
		}

		log.Fatalf("No userchrome with name %s found\n", args[0])
	},
}

func init() {
	rootCmd.AddCommand(userchromeListCmd)
	rootCmd.AddCommand(userchromeAddCmd)
	rootCmd.AddCommand(userchromePresetCmd)
	rootCmd.AddCommand(userchromeSwitchCmd)
}
