# TB

A CLI to use toolbox with some small quality of life features, very opinionated to my own taste.

Invoking `tb $cmd` will look for an exectuable script in that folder with the following name: `tb-$cmd`, and will
invoke it forwarding all arguments.

if `tb-$cmd` is nof found, it will invoke `toolbox` forwarding all arguments.

## Conainer and "Boxes"

This tool has the concept of "boxes", that is, each container managed by toolbox is considered a "box", pretty obvious, right?

This means that:

* all containers created by *tb* will have `-box` name sufix;
* Containerfiles can be stored in a folder (details bellow) and running `tb build foo` will create a `foo-box` container;
* images built with tb will also be named as `localhost/$name-box:latest`.

### Configuration

Since this tool builds container images, it needs a "home folder" to store these file, such a folder should have the following structure:

```
$HOME/.config/tb
└── boxes
    ├── kdev
    │   └── Containerfile
    └── py3
        └── Container
```

The default folder for its base dir is `$HOME/.config/tb` which can be changed by setting a different path the following
environment variable: `TB_BASE_DIR`.

Box names are picked up by their dir names, so "kdev" and "py3" should be used as values for "BOX_NAME" in commands in this case.

## Commands

### build

Builds a box by its name (see the Configuration section).

The resulting image will be tagged as `localhost/$BOX_NAME-box:latest`.

Examples:

```
$ tb build py3
```

### rmi

A bit simimlar to the podman "rmi" sub command, it will remove a box's container image based on the box name.

Examples:

```
$ tb rmi py3
```

### exec

This command is similar to `toolbox run ...`, that is, it will run a command inside a container managed by toolbox:

```
tb exec [--build] [--keepme] BOX_NAME COMMAND [ARGS...]
```

* `--build`: optional flag argument that indicates a new image should be build for that box
* `--keepme`: optional flag argument to keep the box container alive after the command execution
* `BOX_NAME`: the box name to run
* `COMMAND` the command to run
* `ARGS`: optional COMMAND arguments

Examples:

```
$ tb exec dev make
$ tb exec dev gcc -o myprogram myprogram.c
$ tb exec --build py3dev tox
$ tb exec --keepme dev make
$ tb exec --build --keepme dev make
```

### rm

Similar to `podman rm...` but it will delete a running box based on its name:

```
$ tb rm py3
```

### stop

Similar to `podman stop...` but it will stop a running box based on its name:

```
$ tb stop py3
```

## License

[MIT](./LICENSE)
