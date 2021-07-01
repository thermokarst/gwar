# gwar: git workspaces and repositories

this is not meant for general consumption!

i like to be able to quickly bootstrap my git projects when i move into a fresh
os install - having a single-file binary to make that happen seemed really
nice, plus, i was looking for an excuse to play with rust a bit. you have been
warned.

## example config

```toml
[[workspace]]
path = "$HOME/projects"
ssh_key_path = "$HOME/.ssh/id_rsa"
origin.base_addr = "ssh://git@pingo.thermokar.st"
origin.name = "pingo"
repos = [
  "gpx-web-utils",
  "gwar",
  "zettel",
]
remotes = [
  { name = "thermokarst", base_addr = "ssh://git@github.com/thermokarst" },
]
```

## building and testing locally

```bash
sudo port install libiconv
sudo mkdir /opt/local/include/iconv
sudo cp /opt/local/include/iconv.h /opt/local/include/iconv/
# building
CFLAGS=-I/opt/local/include/iconv cargo build
```
