Write an infix to postfix converter for the simple arithmetic operators( + , - , * , / )

Precedence in this case means higher order of evaluation. So multiply('*') and divide('/') have higher precedence than addition('+') and subtraction('-'). But multiply has the same precedence as divide, and addition the same as subtraction.

Algorithm for infix to postfix conversion:

Scan the infix string from left to right
- If it's NOT a paren or an operator print the character out
- If it's a left paren '(' push the left paren onto the stack
- If it's a right paren start a loop:
  - pop an operator off the stack
  - if the popped operator is a left paren '(' break out of the loop
  - if the popped operator is NOT a left paren '(' print it out followed by a space
- If it's any other operator( + , - , * , / )
  - if the stack is empty push the scanned operator onto the stack
  - otherwise start a loop:
    - if the top of the stack is a left paren '(', push the scanned operator onto the stack and break out of the loop
    - if the top of the stack has < precedence to the scanned operator, push the scanned operator onto the stack and break out of the loop
    - if the top of the stack has >= precedence to the scanned operator pop that item off the stack, and print out the popped operator followed by a space, continue the loop(the pop will cause the top of the stack to have changed)

Once the entire infix string has been scanned and processed:
- while the stack is NOT empty, repeatedly pop off and print out operators

Test cases:

| input | expected output |
| :-----: | :---------------: |
| 1 + 2 * 3 - 4 | 1 2 3 * + 4 - |
| 1 + 2 * ( 3 - 4 ) * ( 5 + 6 * 7 ) - 8 | 1 2 3 4 - * 5 6 7 * + * + 8 - |