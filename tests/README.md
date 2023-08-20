# Testing
- Search compiled tests: ```cargo build --tests``` then ```ls target/debug/deps | grep <search-term>```
- ```cargo expand --test <test>```
- If OS complains "too many open files" again: ```ulimit -n <new_number>``` (maybe try 10000, but verify this won't bog everything).


To view prettified logs:
```TEST_LOG=true cargo test | bunyan```
May need to run ```cargo install bunyan```.