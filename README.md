# Murph - Transpile EVM bytecode into huff

Murph can transpile this:

`60003560e01c8063552410771461001c5780632096525514610023575b6004356000555b60005460005260206000f3`

into this:

![murph output](images/output.png)

## Installation

You must install the nightly toolchain first in order to compile it

`rustup toolchain install nightly`

then you can install murph like this

`cargo install --git https://github.com/iFrostizz/murph.git`

## TODO:

- [ ] Writing more tests
- [ ] Breaking big chunks into small functions
- [x] Make JUMP / JUMPI pc fetch more smart
- [x] Ignore init code
- [ ] Input from file
- [ ] Generate stack annotations in comment
