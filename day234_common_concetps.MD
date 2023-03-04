### 1.1 Variable and mutabiliy

- Using **let** to define variables in Rust

- By default variable in Rust is **immutable**, so can not re-assign value to a immutable variable directly (like below)

  ```rust
  // complie error -> error: could not compile `variables` due to previous error
  fn main() {
      let x = 5;
      println!("The value of x is: {x}");
      x = 6;
      println!("The value of x is: {x}");
  }
  ```

  

- Using **mut** to define a mutable variable so that you can change the variable directly

  ```rust
  // no complie error will print 5 and 6
  fn main() {
      let mut x = 5;
      println!("The value of x is: {x}");
      x = 6;
      println!("The value of x is: {x}");
  }
  ```



### 1.2 Constants

Like immutable variables, *constants* are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

- Not allow to use **mut** to define a **constant**, they are always immutable

- And the type of the constant must be declared

- Constant can be set to a constant expression

  ```rust
  fn main() {
      const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
  }
  ```

### 1.3 Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is *shadowed* by the second, which means that the second variable is what the compiler will see when you use the name of the variable. In effect, the second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the **let** keyword as follows:

```rust
// print shadow value 12 in the inner scope and print 6 in outer scope
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```rust
// no complie erorr - shadow can change data type as well
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

The first `spaces` variable is a string type and the second `spaces` variable is a number type. Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; instead, we can reuse the simpler `spaces` name. However, if we try to use `mut` for this, as shown here, we’ll get a compile-time error:

```rust
// complie error - mut can not change data type
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len();
}
```

### 1.4 Data and types

Every value in Rust is of a certain *data type*, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: scalar and compound.

Keep in mind that Rust is a *statically typed* language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

**It's always always a good practise to specify the data type when we declare it.**

#### Scalar types

- integers
- float
- boolean
- character

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let f: bool = false; // boolean with explicit type annotation
    let z: char = 'ℤ'; // char with explicit type annotation
}

// numeric operations
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```



#### Compound types

- tuple
- array

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
  
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
  
    let a = [1, 2, 3, 4, 5]; // array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array with data type i32
    let a = [3; 5]; // create array [3,3,3,3,3]
}

```

### 1.5 Functions

Functions are prevalent in Rust code.We define a function in Rust by entering `fn` followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends. Like many language function can have parameters and return values. A example function is as below:

```rust
// one thing need to pay attention, there is no key word return at end
// only a expression at the end
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

### 1.6 Control flow

The ability to run some code depending on whether a condition is `true` and to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

#### If expression

```rust
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

// one thing need to pay attention is that if number will have a complier error
// unlike other language can convert other data type to boolean automatically
fn main() {
    let number = 3;
  
    // complier error
    if number {
        println!("number was three");
    }
  
    // if and else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
 
    // if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // complie error
    let number = if condition { 5 } else { "six" };
}

```

#### Loop expression

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// using alias 'counting_up', we can break the outter loop directly
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

#### While expression

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

#### For expression

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
```

