# paspio — pasvorta entropio

A (naive) password entropy calculator. It is not intended or designed to be a
measure of password strength, please refrain from using it as such.

### Building the program

```bash
git clone https://github.com/grtcdr/paspio
cd paspio
cargo build
# You should move it to your PATH now...
```

### Usage

```
paspio "liuLe9ohjub8hu2ie"
```

### How does this calculator calculate entropy?

It uses the same formula as
[omnicalculator](https://www.omnicalculator.com/other/password-entropy).

```bash
Entropy = L * log2(R)
```

Where:
- `L` is the length of the password.
- `R` is the size of the pool of characters the password exists in.

### Pools? What pools?

| Pool                    | Elements | Pool size |
| -----:                  | :------  | :-------: |
| Lowercase latin letters | [a-z]    |    26     |
| Uppercase latin letters | [A-Z]    |    26     |
| Digits                  | [0-9]    |    10     |    
| Symbols                 | \`~!@#$%^&*()-=_+[{]}\|;':",.<>/? | 32 |
