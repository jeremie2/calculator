# Calculator

Calculator is an educational project.<br>
It will support basic functionalities to focus on the features of the Rust programming language. Starting from a single file, the project has been updated replacing basic data types with more convenient data structures, grouping functionalities into separate files, etc...<br>
Step by step the project will be upgraded with more rust features.

# Run the program

- **Run** the application from its directory:

  ```shell
  cargo run -q
  ```

  It will prompt:

  ```
  :::: CALCULATOR ::::
  "Enter a calculation or: 'h' for history, 'q' to exit:"
  ```


# Error handling

- **Wrong operation** (+ - * /):

  ```
  ❌ Error: No valid operator found. Try again.
  ```

- Handling **wrong numbers** input:

  ```
  ❌ Error: Invalid first number. Try again.
  ```

  ```
  ❌ Error: Invalid second number. Try again.
  ```

- Handling **zero divide**:

  ```
  ❌ Error: Division by zero is not allowed. Try again.
  ```

# Show results

- Showing the result of the division `4 / 2.5`:

  ```
  👉 4 / 2.5 = 1.6
  ```

# Show history

- List of the executed operations:

  ```
  📜 ::: OPERATION HISTORY :::
  [1] 4 / 2.5 = 1.6
  [2] 10 + 15 = 25
  ```

- **Exiting**:

  ```
  👋 Goodbye!
  ```
