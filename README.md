# zeppelin-sigil

:warning: Prototype :warning:

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

[[https://github.com/elopio/zeppelin-sigil/blob/master/examples/sigil1.png|alt=sigil example 1]]
[[https://github.com/elopio/zeppelin-sigil/blob/master/examples/sigil2.png|alt=sigil example 2]]
[[https://github.com/elopio/zeppelin-sigil/blob/master/examples/sigil3.png|alt=sigil example 3]]

Visual representation of Ethereum addresses using magical sigils.

## Security

Representing an Ethereum address as an image like we do here is **NOT SECURE**.
We have chosen aesthetics first, which is always the wrong choice for highly
secure projects.

The sigils generated are not unique. We are scaling down the sigil so many
different addresses will result in the same lines. And even if we didn't scale
them down, some of the lines will be too close, becoming indistinguishable for
the human eye. Then there's the problem of loops in the address, that will make
different line routes to result in the same sigil.

The image includes the address on the rim of the circle, so every image will be
unique. But that shouldn't be used as a guarantee. There's nothing stopping bad
hombres to generate a sigil and inscribe a different address on the rim.

We will explore some of these topics, like using circles to represent loops.
But in the end, this is nothing more than a game to learn Rust. Use at your own
risk.

## Background

On the album Led Zeppelin IV, each of the members of the band chose
[a sigil to represent themselves](https://en.wikipedia.org/wiki/Led_Zeppelin_IV#The_four_symbols)
as a compromise between anonymity and identity.

At [Zeppelin](https://zeppelin.solutions/), we follow the teachings of these
wise men.

## Install

TODO publish the cargo crate.

## Usage

TODO pass the address from the command line to the binary.

```
cargo run
```

## Acknowledgements

[Cullen Miller](http://pointlinesurface.com/) and
[Gabriel Dunne](http://gabrieldunne.com/) for their magical blockchain
performance art,
[Claves Angelicae](https://claves-angelicae.github.io/dossier/). Check it out!

[Automata Network](http://automata.network/) for the impromptu mini hackathon.

[Zeppelin](https://zeppelin.solutions) for being all kinds of awesome. We get
a lot of time to learn and explore technologies related to
descentralization, security, usability, and more. And
[we are hiring!](https://zeppelin.solutions/jobs/)

## Maintainers

[@elopio](https://github.com/elopio)

## Contribute

Do you like Led Zeppelin? If so, you are welcome to contribute to this
project doing anything that you find fun.

There's a lot to do to clean up the prototype script; and my Rust is still very
limited so many things can be improved.

## License

[GPL-3.0-or-later](LICENSE) © 2018 Leo Arias
