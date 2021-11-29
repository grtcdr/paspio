# paspio

__paspio__, short for _pasvorta entropio_, is a password entropy calculator/library.

Refrain from using this as a sole measure of password strength, it **should** be
used in conjuction with other security tools, e.g. `cracklib-check`.

## Usage

If you're intending on using __paspio__ as a program, run the following in your terminal:

```bash
# Pass one or more passwords to paspio to get their entropy
paspio "liuLe9ohjub8hu2ie"
```

If you're wishing to use it in your own project(s), i.e. as a library, add the following to your project's
`Cargo.toml` and have a look at the [documentation](https://docs.rs/paspio/latest/paspio/) while you're at it.

```
paspio = "0.2"
```

## Installation

- [Cargo](https://crates.io/crates/paspio):

```
cargo install paspio
```

- [Arch User Repository](https://aur.archlinux.org/packages/paspio-git/):

```
git clone https://aur.archlinux.org/paspio-git.git
cd paspio-git
makepkg -si
```

- Building from source:

```bash
git clone https://github.com/grtcdr/paspio
cd paspio
cargo build --release
```

## FAQ

### What is entropy?

[Entropy](https://en.wikipedia.org/wiki/Password_strength#Entropy_as_a_measure_of_password_strength)
measures how unpredictable a password is.

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
