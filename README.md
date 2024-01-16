# RedDebugger

## The Correspondence of Data Types

The correspondence between NBT types and Rust types are shown below, in which some types are come from the third-party crate [`fastnbt`](https://github.com/owengage/fastnbt):

| NBT Type     | Rust Type                    |
| ------------ | ---------------------------- |
| `Byte`       | `i8`                         |
| `Boolean`    | `bool`                       |
| `Short`      | `i16`                        |
| `Int`        | `i32`                        |
| `Long`       | `i64`                        |
| `Float`      | `f32`                        |
| `Double`     | `f64`                        |
| `String`     | `Cow<'a, str>`               |
| `List`       | `Vec<T>`                     |
| `Compound`   | `pub struct CompoundName {}` |
| `Byte Array` | `fastnbt::ByteArray`         |
| `Int Array`  | `fastnbt::IntArray`          |
| `Long Array` | `fastnbt::LongArray`         |

Although the enum `Value` is available in `fastnbt`, we still recommend using specific types listed above to make later data process easily.