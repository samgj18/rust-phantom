# Rust

Just for playing with rust safe APIs.

**From docs**:

PhantomData

Zero-sized type used to mark things that “act like” they own a T.

Adding a PhantomData<T> field to your type tells the compiler that your type acts as though it stores a value of type T, even though it doesn’t really. This information is used when computing certain safety properties.

For a more in-depth explanation of how to use PhantomData<T>, please see the [Nomicon](https://doc.rust-lang.org/std/marker/struct.PhantomData.html).
