trait A {}

enum Names<T: A> {
    Named(T),
    Unnamed(T),
}
