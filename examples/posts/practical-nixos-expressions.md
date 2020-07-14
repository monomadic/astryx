---
title: "Practical NixOS Expressions"
route: "/posts/practical-nixos-expressions"
description: "How to apply expressions in daily use."
category: "NixOS"
tags:
  - "nixos"
  - "deployment"
  - "expressions"
---

There sure are a lot of articles out there on how difficult it is to get started with NixOS. As a developer, I started with nix (the language) and worked up from there, so it made things far more intuitive for me. But several very simple use cases definitely nagged at me as seeming undocumented.

First things first, some definitions used in NixOS.

-

One thing I quickly learned about binaries on NixOS, and by extension, every operating system is how dependent they actually are. Usually one wouldn't notice that the binary is linking into system libraries, as they're always seemingly present in standard locations. NixOS doesn't let you have binaries in any old place, as they are strictly versioned and jailed.

You can view the binary links of an executable a few ways, for example, take a standard ubuntu install using `file`:

```bash
$ file /bin/bash

/bin/bash: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=12f73d7a8e226c663034529c8dd20efec22dde54, strippedfile /bin/bash
```

You can see it is loading a system wide linked at `/lib64/ld-linux-x86-64.so.2`. Great, but on NixOS, we don't have that. This is why regular NixOS programs don't run on linux, they need to be patched with `patchelf`.

If you're still with me, good because now the actual tricky stuff starts.



## Conclusion

Congratulations, `zrs` is now a functional part of your NixOS installation! It is not in the `nixpkgs` official repository of course, and if you create packages you should probably contribute those. But there are cases where you don't really want to do that, now you can.

As a side bonus for getting this far, `zrs` is actually a pretty cool app, and better than other jump apps as far as I can tell. I recommend aliasing to z with `alias z='zrs'` in your `.bashrc`.

``` bash
alias z='zrs'

# add a directory
z --add $PWD
z --add ~/.config/nixpkgs

# list the directories you've added
z --list

# open a folder (or file) in another app
vim `z nixpkgs`
```
