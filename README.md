# Locksidian
![](http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-2.png)
[![build status](https://gitlab.com/locksidian/locksidian/badges/master/build.svg)](https://gitlab.com/locksidian/locksidian/pipelines)

> The one vault your data really need.

Pure [Rust](https://www.rust-lang.org/) implementation of the
[blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology.

Full project documentation can be found here : https://locksidian.gitlab.io/locksidian/locksidian

## Installation

```bash
$ docker login registry.gitlab.com
$ docker pull registry.gitlab.com/locksidian/locksidian:master
$ docker run --name locksidian -d registry.gitlab.com/locksidian/locksidian:master
```

## Documentation

The project's documentation is auto-generated after each push on the `master` branch and is immediately published on
the [Locksidian GitLab Page](https://locksidian.gitlab.io/locksidian/locksidian).

## Contributing

### Project setup

In order to contribute to the project, you first have to follow these brief setup instructions:

 - Use the `nightly` rust toolchain : `rustup default nightly`.
 - If you are running Windows, the `gcc-rs` dependency requires that you set the `CC=gcc` environment variable in
    order to work properly.
 - If you are running an older 32 bits version of MinGW as your C toolchain, you will have to use the **i686** rust
   toolchain : `rustup default stable-i686-pc-windows-gnu`. You might want to upgrade to `MinGW-w64` using [Win-builds](http://win-builds.org)
   to continue using the 64 bits rust distribution. Don't forget to update your `default-host` in order to use the GNU
   toolchain: `rustup set default-host x86_64-pc-windows-gnu`.
 - Install `lisqslite3` (Windows: https://www.sqlite.org/download.html , Debian package: `apt-get install libsqlite3-dev`).
   Note: if the `-lsqlite3` flag is not recognized during compile time, try to copy all the `libsqlite.*` files into the
   `lib` folder of your current Rust toolchain:
   `<home>/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib`
   (this problem was only encountered on the Windows platform).
                                                                              
### Commit guidelines

When contributing to the `Locksidian` project, chances are that you will develop a feature that is requested by a specific
**issue**.

In your commit message, please always reference the issue using the following format:

```
Issue #N

 - Some changes
 - Some other interesting facts
 - Whatevs
```

If your commit closed or fixed the corresponding issue, you can use the following titles: `Close #N` or `Fixed #N`.
This way, the issue will be closed automatically when your commit reaches the `master` branch. 

If you are making modifications that are not linked to any open issue (please, don't do this), you *could* use the feature
branch name as the commit title, making it easy to locate the commit origin in the future.

## Contributors

 - [Valentin Fries](https://www.fries.io) ([GitHub](https://github.com/MrKloan))
 - Aurélien Duval ([GitHub](https://github.com/acid-killa666))