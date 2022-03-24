# aura 😇
#### Tool to search and/or install AUR packages from the terminal


## Syntax

### Search for packages
````
$ aura search cowsay-bin

Searching for "cowsay-bin"
Fetching from: https://aur.archlinux.org/rpc/?v=5&type=search&by=name&arg=cowsay-bin
Found 1 results

            cowsay-bin
            Description: Cowsay implemented using Rust.
            Maintainer: fffzlfk
            Votes: 0
            URL: https://github.com/fffzlfk/scowsay
            Download URL: https://aur.archlinux.org/cowsay-bin.git
````

### Install a package

```
$ aura install cowsay-bin

Installing "cowsay-bin"
Cloning into 'cowsay-bin'...wwwwwwwwwwwwwwwwwsww 
==> Making package: cowsay-bin 0.1.0-1 (Thu Mar 24 15:06:07 2022)
==> Checking runtime dependencies...
==> Checking buildtime dependencies...
==> Installing package cowsay-bin with pacman -U...
loading packages...
(2/2) Refreshing PackageKit...
exit status: 0

Trimmed the output in README  for cosmetic purposes :D

```

## How it works?
The aim is to keep aura as lightweight as possible therefore when it installs a package aura will:
1. Compose the AUR git url using the AUR git clone url format without doing an extra fetch on the API
2. Run git clone
3. Run makepkg -si
4. Remove the files from step 2 and 3 after the installation was finished

## Contribute / Roadmap
- [x] Search
- [x] Install
- [ ] Tests
- [ ] Treat the error cases properly
- [ ] Display a percentage instead of the long output
- [ ] Add limit param to the search function

#### NOTE: I've got a ' C# / JS background and  I am experimenting with rust as I want to fully transition to it in the future. Therefore, feel free to contribute even if you are a beginner, we can learn together.