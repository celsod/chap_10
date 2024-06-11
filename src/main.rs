fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.2};
}

/*This function accepts a parameter of type T and outputs a parameter of type T.  T is the generic type.
The function signature needs the T in angled brackets. */
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*Structure using generic parameter type T. The inputs to the struct need to be the same type due to only specifying on type T.  
If they are different, then the code will not work.*/
struct Point<T> {
    x: T,
    y: T,
}

/*This structure can accept two generic types now.  If a third item was placed in the structure, it will be either T or U.
If a third type was needed, then a third generic needs to be defined. */
struct Point1<T, U> {
    x: T,
    y: U,
}

/*Here is an implementation of a method using the generic T.  We have to declare <T> after the impl part to ensure the compiler
understands that the T in Point is a generic type and not a concrete type.  The impl<T> could be any type or letter. */
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}