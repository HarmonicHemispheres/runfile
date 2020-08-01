# Language Aspects

# Language Architecture

Runfile ultimately is a linear program following the sequence of steps.

1. CLIENT
    1. locate available `runfile`
    2. lex characters into run tokens
    3. parse tokens into commands
2. ENGINE
    1. lookup and validate all commands in list
    2. 
    3. 
    4. execute each command in the list

# Language Spec

```
<statement>    =    <action> <action args>


# ACTIONS
<CMD>          =    CMD <ident> <conditions><attrs><scripts>
<RUN>          =    RUN <conditions><attrs><scripts><nl> 
<RUN short>    =    <conditions><attrs> >> <scripts><nl>


# STATEMENT ELEMENTS
<attrs>        =    [<value><nl><...>]
<conditions>   =    (<value><nl><...>)
<scripts>      =    {<script><nl><...>}
<nl>           =    new line
```


## Action Keywords

- `CMD` - create a command which can be run by a name
- `RUN` - specifies a script to run
- `VAR` - define a variable
- `CONFIG` - 
- `PROMPT`
- `IMPORT` - imports another runfile as an extension


## Sandbox
```
# format 1
<action> <script>
<action> <attributes + flags> >> <script>
<action> <attributes + flags> >> { <command list> }
```

```bash
RUN
?:   win,linux
+:   shell=csh
>>   echo "hello world" 


RUN ?:win,wsl  >> echo help
RUN ?:win  +:shell=csh  >> echo help
RUN [win shell=csh] >> echo help
RUN +[win,wsl] >> echo hlep

CMD 
  id:build 
plat:win,mac
 arg:[name=demo val=1234]
 arg:[name=user val=$prompt msg='choose a user name']
>> echo ${$user}


RUN  plat:mac,linux  >> df -h

CMD id:testing seq:[
    RUN plat:win       >> tester.py --run_win
    RUN plat:linux,mac >> tester.py --run_unix
]

CMD id:testing actions:{
    RUN plat:win       >> tester.py --run_win
    RUN plat:linux,mac >> tester.py --run_unix
}

CMD id:download main:[
    RUN curl -o filename.tar.gz http://filename-4.0.1.tar.gz
]
DOWNLOAD url:http://filename-4.0.1.tar.gz


VAR  id:date  val:'12-13-2020' 
before:[
    RUN plat:win >> echo setting up var 
]
after:[
    RUN win       >> echo setup complete
    RUN mac linux >> echo setup complete
]


IF cond1:'date exists' run1:[
    RUN echo running 1
]  cond2:'date exists' run2:[
    RUN echo running 2
]
```

```json
{
    "settings": {},
    "pipeline": [
        {
            "command": "RUN",
            "plat": ["win", "wsl"],
            ">>": "echo setup complete"
        }
    ]
}
```

```
attribute            <name>:<val>
flag                 <name>
list of commands     <attr>:[<command>...]
multiple key val     <attr>:{<key>=<val>...}
```

<br>
<br>

# Action Format

```
<action>
    [flags]
    [key vals]
    [directives]
    <script>
    default_return / repr       # the value used in formatting

<flag>    =   <ident>
```

<br>
<br>

# Command Ideas


### Unexplored Actions
```bash
PROMPT
COPY
DOWNLOAD
CONFIG
    script.lang:'python'    # sets the language to run scripts with
                            # defaults to powershell or bash
CD to:'../'
USE
MOVE
CWD set:'<path>'
TEMPLATE
GLOBALS
ENV
LIST
FIND        # looks for file in path  
    query:''

    # where to look for the query
    where:<path>
PARALLEL actions:[<actions...>]
ADD_TO_PATH    p:<path>
DEL_FROM_PATH  p:<path>
SCRIPT
    id:numbers
    run_with:'python <code_file>'
    capture:true
    allow_var_fmt:true
    code:[
        t = 1+1
        print(t + ${someother})
    ]
PIPELINE
EVENT
UI.BUTTON
UI.GRAPH
WATCH
```
<Br>

### Action Definitions
```bash
# run a command
RUN
    id:'just a tag'
    <flags>
        capture
    >> <shell script...>

# store a variable
VAR
    id:name
    v:<value>
    (?)from_url:<url>   # converts response data into value
    (?)from_file:<path> # reads file into string val
    (?)from_csv:<path>  # opens csv into dataframe object

# command line arguments
ARG  
    id:arg1  
    type:[flag | option | value]
    help:'a description'

# will run a pipeline of actions if called from the cli
# i.e.  `$ run build`
CMD
    id:'build'
    help:'a description'
    setup:[<actions>...]    # a pipeline to run during setup
    args:[<arg...>]
    pipeline:[<actions...>]

# import a package and its actions
USE
    pkg:git
    (?)channel:msky # where to pull the package from
    login.site: ''
    login.user: ''
    login.pass: ''
    <flags>
        # only load package when called for
        lazy

# run preconfigured TEST suite 
TEST  opts:<options...>

# run preconfigured BUILD action
BUILD  opts:<options>
```

<br>

### Directives & Globals

```bash

# will only run action on certain systems
--[win,mac,linux]

# directives can be applied to ALL actions
--log

# directive with args
--log(to:'logs/things-1.log')

# this action REQUIRES certain resources
--reqs(files:'f1.txt setup.py', condition:'EXISTS')

# let this action be run
--trigger(by:<event action id>)
```

<br>
<hr>

<br>

### Exploring

```bash

# --- --- Meridian Sky --- ---
USE pkg:msky 

MSKY.CONNECT    # establish connection to msky
MSKY.CREATE     # starts a new project 
MSKY.SYNC       # sends project logs to account 

# --- --- GIT --- ---
USE pkg:git from:local 

GIT.ADD
GIT.COMMIT
GIT.PUSH
GIT.PULL

# runs pull then add, commit and push 
GIT.SYNC


# --- --- AZURE --- ---
USE pkg:azure

AZURE.LOGIN
AZURE.STATUS


# --- --- AWS --- ---
USE pkg:aws

AWS.S3.PUSH
AWS.S3.PULL


# --- --- GITLAB --- ---
USE pkg:gitlab


# --- --- GITHUB --- ---
USE pkg:github


# --- --- ANALYTICS --- ---
USE  pkg:analytics  as:a

A.TRAIN
A.PREDICT
A.LOAD
A.SORT

```


### Project structure
`runfile` projects can be made from any folder by creating the `.run/` folder.

```
.run/
    config.yaml
    msky.yaml
    extensions/     # where extensions get installed to
        e_git.yaml
        e_azure.yaml
```


<br>

# Language Spec 2.0

```
[ACTION] [flags] [attributes] >> [script][EOL]
[ACTION] { 
    [flags] 
    [attributes] 
    >> [script] 
    }
[ACTION]
    [flags] 
    [attributes] 
    >> [script]
    ;
[ACTION]! <input value> [EOL]
```
```dockerfile
RUN {win required age:3 script:'df -h'} 
RUN {win mac}[echo hello $RE.login()]
RUN {win mac}[
    echo hello $RE.login()
    ]
RUN!py echo hello


PIPELINE Install [
    RUN win       >> installer.exe --silent -g
    RUN linux mac >> ./installer --silent -L
]  

DOWNLOAD {
    win 
    from:'<url>' 
    to:'<path>'
}
DOWNLOAD mac from:'<url>' to:'<path>'
var data = from_url('mysite.com/data.zip')
VAR id:url1 v:'mysite.com/data.zip'
VAR id:data 
    from_url:$url1;

DOWNLOAD(mac from:'<url>' to:'<path>')
DOWNLOAD(mac from:'<url>' to:'<path>') >> <url> <path>


@f(required mac linux)
@a(shell:'powershell')
RUN df -h

@log
@plats=linux,mac
@shell=powershell
RUN df -h


RUN[mac linux id:tommy] df -h
RUN @(mac linux id:tommy) df -h

[PIPELINE]
@(mac linux id:tommy) 
RUN df -h
```

```dockerfile
var @(id:IDIR v:'../include')
var CC = gcc
var CFLAGS = -I${IDIR}

pipeline hellomake [

]

--------------------
IDIR =../include
CC=gcc
CFLAGS=-I$(IDIR)

ODIR=obj
LDIR =../lib

LIBS=-lm

_DEPS = hellomake.h
DEPS = $(patsubst %,$(IDIR)/%,$(_DEPS))

_OBJ = hellomake.o hellofunc.o 
OBJ = $(patsubst %,$(ODIR)/%,$(_OBJ))


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

hellomake: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS) $(LIBS)

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~ 
```




# more trash ideas
```
 DOWNLOAD(mac from:'<url>' to:'<path>') >> 'ls <path>'
# VAR(i:check from_run:'python check.py')

# RUN ls -l -a --log 
# RUN (mac linux shell:zsh) 
# >> ls -l -a --log 


[!CLI]
FORMAT()
ARG(name:silent ids:'-s, --silent' flag)
CMD(name:CLEAN
    ARG(name:user ids:'-u, --user' store)
    CMD(name:CLEAN
        ARG(name:user ids:'-u, --user' store)
    )
)
CMD(name:TEST
    ARG(name:verbose ids:-v,--verbose flag)
)

[CLEAN]
RUN (unix) >> rm -rf other.o file.o

[BUILD]
RUN (win req:file.o,other.o) >> gcc -lib prg.c prg

DOWNLOAD(mac from:'<url>' to:'<path>')
COPY(win wsl from:'' to:'')

DOWNLOAD (mac shell:'csh') >> https://goodle.co/data.js ./downloads
COPY (win wsl)             >> 'path a' 'path b'

[TEST]
RUN pytest ${TEST.verbose}
CASE(
    c1:$CLI.silent==true
    if1:[
        RUN python --silent -m pip install pandas
    ] 
    else:[
        RUN python -m pip install pandas
    ] 
)

#-----------------------------------------------
[!CODE_ACTIONS]
ACTION(
    name:DEBUG
    FIELD(name:)
    CODE(
        $parse(...)
    )
)


[RULES](
    parser:raw 
    runner:'python -c ${RULES}'
    )
t = 3
print(t)


[STATS](output)
```