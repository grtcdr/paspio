# paspio — pasvorta entropio

A (naive) password entropy calculator.

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

It uses the same formula as [omnicalculator](https://www.omnicalculator.com/other/password-entropy).

```bash
Entropy = L * log2(R)
```

Where:
- L is the length of the password.
- R is the size of the pool of characters the password exists in.
