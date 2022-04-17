# Valet

Stores your objects, and gives you a tag to retrieve them later.

This is my own solution to the problem of self-referential data. A common
pattern is to eliminate cycles by storing all items in a central collection, and
where you would usually store references or smart pointers, you instead store an
index/key into the collection. In this case, the `Valet` type is the collection,
and the `Tag` type is the index.

I found that I was often wanting to use this pattern in different projects, and
so instead of writing the same boilerplate to generate indexes I decided to turn
it into a library. That also gave me an excuse to add other nice-to-have
features, like better type safety for indexes, using a generic newtype `Tag<T>`
instead of a plain type like `u64`.