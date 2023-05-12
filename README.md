# resistor_calculator

resistor_calculator is a simple excercise I did to learn Rust. It takes a circuit of parallels and series of resistances, listed as numbers separated by a space.
The program will calculate the equivalent resistance of the circuit computing parallels and series, even nested.

```
write a circuit of resistor values separated by a space
s(100 200 p(200 200) 300 p(470 230))
The resistance of s(100 200 p(200 200) 300 p(470 230))
 is 854.4286 Ω
```

If the circuit is not valid for any reason (parenthesis, pattern different from s(...) or p(...)) the program will stop and display an error.

It is written really bad, I know, but it's the first tool I've ever wrote till the end and I'm so happy.
