# aaa - cli

[![Build Status](https://travis-ci.org/moparisthebest/adjective-adjective-animal.svg?branch=master)](https://travis-ci.org/moparisthebest/adjective-adjective-animal) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/moparisthebest/adjective-adjective-animal/blob/master/LICENSE-MIT)

Random name generator tool that gives you suitably random and reasonably unique human readable 
(and fairly adorable) ids, ala GiphyCat

## Usage

Simple! Run without any parameters, you get a name:

```sh
> aaa
IndustrialSecretiveSwan
```

Need more? Tell it how many:

```sh
> aaa 10
IronfistedSubhexagonalTern
HoroscopicDrowsyBlackfish
QuarrelsomeDoubtfulCommabutterfly
UniqueHurtYaffle
GrubbyBoskyIchthyosaurs
BolsteredSyntheticTopi
PredespondentAssociativeOkapi
StereotypedComfortableScaup
CuratorialLacklustreWyvern
DazedStonyGiantschnauze
```

If you're ever confused, at least there's help:

```sh
> aaa --help
aaa 0.1.0

Author: Travis Burtrum <admin.github@moparisthebest.com>
A random name generator with results like `DeliriousHungryDolphin'.

USAGE:
    aaa [amount]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <amount>    Number of names to generate (default: 1)
```

## Installation

### Building from source

If you want (or need) to build the CLI from source, the following should not take too long. Note that you'll need a version of Rust (and Cargo which ships with the Rust distributions) before running:

```sh
> git clone https://github.com/moparisthebest/adjective-adjective-animal.git
> cd adjective-adjective-animal/cli
> cargo build --release
# test it out
> ./target/release/aaa
```
