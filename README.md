# Mylanguage: Halcon

In this proyect I would like to try to write a simple compiler that can compile and execute a simple language. The goal is to practice the knoledge aquired in class (Procesadores del lenguaje) and try to expand it so that the code can be executed as well as compiled. Another goal of this proyect is to practice with rust.

---
# Language definition

### Types
The types used by the language will be:
- int 
- str 
- bool
- float
- arr

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

`let variable_name: type;`

If the variable is an array, the initialization will be:
`let variable_name: arr[num_elem]: type;`


### Functions
The functions will be declared with the format:

```
fun function_name (arg: type, arg: type ...) -> type {
    ...
    return ...;
}
```

If the function has got no return type, [-> type] must not be specified

### Entry point

The program entry point will be defined by the following block:

```
beguin{

    }
```


### Raw c

Raw c code will be able to be used (to facilitate the use of stdio library) if it is written inside:

```
raw {
    // c code
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
for (declaration; initialization; condition; Increment/Decrement){

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

# Compilation and execution commands

The basic command will be 
`halcon <source file name> <output file name> [flags]`

notes:
1. A .c file will be outputted
2. if an invalid flag is passed, an invalid flag error will be outputted
3. if a wrong number of arguments is passed, an incorrect usage error will be outputted
4. if no arguments or flags are passed, a help text will be outputted

#### Invalid flag error
```
invalid flag <flag>
valid flags are <list of flags>
```

#### Incorrect Usage error 
```
Incorrect usage
Correct usage:
`halcon <source file name> <output file name> [flags]`
```

#### help
```
Mode of use
`halcon <source file name> <output file name> [flags]`

Flags:
...
```

### Valid Flags
    -c              : output c file
    -o              : output object file
    -b              : output binary file
