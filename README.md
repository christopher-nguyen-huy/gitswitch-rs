# Gitswitch
- Switch ssh keys for multiple github accounts without needing [this](https://https://medium.com/@trionkidnapper/ssh-keys-with-multiple-github-accounts-c67db56f191e):
	- `.ssh/config` file
	- customizing the clone url for each new project
- Should work on
	- [x] Linux
	- [x] Windows
	- [x] Macos
- Rust multiplatform version of [gitswitch-py](https://gist.github.com/christopher-nguyen-huy/718f774e006bbc961f9401208d1afef5)

## What does it do?
- Symlinks the selected profile key to `id_rsa`
- Edits your git email and username

## Prerequisites
- Git installed
- An `.ssh/accounts` directory in the home directory
- Private keys and public keys inside directory matching names (ex: `key1` and `key1.pub`)
- An `~/.ssh/accounts.csv` file
	```
	key,name,email
	key1,name1,email1
	key2,name2,email2
	```

## Notes
#### Windows
Right click the executable, compatibility tab, set to run as administrator

## Todo
- [ ] Remove `.ssh/accounts` directory?
- [ ] Only symlink the private key?