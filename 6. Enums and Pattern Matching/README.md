enum: we can enumerate all possible variants, which is where enumeration gets its name.

The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. In addition, so are its variants: you can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

unit value () (the empty tuple type) 