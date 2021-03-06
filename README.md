computation.rs
==============

A few samples from the Understanding Computation book written in Rust

Small Steps Semantics
---------------------

Operational Semantics define the behaviour of a program by defining structural rules of its parts. For this reduction to be executed an abstract machine is often used.
The Small step semantics is a way to define a machine that evaluates a program by repeatedly reducing it step by step (by applying rules) until the process ends in some final value.
In this section we follow the SIMPLE language as given in Chapter 1 of ["Understanding Computation"](http://computationbook.com/) by Tom Stuart and define the reduction rules of this language in Rust.

To create a simple number.

```rust
let number = Node::number(2);
// => 2
```

There is a Machine type that can reduce an existing AST with an Environment object and prints out the reduction steps.

```rust
Machine::new_with_empty_env(
    Node::add(Node::number(4), Node::number(10)),
).run();
// => 4 + 10
// => 14
```

Right now there is only a single comparison expression, evaluating to a `Boolean` node.

```rust
Machine::new_with_empty_env(
    Node::less_than(
        Node::number(10),
        Node::add(Node::number(4), Node::number(5))
    )
).run();
// => 10 < 4 + 5
// => 10 < 9
// => false
```

The small steps lib supports variables that can be make known to the Environment and assignments for generating entries in the Environment object.

```rust
let statement = Node::assign(
    "x".to_string(),
    Node::add(Node::variable("y".to_string()), Node::number(5))
);
let mut env = Environment::new();
env.add("y".to_string(), Node::multiply(Node::number(2), Node::number(6));
let mut machine = Machine::new(statement, env);
machine.run();
// => x = y + 5,  (y = 2 * 6)
// => x = y + 5,  (y = 12)
// => x = 12 + 5, (y = 12)
// => x = 17,     (y = 12)
// => do-nothing, (x = 17, y = 12)
```

Conditionals can be expressed with If nodes.

```rust
let statement = Node::if_else_cond(Node::boolean(true), Node::number(1), Node::number(10));
Machine::new(statement, Environment::new()).run();
// => if true 1 else 10
// => 1
```

Sequences allow to set two assignments which are evaluated in their order one by one.

```rust
Machine::new_with_empty_env(
    Node::sequence(
        Node::assign("x".to_string(), Node::add(Node::number(1), Node::number(1))),
        Node::assign("y".to_string(), Node::add(Node::variable("x".to_string()), Node::number(3)))
    )
).run();
// => x = 1 + 1; y = x + 3,  ()
// => x = 2; y = x + 3,      ()
// => do-nothing; y = x + 3, (x=2)
// => y = x + 3,             (x=2)
// => y = 2 + 3,             (x=2)
// => y = 5,                 (x=2)
// => do-nothing,            (x=2, y=5)
```

The SIMPLE language also offers support for a While loop.

```
let mut env = Environment::new();
env.add("x".to_string(), Node::number(1));
let node = Node::while_node(
    Node::less_than(Node::variable("x".to_string()), Node::number(4)),
    Node::assign("x".to_string(), Node::add(Node::variable("x".to_string()), Node::number(1)))
);
let mut machine = Machine::new(node, env);
machine.run();
// => while (x < 4) x = x + 1, (x=1)
// => if (x < 4) x = x + 1; while (x < 4) x = x + 1 else do-nothing, (x=1)
// => if (1 < 4) x = x + 1; while (x < 4) x = x + 1 else do-nothing, (x=1)
// => if (true) x = x + 1; while (x < 4) x = x + 1 else do-nothing, (x=1)
// => x = x + 1; while (x < 4) x = x + 1, (x=1)
// => [...]
// => do-nothing, (x=4)
```

The reduction of the `While` loop is a little more complex and turns itself into a syntactically larger program with conditional `If` and `Sequence` statements.
