# resistor_calculator
resistor_calculator is a simple excercise I did to learn Rust. It takes a list of integers separated by a space as resistance values and calculate the parallel.
Anything that is not an integer will prompt an error (yes, floats are not allowed). The program will run until a correct list of integers is given, when it calculates the parallel it stops.

```
write a list of resistor values separated by a space
500 600g
error: invalid digit found in string
write a list of resistor values separated by a space
500 500
The parallel of [500, 500] is 250 Ω
```

I would like to add series and parallels combination in the future.
