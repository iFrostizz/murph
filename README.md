# Murph - Transpile EVM bytecode into huff

Murph can transpile this:

`602f8060093d393df360003560e01c8063552410771461001c5780632096525514610023575b6004356000555b60005460005260206000f3`

![into this](images/output.png)

## TODO:

- [ ] Writing more tests
- [ ] Breaking big chunks into small functions
- [ ] Make JUMP / JUMPI pc fetch more smart
- [ ] Ignore init code
