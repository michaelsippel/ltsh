# ltsh

**(highly experimental)**

tiny utility program for type-analysis of shell pipelines based on ladder-typing

<hr/>

### Example

```sh
[~]$ ltsh <<< 'echo -n $PATH | xargs stat -c %Y | sort -n'
```
```
--- BEGIN TYPE-ANALYSIS ---

* unknown stdin-type of `echo -n $PATH`

* typecheck error
          echo -n $PATH | xargs stat -c %Y
<Seq Path>              | <Seq Path>
<Seq <Seq PathSegment>> | <Seq <Seq PathSegment>>
<Seq <Seq <Seq Char>>>  | <Seq <Seq <Seq Char>>>
<Seq <SepSeq Char '/'>> | <Seq <SepSeq Char '/'>>
<Seq <Seq Char>>        | <Seq <Seq Char>>
<SepSeq Char ':'>       | <SepSeq Char '\n'>
<Seq Char>              | <Seq Char>

* typecheck ok
           xargs stat -c %Y | sort -n
<Seq Date>                  |
<Seq <TimeSince UnixEpoch>> |
<Seq <Duration Seconds>>    |
<Seq ℕ>                     | <Seq ℕ>
<Seq <PosInt 10 BigEndian>> | <Seq <PosInt 10 BigEndian>>
<Seq <Seq <Digit 10>>>      | <Seq <Seq <Digit 10>>>
<Seq <Seq Char>>            | <Seq <Seq Char>>
<SepSeq Char '\n'>          | <SepSeq Char '\n'>
<Seq Char>                  | <Seq Char>

--- END TYPE-ANALYSIS ---
```


### Install

```sh
git clone https://github.com/michaelsippel/ltsh
cd ltsh
cargo install --path .
```

### Use as Zsh-Extension
To automatically check every pipeline entered during interactive shell
use, add the following hook to your `.zshrc`:

```sh
preexec() {
    if ! ltsh <<< "${1}";
    then
        echo "\e[33;1m"
        echo "!! ltsh discoverd a type incompatibility.      !!"
        echo "!! abort [CTRL-C] or continue regardless [RET] !!"
        echo "\e[0m"
        read -s keys
    fi
}
```

## Limitations

* only parses pipelines
* ignores quoting rules & expansions
* regex-based typedb implementation (slow & incapable)


## License
[GPLv3](COPYING)
