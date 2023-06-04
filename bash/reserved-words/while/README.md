[group-start]: <> (tech)

# Bash

[group-end]: <> (tech)

[group-start]: <> (reserved-word)

# while

In Bash, the while loop is used to repeatedly execute a block of code as long as a certain condition remains true. The general syntax of a while loop in Bash is as follows:

```bash
while [ condition ]
do
   # Code to be executed
done
```
Here's a breakdown of how it works:

The while keyword is followed by a condition, enclosed in square brackets [ ]. The condition is typically an expression or a command that evaluates to either true or false.
If the condition is true, the code inside the do block will be executed. This block can contain any valid Bash commands or a series of commands.
After executing the code inside the do block, the loop will check the condition again. If the condition is still true, the loop will continue to execute the code inside the do block. If the condition is false, the loop will terminate, and the program will move on to the next line of code after the done keyword.
Here's a simple example to illustrate the usage of a while loop in Bash:

```bash
#!/bin/bash

counter=1

while [ $counter -le 5 ]
do
   echo "Count: $counter"
   ((counter++))
done
```
In this example, the while loop will execute the code inside the do block as long as the value of the counter variable is less than or equal to 5. Within the loop, it will print the current value of counter and then increment it by 1. The loop will continue until counter becomes 6, at which point the condition becomes false, and the loop terminates.

The output of the above script will be:

```bash
Count: 1
Count: 2
Count: 3
Count: 4
Count: 5
```
This is a basic explanation of how the while loop works in Bash. It provides a convenient way to repeat a block of code until a specific condition is no longer true.

Example 2: Reading Lines from a File

```bash
#!/bin/bash

while IFS= read -r line
do
   echo "Line: $line"
done < input.txt

```
In this example, the while loop reads each line from the input.txt file and assigns it to the line variable. The loop then echoes the content of each line. The loop continues until all lines in the file have been processed.

[group-end]: <> (reserved-word)
