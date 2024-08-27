# Url

- Simple program for basic url encoding and decoding

## Installation

```cargo install url-cli-tool```
- Running the above command will globally install the *url-cli-tool* binary.

## Usage

```url [OPTIONS] [FILE]...```

- you can also pipe the output of another command into url to encrypt (decrypt) its output

```somecommand | url (-d)```

### Arguments

```
    [FILE]...  File to url-encode
```

### Options

```
    -d, --decode           Decode mode
    -o, --output <OUTPUT>  Save the output to a file
    -s, --string <STRING>  Url-encode a string instead of a file (cannot be used together with the [FILE] argument)
    -h, --help             Print help
    -V, --version          Print version
```

---

url 1.1.2
