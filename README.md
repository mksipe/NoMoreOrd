# NoMoreOrd

### NoMoreOrd is a small program that looks up the reverse plaintext character from Unicode. This program had inspired by python ord() functions in computational competitions. The whole point of this program is to speed things up instead of manually typing in a python prompt to find the character 65 in a python shell:
---
```python
>>> ord("4")
52
>>> ord("f")
102
>>> ord("W")
87
>>> ord("2")
50
>>> ord("b")
98
>>> ord("A")
65
```

As shown above this is the usual process for solving an issue of finding a specific character because unicode is a one way function. 

---

while this program automates this program by using the command prompt to help find these characters much easier. 

# Usage

## Single character

```sh
cargo run -- -c '65'
'A'
```

> The -c flag tests only a single binary character.


##  Multiple characters

> text.txt 
```txt
0073
32
0097
0109
32
0097
32
0115
0104
0114
0105
0109
0112
```


```sh
user@localhost~ cargo run -- -f test.txt
line 0  -> (    'I'     )
line 1  -> (    ' '     )
line 2  -> (    'a'     )
line 3  -> (    'm'     )
line 4  -> (    ' '     )
line 5  -> (    'a'     )
line 6  -> (    ' '     )
line 7  -> (    's'     )
line 8  -> (    'h'     )
line 9  -> (    'r'     )
line 10 -> (    'i'     )
line 11 -> (    'm'     )
line 12 -> (    'p'     )
```

---

# Help Page 

```yaml
NoMoreOrd 1.0
Mason Sipe
Converts unicode back to plain ASCII text.

USAGE:
    ord.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --ciphertext <STDIN>    Translates one single value.
    -f, --input <FILEIN>        Translates a file from its newline input.
```