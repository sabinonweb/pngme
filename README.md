
# pngme

#### pngme is a command line program that lets you hide secret messages in PNG files. It performs the given four commands:

    Encode a message into a PNG file
    Decode a message stored in a PNG file
    Remove a message from a PNG file
    Print a list of PNG chunks that can be searched for messages


## API Reference

 - [pngme book](https://jrdngr.github.io/pngme_book/)
 - [pngchat](https://docs.rs/pngchat/latest/pngchat/)



## Installation

#### Clap

```bash
$ cargo add clap --features derive
```
    
#### crc

```bash
$[dependencies]
crc = "3.0"
```