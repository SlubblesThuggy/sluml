macro_rules! gen_vec {
    ($name:ident, $T:ty, $($m:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $name {
            $(
                pub $m: $T,
            )*
        }

        impl_vec!($name, $T, $($m),*);

        impl_vec!(@dot $name, $T, $($m),*);
        impl_vec!(@magnitude $name, $T);
        impl_vec!(@normalize $name, $T);

        impl_vec!(@add $name, $($m),*);
        impl_vec!(@sub $name, $($m),*);
        impl_vec!(@mul $name, $T, $($m),*);
        impl_vec!(@div $name, $T, $($m),*);

        impl_vec!(@neg $name, $($m),*);

        impl_vec!(@index_mut $name, $T, $($m),*);
    }
} pub(crate) use gen_vec;

macro_rules! impl_vec {
    ($name:ident, $T:ty, $($m:ident),*) => {
        impl $name {
            pub fn new($($m: $T),*) -> Self {
                Self { $($m),* }
            }
        }
    };

    (@cross_vec3 $name:ident, $T:ty, $x:ident, $y:ident, $z:ident) => {
        impl Cross for $name {
            type Output = Self;

            fn cross(self, rhs: Self) -> Self {
                Self {
                    $x: self.$y * rhs.$z - self.$z * rhs.$y,
                    $y: self.$z * rhs.$x - self.$x * rhs.$z,
                    $z: self.$x * rhs.$y - self.$y * rhs.$x,
                }
            }
        }
    };

    (@dot $name:ident, $T:ty, $($m:ident),*) => {
        impl Dot for $name {
            type Output = $T;

            fn dot(self, rhs: Self) -> $T {
                0 as $T
                $(+ self.$m * rhs.$m)*
            }
        }
    };

    (@magnitude $name:ident, $T:ty) => {
        impl Magnitude<$T> for $name {
            fn magnitude(self) -> $T {
                self.sq_magnitude().sqrt()
            }
        }
    };

    (@normalize $name:ident, $T:ty) => {
        impl Normalize<$T> for $name {}
    };

    (@index_mut $name:ident, $T:ty, $($m:ident),*) => {
        impl Index<usize> for $name {
            type Output = $T;

            fn index(&self, index: usize) -> &Self::Output {
                indexed_match!(index; $(&self.$m),*)
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                indexed_match!(index; $(&mut self.$m),*)
            }
        }
    };

    (@add $name:ident, $($m:ident),*) => {
        impl Add for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    $(
                        $m: self.$m + rhs.$m,
                    )*
                }
            }
        }
    };

    (@sub $name:ident, $($m:ident),*) => {
        impl Sub for $name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    $(
                        $m: self.$m - rhs.$m,
                    )*
                }
            }
        }
    };

    (@mul $name:ident, $T:ty, $($m:ident),*) => {
        impl Mul for $name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self {
                    $(
                        $m: self.$m * rhs.$m,
                    )*
                }
            }
        }

        impl Mul<$T> for $name {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self {
                Self {
                    $(
                        $m: self.$m * rhs,
                    )*
                }
            }
        }
    };

    (@div $name:ident, $T:ty, $($m:ident),*) => {
        impl Div for $name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self {
                    $(
                        $m: self.$m / rhs.$m,
                    )*
                }
            }
        }

        impl Div<$T> for $name {
            type Output = Self;

            fn div(self, rhs: $T) -> Self {
                Self {
                    $(
                        $m: self.$m / rhs,
                    )*
                }
            }
        }
    };

    (@neg $name:ident, $($m:ident),*) => {
        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    $(
                        $m: -self.$m,
                    )*
                }
            }
        }
    };
} pub(crate) use impl_vec;

macro_rules! indexed_match {
    // (what we're matching; the different match results comma-separated in order)
    ($e:expr; $($rest:tt)*) => {
        indexed_match!{@(0; $e; $($rest)*,)}
    };
    
    // generates one match arm and recursively calls itself until all potential match results have been processed
    (@($idx:expr; $e:expr; $val:expr, $($rest:tt)*) $($arms:tt)*) => {
        indexed_match!{
            // @(next index; match argument; remaining match results)
            @(1+$idx; $e; $($rest)*)

            // match arms that have been generated
            $($arms)* x if x == $idx => $val,
        }
    };
    
    // produces the final output
    (@($idx:expr; $e:expr; $(,)?) $($arms:tt)* ) => {
        match $e {
            $($arms)*
            _ => panic!("Index out of bounds.")
        }
    };
} pub(crate) use indexed_match;