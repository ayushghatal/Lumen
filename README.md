# Lumen
A **simple**, **clean**, **minimal** and **fast** programming language.

## Syntax

```
# variable declaration
var identifier : Int;
var identifier : Int = 10;
var identifier : Char = 'a';
var identifier : Float = 1.63;

# string
var identifier : String = "Hello";

# constants
const identifier : Int = 10;

# function
function identifier(parameter: Type, parameter2: Type): ReturnType {
  # code here
  return something;
}

# function with no parameter
function identifier(): ReturnType {
  # code here
  return something;
}

# function with no return
function identifier(parameter: Type, parameter2: Type): Void {
  # code here
  return; # not necessary 
}

# calling function
identifier(argument, argument2); # in sequence as declared in function
var result : Int = add(5, 10);
```

Thats all I thought for now.
