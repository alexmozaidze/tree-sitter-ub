# Tree-sitter pointer UB

A sample program that causes undefined behaviour with Tree-sitter Rust bindings.

## UB

UB occurs when calling `QueryCursor::captures`. This method returns an iterator over all the captures, but it seems to return duplicates whenever we iterate over it or construct a collection with it. This unsound behaviour seems to go away if one were to use `.cloned()` method, which implies that there is some incorrect pointer&rarr;reference conversion within the bindings.
