package cmd

import (
	"fmt"
	"log"
	"os"

	"github.com/ryanccn/nyoom/config"
	"github.com/spf13/cobra"
)

var profileCmd = &cobra.Command{
	Use:   "profile [dir]",
	Short: "Configure the active Firefox profile or get current profile directory",
	Args: func(cmd *cobra.Command, args []string) error {
		if err := cobra.MaximumNArgs(1)(cmd, args); err != nil {
			return err
		}

		if len(args) == 0 {
			return nil
		}

		stat, err := os.Stat(args[0])

		if os.IsNotExist(err) {
			return err
		}
		if !stat.IsDir() {
			return fmt.Errorf("%s is not a directory", args[0])
		}

		return nil
	},
	Run: func(cmd *cobra.Command, args []string) {
		isSetting := len(args) > 0
		cfg := config.ReadConfig()

		if isSetting {
			cfg.Profile = args[0]
			err := config.WriteConfig(cfg)
			if err != nil {
				log.Fatal(err)
			}
		} else {
			if len(cfg.Profile) > 0 {
				fmt.Println(cfg.Profile)
			} else {
				fmt.Println("Not configured")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(profileCmd)
}
