# gwar: git workspaces and repositories

this is not meant for general consumption!

i like to be able to quickly bootstrap my git projects when i move into a fresh
os install - having a single-file binary to make that happen seemed really
nice, plus, i was looking for an excuse to play with rust a bit. you have been
warned.

## building and testing locally

```bash
sudo port install libiconv
sudo mkdir /opt/local/include/iconv
sudo cp /opt/local/include/iconv.h /opt/local/include/iconv/
# building
CFLAGS=-I/opt/local/include/iconv cargo build
```
