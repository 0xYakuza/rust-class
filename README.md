# rust-class
Class in rust

# Example usage
```rust
#[macro_use]
use rust_class::class;

class! {
    class Person {
        [String name];

        Self new(name: String) {
            Self {
                name: name,
            }
        }

        String get_name(self) {
            self.name
        }
    }

    class Sum {
        [
            u8 num1
            u8 num2
        ];

        Self new(num1: u8, num2: u8) {
            Self {
                num1: num1,
                num2: num2,
            }
        }

        u8 sum(self) {
            self.num1 + self.num2
        }
    }
}

fn main() {
    let person_test = Person::new("Alice".to_string());
    println!("{}", hl_test.get_name());

    let sum_test = Sum::new(1, 4);
    println!("{}", an_test.sum());
}
```