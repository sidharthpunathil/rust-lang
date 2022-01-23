The &self is actually short for self: &Self. 

Here’s how it works: when you call a method with object.something(), Rust automatically adds in &, &mut, or * so object matches the signature of the method. In other words, the following are the same:

p1.distance(&p2);
(&p1).distance(&p2);

All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.