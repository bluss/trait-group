/// Create a new trait that acts like an alias or “group” of a bunch of
/// traits.
///
/// The macro works by creating a new trait and making one blanket implementation
/// of it for all types that implement the traits in the group. It
/// replaces `Self` automatically in the impl.
///
/// The trait may be declared with `pub` or without.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate trait_group;
///
/// use std::ops::Add;
/// 
/// trait_group! {
///     /// You can document the trait here
///     pub trait CanAdd : Add<Self, Output=Self> + Copy
/// }
/// 
/// fn foo<T: CanAdd>(x: T) -> T { x + x }
/// 
/// fn main() { println!("{}", foo(2)); }
/// 
/// ```
#[macro_export]
macro_rules! trait_group {
    (@as_items $($it:item)*) => ($($it)*);
    (@replace_self with $rep:tt [$($st:tt)*] Self $($tail:tt)*) => {
        trait_group!{@replace_self with $rep [$($st)* $rep] $($tail)*}
    };
    (@replace_self with $rep:tt [$($st:tt)*] $t:tt $($tail:tt)*) => {
        trait_group!{@replace_self with $rep [$($st)* $t] $($tail)*}
    };
    (@replace_self with $rep:tt [$($st:tt)*]) => {
        trait_group!{@as_items $($st)*}
    };
    // User-facing rule: pub trait
    ($(#[$attr:meta])* pub trait $name:ident : $($t:tt)+) => {
        trait_group!{@as_items $(#[$attr])* pub trait $name : $($t)+ { }}
        trait_group!{@replace_self with T [] impl<T> $name for T where T: $($t)+ { }}
    };
    // User-facing rule: (not pub) trait 
    ($(#[$attr:meta])* trait $name:ident : $($t:tt)+) => {
        trait_group!{@as_items $(#[$attr])* trait $name : $($t)+ { }}
        trait_group!{@replace_self with T [] impl<T> $name for T where T: $($t)+ { }}
    };
}


#[cfg(test)]
mod tests {
    #[test]
    #[allow(deprecated)]
    fn it_works() {
        trait_group! {
            /// This is the documentation
            #[deprecated(note = "just a test")]
            trait Test : Extend<u8> + Default
        }

        fn foo<V: Test>() -> V { let mut v = V::default(); v.extend(Some(0)); v }

        assert_eq!(foo::<Vec<_>>(), vec![0]);
    }
}
