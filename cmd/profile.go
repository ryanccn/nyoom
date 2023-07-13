package cmd

import (
	"fmt"
	"log"
	"os"

	"github.com/spf13/cobra"
	"github.com/spf13/viper"
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

		if isSetting {
			viper.Set("profile", args[0])
			err := viper.WriteConfig()

			if err != nil {
				log.Fatalln(err)
			}
		} else {
			if viper.IsSet("profile") {
				fmt.Println(viper.GetString("profile"))
			} else {
				fmt.Println("Not configured")
			}
		}
	},
}

func init() {
	rootCmd.AddCommand(profileCmd)
}
