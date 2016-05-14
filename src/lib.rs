use std::convert::{Into, AsRef, AsMut};

#[derive(Debug, Clone, Copy)]
pub enum BezNode {
    Point {
        x: f32,
        y: f32
    },

    Control {
        x: f32,
        y: f32
    }
}

impl BezNode {
    pub fn new_point(x: f32, y: f32) -> BezNode {
        BezNode::Point {
            x: x,
            y: y
        }
    }

    pub fn new_control(x: f32, y: f32) -> BezNode {
        BezNode::Control {
            x: x,
            y: y
        }
    }

    pub fn x(self) -> f32 {
        <BezNode as Into<(f32, f32)>>::into(self).0
    }

    pub fn y(self) -> f32 {
        <BezNode as Into<(f32, f32)>>::into(self).1
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

impl Into<[f32; 2]> for BezNode {
    fn into(self) -> [f32; 2] {
        use self::BezNode::*;

        match self {
            Point{x, y}    |
            Control{x, y} => [x, y]
        }
    }
}

impl Into<(f32, f32)> for BezNode {
    fn into(self) -> (f32, f32) {
        use self::BezNode::*;

        match self {
            Point{x, y}    |
            Control{x, y} => (x, y)
        }
    }
}

pub struct BezCube<I> 
        where I: AsRef<[BezNode]> {
    container: I
}

impl<I> BezCube<I> 
        where I: AsRef<[BezNode]> {
    pub fn from_container(c: I) -> Result<BezCube<I>, BevError> {
        {
            let c = c.as_ref();
            if c.len() % 3 != 1 {
                return Err(BevError::InvalidLength)
            }

            for i in 0..c.len()/3 {
                let curve = &c[i*3..(i+1)*3+1];
                if !(curve[0].is_point()   &&
                     curve[1].is_control() &&
                     curve[2].is_control() &&
                     curve[3].is_point()) {
                    return Err(BevError::BadNodePattern)
                }
            }
        }

        Ok(BezCube {
            container: c
        })
    }
}

impl<I> AsRef<I> for BezCube<I>
    where I: AsRef<[BezNode]> {

    fn as_ref(&self) -> &I {
        &self.container
    }
}

impl<I> AsMut<I> for BezCube<I>
    where I: AsRef<[BezNode]> {

    fn as_mut(&mut self) -> &mut I {
        &mut self.container
    }
}

#[derive(Debug)]
pub enum BevError {
    BadNodePattern,
    InvalidLength
}