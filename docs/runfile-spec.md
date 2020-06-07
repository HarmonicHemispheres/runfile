
# Runfile
an cli automation focused programming too, inspired from make.


## comments

```python
# a comment
```


## scripts
```python
>> echo hello
```


## a command block

```python
# this will use new lines following the pattern <'>>'>
# to indicate new commands
!cmd { 
    >> echo hello 
    >> echo hello again
}


# name a command to be reused
!cmd Installer {
    >> echo installing
    >> echo downloading
    >> echo done
}
```

## use a defined command block

```python
!call Installer
```

## define a command block with arguments
```python
!cmd Installer {
    >> echo {0}
    >> echo {1}
    >> echo {2}
}
```

## use a defined command block with arguments
```python
!call Installer (1, 'name', '360-909-6589')
```

## use the options macro to adjust runfile settings
```python
!options {
    "allow_aliases": [
        ">> __",
        "$__"
    ]
}
```

<br>

# IDEAS


```python
!check py = my_custom_checker.py -a input1 
!check check = my_custom_checker.py -a {1}

!var file = new_file.txt

!cmd build {
    [l]>> touch new.txt $file
}


!cmd compile [build create build-docs] 
{
    (win)>> touch new_file.txt
    (lnx)>>
    (mac)>>
}
```


## Arguments
building cli tools is very easy with runfile using the `!arg` keyword.

```rust
!arg <ident> [<attrs>]
```

this will allow the user to write out simple variables that will
automatically get parsed out by runfile and be usable as a value
inside the runfile scripts. attributes are available to help define
behavior of arguments.

```python
!arg debug [
    optional:<true or false>,   # defaults to true
    short_flag:<single letter>,
    long_flag:<word>
    ]
```

**example**
```rust
!arg debug [type:flag, short_flag:d]
!arg username [
    type:store,
    optional:false, 
    long_flag:user
    ]
!arg password [
    type:store,
    optional:false, 
    long_flag:pass
    ]
```
```shell
$ run -d --user myusername --pass asdfhjk234 
```