# Calculator

Calculator is an educational project.
It will support basic functionality to focus on the features of the Rust programming language. Starting from a single file, the project has been updated replacing basic data types with more convenient data structures, grouping functionality into separate files, etc...
Step by step the project will be upgrated with more rust features.

# Run the program

- **Run** the application from its directory:

  ```shell
  cargo run -q
  ```

  It will prompt:

  ```
  :::: CALCULATOR ::::
  Select the operation (+ - * /) or a command (h = history, q = exit):
  ```


# Error handling

- **Wrong operation** (+ - * /):

  ```
  ❌ Error: invalid command or operator. Try again.
  ```

- Handling **wrong number** input:

  ```
  ❌ Error: insert a valid number.
  ```

- Handling **zero divide**:

  ```
  ❌ Error: can't divide by zero. Try again.
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
