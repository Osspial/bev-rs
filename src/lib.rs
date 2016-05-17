extern crate num;

pub mod core;

use std::convert::{Into, AsRef};
use std::marker::PhantomData;
use num::{Float, FromPrimitive};

use core::BezCubePoly;

#[derive(Debug, Clone)]
pub struct Point<F: Float> {
    pub x: F,
    pub y: F
}

impl<F: Float> Into<[F; 2]> for Point<F> {
    fn into(self) -> [F; 2] {
        [self.x, self.y]
    }
}

impl<F: Float> Into<(F, F)> for Point<F> {
    fn into(self) -> (F, F) {
        (self.x, self.y)
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