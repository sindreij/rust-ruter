# Ruter API Command Line Client

Yet another Ruter API Client, using the reis api from Ruter labs.
This CLI is written in rust.

## Installating

No binaries are yet available, so you need to install from source code.

```bash
# Install rust, not needed if you already have rust installed. Note: You needed
# 1.15 because of serde, so this command will not work until 1.15 is released
# (2017-02-02)
curl https://sh.rustup.rs -sSf | sh
git clone https://github.com/sindreij/rust-ruter
cargo install
```

## Usage

Use with the stop name (or a part of it) to get all departures from that stop.
The tool will just use the first station ruter returns when searching for what
you input.

```bash
$ ruter stortinget
Avganger fra Stortinget [T-bane]
Retning øst
   2 Ellingsrudåsen             2m  0s 1
   4 Bergkrystallen             4m 13s 1
   3 Mortensrud                 6m 46s 1
   5 Vestli                     8m  1s 1
   1 Helsfyr                     21:04 1
   5 Ringen via Tøyen            21:05 1
   2 Ellingsrudåsen              21:07 1
   4 Bergkrystallen              21:11 1
   5 Vestli                      21:16 1
   3 Mortensrud                  21:17 1

Retning vest
   5 Sognsvann                  0m  0s 2
   1 Frognerseteren             1m 14s 2
   5 Ringen via Majorstuen      3m 33s 2
   3 Kolsås                     6m 52s 2
   4 Vestli via Majorstuen      9m 56s 2
   2 Østerås                     21:04 2
   1 Frognerseteren              21:08 2
   5 Sognsvann                   21:08 2
   5 Ringen via Majorstuen       21:11 2
   3 Kolsås                      21:14 2
```
