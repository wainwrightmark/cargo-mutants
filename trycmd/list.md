# List possible mutants

## List mutants as text

```trycmd
$ cargo-mutants mutants --list -d testdata/tree/factorial
src/bin/main.rs:1: replace main with ()
src/bin/main.rs:7: replace factorial -> u32 with Default::default()

```

## List mutants as text, with diffs

```trycmd
$ cargo-mutants mutants --list --diff -d testdata/tree/factorial
src/bin/main.rs:1: replace main with ()
--- src/bin/main.rs
+++ replace main with ()
@@ -1,12 +1,10 @@
 fn main() {
-    for i in 1..=6 {
-        println!("{}! = {}", i, factorial(i));
-    }
+() /* ~ changed by cargo-mutants ~ */
 }
 
 fn factorial(n: u32) -> u32 {
     let mut a = 1;
     for i in 2..=n {
         a *= i;
     }
     a

src/bin/main.rs:7: replace factorial -> u32 with Default::default()
--- src/bin/main.rs
+++ replace factorial with Default::default()
@@ -1,19 +1,15 @@
 fn main() {
     for i in 1..=6 {
         println!("{}! = {}", i, factorial(i));
     }
 }
 
 fn factorial(n: u32) -> u32 {
-    let mut a = 1;
-    for i in 2..=n {
-        a *= i;
-    }
-    a
+Default::default() /* ~ changed by cargo-mutants ~ */
 }
 
 #[test]
 fn test_factorial() {
     println!("factorial({}) = {}", 6, factorial(6)); // This line is here so we can see it in --nocapture
     assert_eq!(factorial(6), 720);
 }


```

## List mutants as json

```trycmd
$ cargo-mutants mutants --list --json -d testdata/tree/factorial
[
  {
    "file": "src/bin/main.rs",
    "line": 1,
    "function": "main",
    "return_type": "",
    "replacement": "()"
  },
  {
    "file": "src/bin/main.rs",
    "line": 7,
    "function": "factorial",
    "return_type": "-> u32",
    "replacement": "Default::default()"
  }
]
```
