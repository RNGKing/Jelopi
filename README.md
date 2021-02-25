# Machine
Rust based GPIO controller for a raspberry pi based on work by [jcreekmore](https://gist.github.com/jcreekmore). The concept is loosely based on how 
PLCs operate in industry.

# Getting Started
* Download the repo
* Run the following command in the project folder
```
cargo build
```
* To see the examples run
```
cargo run <relative_path>/example_data/register_data.json <relative_path>/example_data/instruction_set.ins 
```


# TO-DO
* Add Input Pins read to the evaluation process
* Add Output Pins set to the evaluation process
* Add in the concept of "rungs" into the evaluation process
* Create tests
* Create instruction creator application (likely Electronjs based?)
* Expand branching to be more clear and readable
* Add documentation on the basics of the OpCodes
* Add system documentation
* Create build package (using yocto?) to automate deploy to raspberry pi
* Incorporate the RPPAL package into the build definitions
