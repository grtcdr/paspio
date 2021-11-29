# paspio

[![build](https://img.shields.io/github/workflow/status/grtcdr/paspio/paspio)](https://github.com/grtcdr/paspio/actions)
[![docs](https://img.shields.io/docsrs/paspio)](https://docs.rs/paspio/latest/paspio)
[![version](https://img.shields.io/crates/v/paspio)](https://crates.io/crates/paspio)

__paspio__, short for _pasvorta entropio_, is a password entropy calculator/library.

Please refrain from using this as a **sole** measure of password strength, it **should** be
used in conjuction with other security tools, e.g. `cracklib-check`.

## Usage

If you're intending on using __paspio__ as a program, run the following in your terminal:

```bash
# Pass one or more passwords to paspio to get their entropy
paspio "liuLe9ohjub8hu2ie"
```

If you're wishing to use it in your own project(s), i.e. as a library, then add the following to your project's
`Cargo.toml`:

```
paspio = "0.2"
```

## Installation

#### [Ca](https://crates.io/crates/paspio/)rgo

```
cargo install paspio
```

#### [Ar](https://aur.archlinux.org/packages/paspio-git/)ch User Repository

```
git clone https://aur.archlinux.org/paspio-git.git
cd paspio-git
makepkg -si
```

#### Building from source

```bash
git clone https://github.com/grtcdr/paspio
cd paspio
cargo build --release
```

### Prebuilt binaries

Prebuilt binaries for _Windows_, _Linux_ and _macOS_ are available in the [releases page](https://github.com/grtcdr/paspio/releases/).

## FAQ

### What is this tool useful for?

Ever wanted to verify the strength of your passwords, but felt slightly
uncomfortable doing so over the internet?

If you answered __yes__, then you're looking at the right tool. After all, it's the
only reason I created it; to check how random my passwrods are, but
_locally_.

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
