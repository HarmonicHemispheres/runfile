
![](docs/banner-1.png)


`runfile` is a modern cli tool inspired by makefile and built in rust. `runfile` goes beyond the scope of build scripts and also aims to make automating cross platform helper, configuration and other scripts used in projects easier to write and easier to read. 


# Example

```bash
# supports 'running' script commands with the default shell
>> powershell echo hello world

# supports value variables
!var show = echo hi again

# supports platform specific scripts
[linux]>> ${show}
[mac  ]>> ${show}
[win  ]>> powershell ${show}

# supports multi platform scripts
[linux,mac]>> ${show}

# supports specific named commands available via cli
!cmd build {
    [mac  ]>> echo help
    [linux]>> echo help
    [win  ]>> powershell echo hiaaaa
}
```

<br>


# Install
To install `runfile`, simply download an appropriate executable for your platform (or build from source) and put the `runfile` program in your project directory.

Alternatively you could add the program to your path to be able to use it from anywhere.

- `v0.1.2`
    - linux - https://runfile.s3-us-west-2.amazonaws.com/runfile
    - windows - https://runfile.s3-us-west-2.amazonaws.com/runfile.exe

<br>

# Run

The `runfile` CLI tool has multiple options that can aid your scripting needs as seen below. 
```
USAGE:
    runfile [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug      Print extra content to the console
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cmd <cmd>             [default: __main__]
    -r, --runfile <runfile>    Specify a specific runfile to use [default: runfile]
```
