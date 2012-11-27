/**
 * Angle and Trigonometry Functions
 *
 * This module corresponds to Section 8.1 of the [GLSL 4.30.6 specification]
 * (http://www.opengl.org/registry/doc/GLSLangSpec.4.30.6.pdf).
 */
use num::cast::{NumCast, cast};
use angle::Radians;
use vec::{Vec3, Vec2, Vec4};

///
/// Triganomic functions
///
/// http://en.wikipedia.org/wiki/Trigonometric_functions
///
priv trait Trig<T> {
    pure fn sin() -> T;
    pure fn cos() -> T;
    pure fn tan() -> T;
}

#[inline(always)] pub pure fn sin<T:Trig<R>, R>(theta: &T) -> R { theta.sin() }
#[inline(always)] pub pure fn cos<T:Trig<R>, R>(theta: &T) -> R { theta.cos() }
#[inline(always)] pub pure fn tan<T:Trig<R>, R>(theta: &T) -> R { theta.tan() }

priv impl<T:Copy Num NumCast> Radians<T>: Trig<T> {
    #[inline(always)] pure fn sin() -> T { cast(f64::sin(cast(*self))) }
    #[inline(always)] pure fn cos() -> T { cast(f64::cos(cast(*self))) }
    #[inline(always)] pure fn tan() -> T { cast(f64::tan(cast(*self))) }
}

pub impl<T:Copy Num NumCast> Vec2<Radians<T>>: Trig<Vec2<T>>  {
    #[inline(always)]
    pure fn sin() -> Vec2<T> {
        Vec2::new(sin(&self[0]),
                  sin(&self[1]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec2<T> {
        Vec2::new(cos(&self[0]),
                  cos(&self[1]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec2<T> {
        Vec2::new(tan(&self[0]),
                  tan(&self[1]))
    }
}

pub impl<T:Copy Num NumCast> Vec3<Radians<T>>: Trig<Vec3<T>>  {
    #[inline(always)]
    pure fn sin() -> Vec3<T> {
        Vec3::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec3<T> {
        Vec3::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec3<T> {
        Vec3::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]))
    }
}

pub impl<T:Copy Num NumCast> Vec4<Radians<T>>: Trig<Vec4<T>>  {
    #[inline(always)]
    pure fn sin() -> Vec4<T> {
        Vec4::new(sin(&self[0]),
                  sin(&self[1]),
                  sin(&self[2]),
                  sin(&self[3]))
    }
    
    #[inline(always)]
    pure fn cos() -> Vec4<T> {
        Vec4::new(cos(&self[0]),
                  cos(&self[1]),
                  cos(&self[2]),
                  cos(&self[3]))
    }
    
    #[inline(always)]
    pure fn tan() -> Vec4<T> {
        Vec4::new(tan(&self[0]),
                  tan(&self[1]),
                  tan(&self[2]),
                  tan(&self[3]))
    }
}

///
/// Inverse triganomic functions
///
/// http://en.wikipedia.org/wiki/Inverse_trigonometric_functions
///
pub trait InvTrig {
    pure fn asin() -> Radians<self>;
    pure fn acos() -> Radians<self>;
    pure fn atan() -> Radians<self>;
}

#[inline(always)] pub pure fn asin<T:InvTrig>(x: &T) -> Radians<T> { x.asin() }
#[inline(always)] pub pure fn acos<T:InvTrig>(x: &T) -> Radians<T> { x.acos() }
#[inline(always)] pub pure fn atan<T:InvTrig>(x: &T) -> Radians<T> { x.atan() }

pub impl f32: InvTrig {
    #[inline(always)] pure fn asin() -> Radians<f32> { Radians(f32::asin(self)) }
    #[inline(always)] pure fn acos() -> Radians<f32> { Radians(f32::acos(self)) }
    #[inline(always)] pure fn atan() -> Radians<f32> { Radians(f32::atan(self)) }
}

pub impl f64: InvTrig {
    #[inline(always)] pure fn asin() -> Radians<f64> { Radians(f64::asin(self)) }
    #[inline(always)] pure fn acos() -> Radians<f64> { Radians(f64::acos(self)) }
    #[inline(always)] pure fn atan() -> Radians<f64> { Radians(f64::atan(self)) }
}

pub impl float: InvTrig {
    #[inline(always)] pure fn asin() -> Radians<float> { Radians(f64::asin(cast(self)).to_float()) }
    #[inline(always)] pure fn acos() -> Radians<float> { Radians(f64::acos(cast(self)).to_float()) }
    #[inline(always)] pure fn atan() -> Radians<float> { Radians(f64::atan(cast(self)).to_float()) }
}

// TODO: figure out how to merge with InvTrig
pub trait InvTrigV<T> {
    pure fn asin() -> T;
    pure fn acos() -> T;
    pure fn atan() -> T;
}

pub impl<T:Copy Num NumCast InvTrig> Vec2<T>: InvTrigV<Vec2<Radians<T>>>  {
    #[inline(always)]
    pure fn asin() -> Vec2<Radians<T>> {
        Vec2::new(asin(&self[0]),
                  asin(&self[1]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec2<Radians<T>> {
        Vec2::new(acos(&self[0]),
                  acos(&self[1]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec2<Radians<T>> {
        Vec2::new(atan(&self[0]),
                  atan(&self[1]))
    }
}

pub impl<T:Copy Num NumCast InvTrig> Vec3<T>: InvTrigV<Vec3<Radians<T>>>  {
    #[inline(always)]
    pure fn asin() -> Vec3<Radians<T>> {
        Vec3::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec3<Radians<T>> {
        Vec3::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec3<Radians<T>> {
        Vec3::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]))
    }
}

pub impl<T:Copy Num NumCast InvTrig> Vec4<T>: InvTrigV<Vec4<Radians<T>>>  {
    #[inline(always)]
    pure fn asin() -> Vec4<Radians<T>> {
        Vec4::new(asin(&self[0]),
                  asin(&self[1]),
                  asin(&self[2]),
                  asin(&self[3]))
    }
    
    #[inline(always)]
    pure fn acos() -> Vec4<Radians<T>> {
        Vec4::new(acos(&self[0]),
                  acos(&self[1]),
                  acos(&self[2]),
                  acos(&self[3]))
    }
    
    #[inline(always)]
    pure fn atan() -> Vec4<Radians<T>> {
        Vec4::new(atan(&self[0]),
                  atan(&self[1]),
                  atan(&self[2]),
                  atan(&self[3]))
    }
}



///
/// Hyperbolic functions
///
/// http://en.wikipedia.org/wiki/Hyperbolic_function
///
pub trait Hyp {
    pure fn sinh() -> self;
    pure fn cosh() -> self;
    pure fn tanh() -> self;
    // pure fn asinh() -> self;
    // pure fn acosh() -> self;
    // pure fn atanh() -> self;
}

#[inline(always)] pub pure fn sinh<T:Hyp>(x: &T) -> T { x.sinh() }
#[inline(always)] pub pure fn cosh<T:Hyp>(x: &T) -> T { x.cosh() }
#[inline(always)] pub pure fn tanh<T:Hyp>(x: &T) -> T { x.tanh() }

pub impl f32: Hyp {
    #[inline(always)] pure fn sinh() -> f32 { f32::sinh(self) }
    #[inline(always)] pure fn cosh() -> f32 { f32::cosh(self) }
    #[inline(always)] pure fn tanh() -> f32 { f32::tanh(self) }
}

pub impl f64: Hyp {
    #[inline(always)] pure fn sinh() -> f64 { f64::sinh(self) }
    #[inline(always)] pure fn cosh() -> f64 { f64::cosh(self) }
    #[inline(always)] pure fn tanh() -> f64 { f64::tanh(self) }
}

pub impl float: Hyp {
    #[inline(always)] pure fn sinh() -> float { cast(f64::sinh(cast(self))) }
    #[inline(always)] pure fn cosh() -> float { cast(f64::cosh(cast(self))) }
    #[inline(always)] pure fn tanh() -> float { cast(f64::tanh(cast(self))) }
}

pub impl <T:Copy Hyp> Vec2<T>: Hyp {
    #[inline(always)]
    pure fn sinh() -> Vec2<T> {
        Vec2::new(sinh(&self[0]),
                  sinh(&self[1]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec2<T> {
        Vec2::new(cosh(&self[0]),
                  cosh(&self[1]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec2<T> {
        Vec2::new(tanh(&self[0]),
                  tanh(&self[1]))
    }
}

pub impl <T:Copy Hyp> Vec3<T>: Hyp {
    #[inline(always)]
    pure fn sinh() -> Vec3<T> {
        Vec3::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec3<T> {
        Vec3::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec3<T> {
        Vec3::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]))
    }
}

pub impl <T:Copy Hyp> Vec4<T>: Hyp  {
    #[inline(always)]
    pure fn sinh() -> Vec4<T> {
        Vec4::new(sinh(&self[0]),
                  sinh(&self[1]),
                  sinh(&self[2]),
                  sinh(&self[3]))
    }
    
    #[inline(always)]
    pure fn cosh() -> Vec4<T> {
        Vec4::new(cosh(&self[0]),
                  cosh(&self[1]),
                  cosh(&self[2]),
                  cosh(&self[3]))
    }
    
    #[inline(always)]
    pure fn tanh() -> Vec4<T> {
        Vec4::new(tanh(&self[0]),
                  tanh(&self[1]),
                  tanh(&self[2]),
                  tanh(&self[3]))
    }
}