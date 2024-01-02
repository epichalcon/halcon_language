# Mylanguage: Halcon

![Work In Progress](https://img.shields.io/badge/Work%20In%20Progress-orange?style=for-the-badge)

In this proyect I would like to try to write a simple interpreter that can interpret and execute a simple language. The goal is to practice the knoledge aquired in class (Procesadores del lenguaje) and through the book writing an interpreter with GO and try to expand it so that the code can be executed. Another goal of this proyect is to practice with rust and develop a medum sized proyect with several modules that interact with eachother

---
# Language definition

As I am following the "writing an interpreter in go" book, this language will have dynamic typing and will be expression based.

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
    - Post and preincrement ´++´ _TODO_
    - Post and predecrement ´--´ _TODO_
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

If the function has got no return type, [-> type] must not be specified

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

TODO: elif

### For
The for block will be defined as:

```
for (declaration; initialization; condition; Increment/Decrement){

    }

```

TODO: not yet implemented for loop

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

TODO: not yet implemented for loop

### Comments
The language will have simple comments starting with ´//´ that continue untill a linebreak

// this is a comment

TODO: not yet implemented coments

# Compilation and execution commands

The basic command will be 
`halcon [<source file name>]`

the file extension must be .hc

If a source file is not specified, a REPL will be executed which will prompt the user to provide commands one at a time.
