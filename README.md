# paspio — pasvorta entropio

A (naive) password entropy calculator. 

Refrain from using this as a sole measure of password strength, it **should** be
used in conjuction with other tools.

## Usage

paspio can act as both a _library_ and a _command-line program_.

If you're intending on using the program, run the following in your terminal:

```bash
# Pass one or more passwords to paspio to get their entropy
paspio "liuLe9ohjub8hu2ie"
```

If you're using it in your own program, add the following to your project's
Cargo.toml and have a look at the [documentation](https://docs.rs/paspio/0.2.0/paspio/) while you're at it.

```
paspio = "0.2.0"
```

## Installation

- [crates.io](https://crates.io/crates/paspio)

```
cargo install paspio
```

- [AUR](https://aur.archlinux.org/packages/paspio-git/)

```
git clone https://aur.archlinux.org/paspio-git.git
cd paspio-git
makepkg -si
```

- Building from source

```bash
git clone https://github.com/grtcdr/paspio
cd paspio
cargo build
# You should move it to your PATH now...
```

## FAQ

### How does this calculator calculate entropy?

```bash
Entropy = L * log2(R)
```

Where:
- `L` is the length of the password.
- `R` is the size of the pool of characters the password exists in.

### Pool? What pool?

| Pool                    | Elements | Pool size | Notes |
| -----:                  | :------  | :-------: | :---- |
| Lowercase latin letters | [a-z]    |    26     |       |
| Uppercase latin letters | [A-Z]    |    26     |       |
| Digits                  | [0-9]    |    10     |       |
| Symbols                 | \`~!@#$%^&*()-=_+[{]}\|;' :",.<>/? | 33 | Whitespace is included |
