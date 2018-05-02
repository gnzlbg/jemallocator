# Updating jemalloc

Updating the `jemalloc` version requires generating new `configure` files, which
requires `autoconf` to be installed.

To generate the configuration files, go to the `jemalloc` source directory and:

```shell
./autogen.sh
make distclean
```
