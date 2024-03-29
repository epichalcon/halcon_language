# Mylanguage: Halcon

In this proyect I would like to try to write a simple interpreter that can interpret and execute a simple language. The goal is to practice the knoledge aquired in class (Procesadores del lenguaje) and through the book ["writing an interpreter with GO"](https://interpreterbook.com) and expand on it to write my own programming language. Another goal of this proyect is to practice with rust and develop a medum sized proyect with several modules that interact with eachother.

---

# Building the proyect

As the proyect is writen in rust and uses the Cargo package manager, rust and cargo must be installed in the system. To install them follow the instructions in this [link](https://doc.rust-lang.org/cargo/getting-started/installation.html) 

To build the proyect do the following steps:
- clone the repo
- compile the proyect (this will generate the executable in the target/release folder)
- copy the executable to the root folder

```
git clone https://github.com/epichalcon/halcon_language.git
cd halcon_language
cargo build --release
cp target/release/halcon .
```

# Compilation and execution commands

The basic command will be 
`./halcon [<source file name>]`

the file extension must be .hc

If a source file is not specified, a REPL will be executed which will prompt the user to provide commands one at a time.


# Language definition

As I am following the "writing an interpreter in go" book and expanding on its content to add functionality I want to have like loops, elifs and some operators that aren't included, this language will have dynamic typing and will be expression based.

### Types
The types used by the language will be:
- int 
- str 
- bool
- arr
- dict


### Operators
The language will have the following operators:
- Aritmetic:
    - Plus ´+´
    - Minus ´-´
    - Multiplication ´*´
    - Division ´/´
    - Modulus ´%´
- Relation:
    - Equals ´==´
    - Not equals ´!=´
    - Less than ´<´
    - Grater than ´>´
    - Less or equal ´<=´
    - Grater or equal ´>=´
- Logic:
    - and ´and´
    - or ´or´
    - not ´not´
- Increment Decrement:
    - Post and preincrement ´++´
    - Post and predecrement ´--´ 
- Asignation:
    - Simple asignation ´=´
    - SumAsignation ´+=´
    - MinusAsignation ´-=´
    - MultiplyAsignation ´*=´
    - DivideAsignation ´/=´

Parentheses will be used to change the order or priority of the operations. By default the order will be from left to right

### Identifiers
Variable and function identifiers must start with a letter (lower or uppercase) and must only have the following characters:
- `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_`

### Variable

Variables must be initialized berfore being used with the following format:

`let variable_name = value;`

If the variable is an array, the initialization will be:
`let variable_name = [val1, val2 ...];`


### Functions
The functions will be declared with the format:

```
let function_name = fun (arg1, arg2...) {
    ...
    return ...;
}
```

### If/else

The if else block will be defined as:

```
if(condition){

    }
elif(condition){

    }
else{

    }
```
### For
The for block will be defined as:

```
for (initialization; condition; Increment/Decrement){

    }

```

### While
The while block will be defined as:

```
while(condition){

    }
```

If it is an infinite loop it can be declared as:

```
loop{

    }
```

where all of the fields are optional

### Comments
The language will have simple comments starting with ´//´ that continue untill a linebreak

// this is a comment

