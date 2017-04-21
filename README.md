# Locksidian

[![build status](https://gitlab.com/locksidian/locksidian/badges/master/build.svg)](https://gitlab.com/locksidian/locksidian/pipelines)
[![](http://www.wtfpl.net/wp-content/uploads/2012/12/wtfpl-badge-2.png)](http://www.wtfpl.net/)

> The one vault your data really need.

Pure [Rust](https://www.rust-lang.org/) implementation of the
[blockchain](https://en.wikipedia.org/wiki/Blockchain_(database)) technology.

Full project documentation can be found here : http://locksidian.fries.io/

## Installation

### From sources

```bash
$ docker build -t locksidian:latest .
$ docker run --name locksidian -v .:/opt/locksidian -p 8080:8080 -d locksidian:latest
```
 
Or alternatively using Docker Compose:

```bash
$ docker-compose up -d
```

### From precompiled binaries

```bash
$ docker login registry.gitlab.com
$ docker pull registry.gitlab.com/locksidian/locksidian:master
$ docker run --name locksidian -v .:/opt/locksidian -p 8080:8080 -d registry.gitlab.com/locksidian/locksidian:master
```

## Documentation

The project's documentation is auto-generated after each push on the `master` branch and is immediately published on
the [Locksidian GitLab Page](https://locksidian.gitlab.io/locksidian/locksidian).

## Contributing

### Project setup: Windows

 - Install [msys2](http://www.msys2.org/);
 - **Optional**: Create a cross-integration between your `msys2` environment and your "native" Windows environment by
   defining the following Windows environment variables:
   
```bash
MSYS2_PATH_TYPE=inherit     # Your Windows PATH will be appended to your msys2 PATH
MSYS2_HOME=C:\msys64        # Replace with your custom install path

# Append to your Windows PATH in order to use msys2 executables from CMD/PowerShell:
# ;%MSYS2_HOME%\mingw64\bin;%MSYS2_HOME%\usr\bin
```

 - Install the `Rust` development environment and the `Locksidian` dependencies:

```bash
# Update distro
pacman -Syu

# Install Locksidian dependencies
pacman -Sy mingw-w64-x86_64-gcc \
           mingw-w64-x86_64-pkg-config \
           mingw-w64-x86_64-openssl \
           mingw-w64-x86_64-sqlite3 \
           curl
           
# Install Rust
curl -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain nightly --default-host x86_64-pc-windows-gnu -y
```

 - *Note*: if the `-lsqlite3` flag is not recognized at compile time, try to copy all the `<msys2>/mingw64/lib/libsqlite.*`
    files into the `lib` folder of your current Rust toolchain:
    `<home>/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/x86_64-pc-windows-gnu/lib`
                                                                              
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