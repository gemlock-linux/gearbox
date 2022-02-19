# Gearbox
A box full of utilities, it's written in rust and it's ment as a replacement for coreutils / busybox.

## List of content
* [How to setup](#how-to-setup)
    * Systems
        * [Ubuntu](#ubuntu)
        * [Arch](#arch)
    * [How to build](#how-to-build)
* [Modules](#modules)
* [License](#license)

## How to setup

To setup, it's quite simple. You install the rust toolchain from either your package manager or directly from the rustup website linked [here](https://www.rust-lang.org/learn/get-started)

* [Rust](https://www.rust-lang.org/learn/get-started)
* [glibc](https://www.gnu.org/software/libc/)

### Ubuntu
```
sudo apt install curl build-essential -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Arch
```
sudo pacman -Sy base-devel curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## How to build

```sh
# To build, simpy run
~$ cargo build

# To run, simply do
~$ cargo run

# You can use each module by using
~$ cargo run -- [MODULE_NAME] <MODULE_ARGS>
```

## Modules
Those are the checklist for the modules / commands that are supported.
Note that not everything is implemented just yet, we're working on it!

- [ ] `chcon`
- [ ] `chgrp`
- [ ] `chown`
- [ ] `chmod`
- [ ] `cp`
- [ ] `dd`
- [ ] `df`
- [ ] `dir`
- [ ] `dircolors`
- [ ] `install`
- [ ] `ln`
- [ ] `ls`
- [ ] `mkdir`
- [ ] `mkfifo`
- [ ] `mknod`
- [ ] `mktemp`
- [ ] `mv`
- [ ] `realpath`
- [ ] `rm`
- [ ] `rmdir`
- [ ] `shred`
- [ ] `sync`
- [x] `touch`
- [ ] `truncate`
- [ ] `vdir`
- [ ] `b2sum`
- [ ] `base32`
- [ ] `base64`
- [ ] `cat`
- [ ] `cksum`
- [ ] `comm`
- [ ] `csplit`
- [ ] `cut`
- [ ] `expand`
- [ ] `fmt`
- [ ] `fold`
- [ ] `head`
- [ ] `join`
- [ ] `md5sum`
- [ ] `nl`
- [ ] `numfmt`
- [ ] `od`
- [ ] `paste`
- [ ] `ptx`
- [ ] `pr`
- [ ] `sha1sum`
- [ ] `sha224sum`
- [ ] `sha256sum`
- [ ] `sha384sum`
- [ ] `sha512sum`
- [ ] `shuf`
- [ ] `sort`
- [ ] `split`
- [ ] `sum`
- [ ] `tac`
- [ ] `tail`
- [ ] `tr`
- [ ] `tsort`
- [ ] `unexpand`
- [ ] `uniq`
- [ ] `wc`
- [ ] `arch`
- [ ] `basename`
- [ ] `chroot`
- [ ] `date`
- [ ] `dirname`
- [ ] `du`
- [x] `echo`
- [ ] `env`
- [ ] `expr`
- [ ] `factor`
- [x] `false`
- [ ] `groups`
- [ ] `hostid`
- [ ] `id`
- [ ] `link`
- [ ] `logname`
- [ ] `nice`
- [ ] `nohup`
- [ ] `nproc`
- [ ] `pathchk`
- [ ] `pinky`
- [ ] `printenv`
- [ ] `printf`
- [x] `pwd`
- [ ] `readlink`
- [ ] `runcon`
- [ ] `seq`
- [ ] `sleep`
- [ ] `stat`
- [ ] `stdbuf`
- [ ] `stty`
- [ ] `tee`
- [ ] `test`
- [ ] `timeout`
- [x] `true`
- [ ] `tty`
- [x] `uname`
- [ ] `unlink`
- [ ] `uptime`
- [ ] `users`
- [ ] `who`
- [x] `whoami`
- [x] `yes`
- [ ] `[`

## License

Gearbox's code is licensed under the [MIT licence](https://opensource.org/licenses/MIT). Please see [the licence file](./LICENSE) for more information. [tl;dr](https://tldrlegal.com/license/mit-license) you can do whatever you want as long as you include the original copyright and license notice in any copy of the software/source.
