## Url

- Simple program for basic url encoding and decoding

### Usage

```url [OPTIONS] [FILE]```

- you can also pipe the output of an another command into url to encrypt (decrypt) its output

```somecommand | url (-d)```

#### Arguments

  [FILE]  File to url-encode

#### Options
```
  -d, --decode           Decode mode
  -s, --string <STRING>  Url-encode a string instead of a file (cannot be used together with the [FILE] argument)
  -h, --help             Print help
  -V, --version          Print version
```

---

url 1.0.0
