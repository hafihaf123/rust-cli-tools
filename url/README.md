# Url

- Simple program for basic url encoding and decoding

## Installation

```cargo install url-cli-tool```
- Running the above command will globally install the *url-cli-tool* binary.

## Usage

```url [OPTIONS] [FILE]...```

- you can also pipe the output of an another command into url to encrypt (decrypt) its output

```somecommand | url (-d)```

### Arguments

```
  [FILE]...  File to url-encode. The output will be saved to _<filename>\_encoded.txt_ or _<filename>\_decoded.txt_
```

### Options

```
  -d, --decode           Decode mode
  -s, --string <STRING>  Url-encode a string instead of a file (cannot be used together with the [FILE] argument). Output will be written to stdout
  -h, --help             Print help
  -V, --version          Print version
```

---

url 1.1.0
