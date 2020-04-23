
# Runfile
an cli automation focused programming too, inspired from make.


## comments

```python
# a comment
```


## commands
```python
>> echo hello
```


## a command block

```python
# this will use new lines following the pattern <whitespace><'>>'>
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