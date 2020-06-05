/*
Data stored in memory can be on:
    1. the stack
        data knwon fixed-size
        push an pop
    2. the heap
        allocate
        deallocate

Ownership rules:
    1. each value has one and only one owner variable,
    2. when owner goes out of scope, the value will be dropped

Ways data interact:
    1. move
        => shallow copy, 
        => only meta-data (on the stack) is copied onto stack, 
        => the heap-data will not be copied
        => invalid the origin variable
    2. clone
        => deep copy,
        => stack and heap data both copied
        => origin variable still valid
    3. copy for stack-only data
        => types annotated with Copy trait
        * integers
        * Boolean types
        * floating point numbers
        * char type
        * Tuples as long as all members are Copy
*/

fn main() {
    println!("{}", 1);
}