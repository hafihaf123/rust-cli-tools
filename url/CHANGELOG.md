# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2024-08-27

### Change

- encoded and decoded filenames will have the format _\<filename\>.encoded_ or _\<filename\>.decoded_
- fixed some bugs

## [1.1.0] - 2024-08-07

### Added

- batch file encoding/decoding by specifying multiple files

### Changed

- when encoding/decoding a file, it will now print the result to a new _\<filename\>\_encoded.txt_ or _\<filename\>\_decoded.txt_ file
  - also works with batch encoding/decoding
  - for now, it won't strip the original file extension, it will just add the _.txt_ extension on top
- updated help message to correspond with new changes

## [1.0.0] - 2024-08-03

### Added

- main functionality - string encoding
- the decode flag
- file encoding, string encoding moved to an argument
- help and version messages
