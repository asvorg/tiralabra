# Test documentation

Tests are done by unittesting the relevant functions. The main.rs and ui.rs modules are left out of testing. The tests are rerunnable simply by running ``` cargo test ```. Every function is tested with the relevant inputs.

## Tarpaulin coverage reports

![alt text](https://github.com/asvorg/tiralabra/blob/main/rsa/documentation/images/Screenshot%20from%202023-06-10%2005-52-28.png)
![alt text](https://github.com/asvorg/tiralabra/blob/main/rsa/documentation/images/Screenshot%20from%202023-06-10%2005-52-31.png "src tests")

The testing coverage is at the moment 97.6%. Testing is done by cargo test, and the coverage reports are done with [Tarpaulin](https://github.com/xd009642/tarpaulin)