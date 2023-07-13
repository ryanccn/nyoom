package cmd

import (
	"fmt"
	"log"

	"github.com/ryanccn/nyoom/config"
	"github.com/ryanccn/nyoom/lib"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
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

var userchromeSwitchCmd = &cobra.Command{
	Use:   "switch <name>",
	Short: "Switch to a userchrome",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		userchromes := config.GetUserChromes()

		for _, chrome := range userchromes {
			if chrome.Name == args[0] {
				profile := viper.GetString("profile")
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
	rootCmd.AddCommand(userchromeSwitchCmd)
}
