package cmd

import (
	"log"

	"github.com/ryanccn/nyoom/config"
	"github.com/spf13/cobra"
)

var configCmd = &cobra.Command{
	Use:   "config",
	Short: "Manage userchrome-linked configs",
}

var configAddRaw bool

var configAddCmd = &cobra.Command{
	Use:   "set <userchrome> <key> <value>",
	Short: "Sets a Firefox config on a userchrome",
	Args:  cobra.ExactArgs(3),
	Run: func(cmd *cobra.Command, args []string) {
		userchrome, key, value := args[0], args[1], args[2]
		err := config.SetUserChromeConfig(userchrome, key, value, configAddRaw)

		if err != nil {
			log.Fatal(err)
		}
	},
}

var configUnsetCommand = &cobra.Command{
	Use:   "unset <userchrome> <key>",
	Short: "Unsets a Firefox config on a userchrome",
	Args:  cobra.ExactArgs(2),
	Run: func(cmd *cobra.Command, args []string) {
		userchrome, key := args[0], args[1]
		err := config.UnsetUserChromeConfig(userchrome, key)

		if err != nil {
			log.Fatal(err)
		}
	},
}

func init() {
	configAddCmd.Flags().BoolVarP(&configAddRaw, "raw", "r", false, "Whether the provided is a raw JavaScript value (true) or a string (false)")
	configCmd.AddCommand(configAddCmd)
	configCmd.AddCommand(configUnsetCommand)

	rootCmd.AddCommand(configCmd)
}
