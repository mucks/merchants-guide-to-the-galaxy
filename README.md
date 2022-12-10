# Merchants Guide to the Galaxy


## Constraints

* word assignement is only to one individual roman numeral
	* 'glob is I' is valid!
	* 'prok is V' is valid!
	* 'phish is X' is valid!
	* 'teesh is III' is invalid!
* words can't be  assigned multiple times
	* 'glob is I' is valid!
	* 'glob is V' is invalid!

* sentence input to credits only contains one non roman assigned word
	* 'glob glob Silver is 34 Credits' is valid
	* 'glob prok Gold is 57800 Credits' is valid
	* 'glob prok Gold Silver is 55555 Credits' is invalid

* only questions of 'how much is' and 'how many Credits is' are supported
	

## Instructions
* install rust
	* instructions on https://rust-lang.org/tools/install
* to run the application in interactive mode use 'cargo run'
* to run the application with input from a file 'cargo run file-path'
* to test the application use 'cargo test'