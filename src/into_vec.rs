macro_rules! impl_tuple_to_vec {
    // Each invocation takes a list of idents that represent tuple fields.
    ($( $args:ident ),+ ) => {
        impl<T, $( $args ),+> ToVec<T> for ( $( $args ),+ )
        where
            $( $args: Into<T> ),+
        {
            #[allow(non_snake_case)]
            fn to_vec(self) -> Vec<T> {
                let ( $( $args ),+ ) = self;
                vec![ $( $args.into() ),+ ]
            }
        }
    };
}

pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl<T, A> ToVec<T> for Vec<A>
where
    A: Into<T>,
{
    fn to_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}

impl<'a, T, A> ToVec<T> for &'a [A]
where
    A: 'a + Into<T> + Clone,
{
    fn to_vec(self) -> Vec<T> {
        self.iter().cloned().map(Into::into).collect()
    }
}

impl<'a, T, A, const N: usize> ToVec<T> for &'a [A; N]
where
    A: 'a + Into<T> + Clone,
{
    fn to_vec(self) -> Vec<T> {
        self.iter().cloned().map(Into::into).collect()
    }
}

impl<T, A, const N: usize> ToVec<T> for [A; N]
where
    A: Into<T> + Clone,
{
    fn to_vec(self) -> Vec<T> {
        self.into_iter().map(Into::into).collect()
    }
}

impl_tuple_to_vec!(A, B);
impl_tuple_to_vec!(A, B, C);
impl_tuple_to_vec!(A, B, C, D);
impl_tuple_to_vec!(A, B, C, D, E);
impl_tuple_to_vec!(A, B, C, D, E, F);
impl_tuple_to_vec!(A, B, C, D, E, F, G);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple_to_vec!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);

#[macro_export]
macro_rules! impl_to_vec {
    ($for:ident, $lifetime:lifetime $type:ty $(, $($rest:tt)*)?) => {
        impl<$lifetime> $crate::into_vec::ToVec<$for> for &$lifetime $type {
            fn to_vec(self) -> Vec<$for> {
                vec![self.clone().into()]
            }
        }
        $($crate::impl_to_vec!($for, $($rest)*);)?
    };
    ($for:ident, $type:ty $(, $($rest:tt)*)?) => {
        impl $crate::into_vec::ToVec<$for> for $type {
            fn to_vec(self) -> Vec<$for> {
                vec![self.into()]
            }
        }
        $($crate::impl_to_vec!($for, $($rest)*);)?
    };
}

pub trait ToRows<T> {
    fn to_rows(self) -> Vec<Vec<T>>;
}

impl<T, A> ToRows<T> for Vec<A>
where
    A: ToVec<T>,
{
    fn to_rows(self) -> Vec<Vec<T>> {
        self.into_iter().map(|a| a.to_vec()).collect()
    }
}

impl<'a, T, A> ToRows<T> for &'a [A]
where
    A: 'a + ToVec<T> + Clone,
{
    fn to_rows(self) -> Vec<Vec<T>> {
        self.iter().cloned().map(|a| a.to_vec()).collect()
    }
}

impl<T, A, const N: usize> ToRows<T> for [A; N]
where
    A: ToVec<T> + Clone,
{
    fn to_rows(self) -> Vec<Vec<T>> {
        self.into_iter().map(|a| a.to_vec()).collect()
    }
}

macro_rules! impl_tuple_to_rows {
    ( $( $name:ident ),+ ) => {
        impl<T, $( $name ),+> ToRows<T> for ( $( $name ),+ )
        where
            $( $name: ToVec<T> ),+
        {
            #[allow(non_snake_case)]
            fn to_rows(self) -> Vec<Vec<T>> {
                let ( $( $name ),+ ) = self;
                vec![ $( $name.to_vec() ),+ ]
            }
        }
    };
}

impl_tuple_to_rows!(A, B);
impl_tuple_to_rows!(A, B, C);
impl_tuple_to_rows!(A, B, C, D);
impl_tuple_to_rows!(A, B, C, D, E);
impl_tuple_to_rows!(A, B, C, D, E, F);
impl_tuple_to_rows!(A, B, C, D, E, F, G);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I, J);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple_to_rows!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
