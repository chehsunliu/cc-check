# cc-check

A small tool that checks a program's actual output against expected results.

## Usage

Use cargo to build this command line tool:

```sh
$ cargo build --release
$ ./target/release/cc-check --help
GG

USAGE:
    cc-check [OPTIONS] --executable <FILE> --input-folder <FOLDER> --output-folder <FOLDER>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -e, --executable <FILE>         The executable to run
    -i, --input-folder <FOLDER>     The folder containing testing files
    -o, --output-folder <FOLDER>    The folder containing expected result files
        --task-timeout <SECONDS>    The timeout in milliseconds. [default: 3000ms]
```

Suppose your program for an algorithm problem is `a.out`. Prepare the testing data in a folder:

```sh
$ ls ./input-data
01.txt 02.txt 03.txt
```

and the corresponding answers in the other folder with the **same** filenames:

```sh
$ ls ./expected-output-data
01.txt 02.txt 03.txt
```

Then run the binary as follows:

```sh
$ cc-check -e ./a.out -i ./input-data -o ./expected-output-data
```

## Screenshots

![example-1](/docs/example-1.png)

![example-2](/docs/example-2.png)
