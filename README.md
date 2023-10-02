# ltsh
small utility to perform a type-check on shell-pipelines
<hr/>

### Example

```sh
[~]$ ltsh <<< 'echo -n $PATH | xargs stat -c %x | sort -n'
```
```
--- BEGIN TYPE-ANALYSIS ---

* unknown stdin-type for `echo -n $PATH`

* !====> TYPE MISMATCH !! <====!
    ——————————
  ....`echo -n $PATH` outputs
<Seq Path~<Seq PathSegment~<Seq Char>>~<SepSeq Char '/'>~<Seq Char>>~<SepSeq Char ':'>~<Seq Char>
    ———————————
  .... `xargs stat -c %x` expects
<Seq Path~<Seq PathSegment~<Seq Char>>~<SepSeq Char '/'>~<Seq Char>>~<SepSeq Char '\n'>~<Seq Char>
    ——————————

* !====> TYPE MISMATCH !! <====!
    ——————————
  ....`xargs stat -c %x` outputs
<Seq Date~ISO-8601~<Seq Char>>~<SepSeq Char '\n'>~<Seq Char>
    ———————————
  .... `sort -n` expects
<Seq ℕ>~<Seq <PosInt 10 BigEndian>~<Seq <Digit 10>~Char>>~<SepSeq Char '\n'>~<Seq Char>
    ——————————

--- END TYPE-ANALYSIS ---
```


### Use as Zsh-extension
To automatically check every pipeline entered during interactive shell
use, add the following hook to your `.zshrc`:

```sh
preexec() {
    ltsh <<< "$1"
}
```

## Limitations

* only parses pipelines
* ignores quoting rules & expansions
* regex-based typedb implementation (slow & incapable)


### License
[GPLv3](COPYING)
