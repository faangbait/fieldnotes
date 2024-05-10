After three decades of writing code in dozens of languages, I often find myself drowning in syntax. I created this cheatsheet to remind me which languages have which features so that I can code-switch between them more rapidly.

|                                       | Python    | JS/Typescript     | Rust      | Golang
|-------------                          | ------    | -----     | ----      | ------
| **OOP Principle**                     | Classes   | Protos    | Structs   | Constructors
| **Generics**                          | N/A       | &check;   | &check;   | &check;
| **Interfaces**                        | Dataclass | &check;   | Traits    | &check;
| **"Truthy" equality**                 | &check;   | &check;   | &cross;   | &cross;
| **String Munging**                    | &check;   | &check;   | &cross;   | &cross;
| **Closures**                          | Lambda    | &check;   | &check;   | &check;
| **Concurrency**                       | Async     | Promise   | Mutexes   | Goroutine
| **Concurrency Sync**                  |           |           | Channels  | Channels
| **Pass By Ref/Val**                   | Assign    | Objects ByRef; Prims ByVal | ByRef | ByVal
| **Memory Management**                 | GC        | GC        | Borrowed  | Unsafe
| **Heap/Stack Control**                | Dynamic   | Dynamic   | Box<T>    | Dynamic
| **Optional Typing**                   | &check;   | &check;   | &check;   | &cross;
