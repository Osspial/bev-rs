extern crate num;

pub mod core;

use std::convert::{Into, From, AsRef};
use std::ops::{Add, Mul, Div};
use std::marker::PhantomData;
use num::{Float, FromPrimitive};

use core::BezCubePoly;

macro_rules! npoint_ops {
    ($lhs:ty; $rhs:ty = $output:ident<$g_name:ident: $g_ty:ident> {$($field:ident),*}) => {
        impl<$g_name: $g_ty> Add<$rhs> for $lhs {
            type Output = $output<$g_name>;

            fn add(self, rhs: $rhs) -> $output<$g_name> {
                $output {
                    $($field: self.$field + rhs.$field),*
                }
            }
        }
    }
}

macro_rules! impl_npoint {
    // auxiliary/entry point workaround made by durka42
    // read the last rule of the macro first
    (@go $dims: expr; $name:ident<$g_name:ident: $g_ty:ident> {$($field:ident: $f_ty:ty),*} $fields:tt $(,$sibling:ty)*) => {
                                                              // ^ match the first duplicate of the fields
                                                                                            // ^ keep the second one as a tt
        #[derive(Debug, Clone, Copy)]
        pub struct $name<$g_name: $g_ty> {
            $(
                pub $field: $f_ty
            ),*
        }

        impl<$g_name: $g_ty> Into<[$g_name; $dims]> for $name<$g_name> {
            fn into(self) -> [$g_name; $dims] {
                [$(self.$field),*]
            }
        }

        impl<$g_name: $g_ty> Into<($($f_ty),*)> for $name<$g_name> {
            fn into(self) -> (F, F) {
                ($(self.$field),*)
            }
        }

        npoint_ops!($name<$g_name>; $name<$g_name> = $name<$g_name: $g_ty> {x, y});

        $(
            // pass the tt-wrapped fields to auxiliary macro
            impl_npoint!(@sib $name $g_name $g_ty $sibling $fields);
            npoint_ops!($name<$g_name>; $sibling = $name<$g_name: $g_ty> {x, y});
        )*
    };
    
    // auxiliary rule
    (@sib $name:ident $g_name:ident $g_ty:ident $sibling:ty {$($field:ident: $f_ty:ty),*}) => {
                                                // ^ finally destructure fields out of tt here
        impl<$g_name: $g_ty> From<$sibling> for $name<$g_name> {
            fn from(sib: $sibling) -> $name<$g_name> {
                $name {
                    $($field: sib.$field),*
                }
            }
        }
    };
    
    // entry point rule
    ($dims: expr; $name:ident<$g_name:ident: $g_ty:ident> $fields:tt $(,$sibling:ty)*) => {
                                                          // ^ match fields all together as a tt
        impl_npoint!(@go
            $dims;
            $name<$g_name: $g_ty>
            $fields // duplicate the fields
            $fields
            $(,$sibling)*
        );
    };
}

impl_npoint!{2; Point<F: Float> {
    x: F,
    y: F
}, Vector<F>}

impl_npoint!{2; Vector<F: Float> {
    x: F,
    y: F
}, Point<F>}

impl<F: Float> Vector<F> {
    pub fn len(self) -> F {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vector<F> {
        self / self.len()
    }
}

impl<F: Float> Mul<F> for Vector<F> {
    type Output = Vector<F>;

    fn mul(self, rhs: F) -> Vector<F> {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl<F: Float> Div<F> for Vector<F> {
    type Output = Vector<F>;

    fn div(self, rhs: F) -> Vector<F> {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum BezNode<F: Float> {
    Point {
        x: F,
        y: F
    },

    Control {
        x: F,
        y: F
    }
}

impl<F: Float> BezNode<F> {
    pub fn new_point(x: F, y: F) -> BezNode<F> {
        BezNode::Point {
            x: x,
            y: y
        }
    }

    pub fn new_control(x: F, y: F) -> BezNode<F> {
        BezNode::Control {
            x: x,
            y: y
        }
    }

    pub fn x(self) -> F {
        <BezNode<F> as Into<(F, F)>>::into(self).0
    }

    pub fn y(self) -> F {
        <BezNode<F> as Into<(F, F)>>::into(self).1
    }

    pub fn is_point(self) -> bool {
        use self::BezNode::*;
        match self {
            Point{..} => true,
            Control{..} => false
        }
    }

    pub fn is_control(self) -> bool {
        use self::BezNode::*;
        match self {
            Point{..} => false,
            Control{..} => true
        }
    }
}

impl<F: Float> Into<[F; 2]> for BezNode<F> {
    fn into(self) -> [F; 2] {
        use self::BezNode::*;

        match self {
            Point{x, y}    |
            Control{x, y} => [x, y]
        }
    }
}

impl<F: Float> Into<(F, F)> for BezNode<F> {
    fn into(self) -> (F, F) {
        use self::BezNode::*;

        match self {
            Point{x, y}    |
            Control{x, y} => (x, y)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BezCube<F: Float + FromPrimitive> {
    pub x: BezCubePoly<F>,
    pub y: BezCubePoly<F>
}

impl<F: Float + FromPrimitive> BezCube<F> {
    pub fn interp(&self, t: F) -> Point<F> {
        Point {
            x: self.x.interp(t),
            y: self.y.interp_unbounded(t) // The interp is already checked when we call x.interp, so we don't have to do it here
        }
    }

    pub fn derivative(&self, t: F) -> Vector<F> {
        Vector {
            x: self.x.derivative(t),
            y: self.y.derivative_unbounded(t)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BezCubeChain<C, F> 
        where C: AsRef<[BezNode<F>]>,
              F: Float {
    container: C,
    float_type: PhantomData<F>
}

impl<C, F> BezCubeChain<C, F> 
        where C: AsRef<[BezNode<F>]>,
              F: Float {
    pub fn from_container(c: C) -> Result<BezCubeChain<C, F>, BevError> {
        {
            let cslice = c.as_ref();
            if cslice.len() % 3 != 1 {
                return Err(BevError::InvalidLength)
            }

            for i in 0..cslice.len()/3 {
                let curve = &cslice[i*3..(i+1)*3+1];
                if !(curve[0].is_point()   &&
                     curve[1].is_control() &&
                     curve[2].is_control() &&
                     curve[3].is_point()) {
                    return Err(BevError::BadNodePattern)
                }
            }
        }

        Ok(BezCubeChain {
            container: c,
            float_type: PhantomData
        })
    }

    pub unsafe fn from_container_unchecked(c: C) -> BezCubeChain<C, F> {
        BezCubeChain {
            container: c,
            float_type: PhantomData
        }
    }

    pub fn unwrap(self) -> C {
        self.container
    }
}

impl<C, F> AsRef<C> for BezCubeChain<C, F> 
        where C: AsRef<[BezNode<F>]>,
              F: Float {

    fn as_ref(&self) -> &C {
        &self.container
    }
}

#[derive(Debug)]
pub enum BevError {
    BadNodePattern,
    InvalidLength
}