
![[Lines walking sketch]]

# Requirements

## step function

```functionplot
---
title: LargeStep
xLabel: x
yLabel: step(x)
bounds: [-2, 2, -2, 2]
disableZoom: true
grid: true
---
step(x)=x<0 ? -1 : 1
```

```rust
fn large_step(x: f32, thresold: f32) -> f32 {
    if x < thresold {
        -1.
    } else {
        1.
    }
}
```

## Vector class

```rust
struct Vec2;

/// element-wise multiplication
impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output;
}

/// element-wise addition
impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, rhs: Vec2) -> Self::Output;
}

/// element-wise substraction
impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Vec2) -> Self::Output;
}

impl Vec2 {
    /// get the norm (or the length) of the vector
    pub fn norm(self) -> f32;

    /// normalize the vector
    pub fn normalize(&mut self);

    /// return a normalized copy of the vector
    pub fn normalized(self) -> Self;

    /// rotate the vector
    pub fn rotate(&mut self, angle: f32);

    /// return a normalized copy of the vector
    pub fn rotated(self, angle: f32) -> Self;

    /// return the dot product of the vectors
    pub fn dot(self, other: Self) -> f32;

    /// return a new vector linearly interpolated to a target vector
    pub fn lerp(self, target: Self, t: f32) -> Vec2;

    /// get the angle from the vector to +1
    pub fn get_angle(&self) -> f32;

    /// tangent angle
    pub fn tan_angle(&self) -> f32;

    /// inverse tangent angle
    pub fn inv_tan_angle(&self) -> f32;

    /// floor each component
    pub fn floor(&self) -> Vec2i;
}
```

## Trigonometric functions

available in the std crate, or can be approximated

## Other maths utils

### Floor & Ceil

The integer (resp. integer + 1) part of the number

```functionplot
---
title: Floor & Ceil
xLabel: x
yLabel: floor(x) & ceil(x)
bounds: [-4.5, 4.5, -4.5, 4.5]
disableZoom: true
grid: true
---
f(x)=floor(x)
c(x)=ceil(x)
```

```rust
fn floor(x: f32) -> i32 {
    x as i32
}

fn ceil(x: f32) -> i32 {
    x as i32 + 1
}
```

# X Ray-Tracing

Compute `iniX`, the initial displacement to reach the first x-aligned line.

![[Lines walking sketch#^area=nwWLoCW2]]

```rust
let y;
if sin(theta) > 0. {
    y = ceil(pos.y) as f32 - pos.y;
} else if sin(theta) < 0. {
    y = floor(pos.y) as f32 - pos.y;
} else {
    return f32::INFINITY;
}
let ini_x = Vec2 {
    x: y / tan(theta),
    y,
};
```

Compute `deltaX`, the constant displacement to get to the next x-aligned line.

![[Lines walking sketch#^area=8gjuUXm7]]

```rust
let delta_x = Vec2 {
    x: large_step(sin(theta), 0.) / tan(theta),
    // -1 or 1, depending of the sign of sin(theta)
    y: large_step(sin(theta), 0.),
};
```

Now add `deltaX` to `pos` and `iniX` more and more (with a little more, let's call it $\Phi$).

```rust
let phi = Vec2 { x: 1., y: 0. }.rotated(theta) * 0.001;
for i in 0..MAX_ITERS {
    let v = pos + ini_x + delta_x * i as f32;
    if map.get_at((v + phi).floor()) {
        return v.norm();
    }
}
f32::INFINITY
```

# Y Ray-Tracing

Compute `iniY`, the initial displacement to reach the first x-aligned line.

![[Lines walking sketch#^area=0HkgkKWY]]

```rust
let x;
if cos(theta) > 0. {
    x = ceil(pos.x) as f32 - pos.x;
} else if cos(theta) < 0. {
    x = floor(pos.x) as f32 - pos.x;
} else {
    return f32::INFINITY;
}
let ini_y = Vec2 {
    x,
    y: x * tan(theta),
}
```

Compute `deltaY`, the constant displacement to get to the next y-aligned line.

![[Lines walking sketch#^area=Jmtd1YwO]]

```rust
let delta_y = Vec2 {
    // -1 or 1, depending of the sign of cos(theta)
    x: large_step(cos(theta), 0.),
    y: large_step(cos(theta), 0.) * tan(theta),
}
```

Now add `deltaY` to `pos` and `iniY` more and more (with a little more, let's call it $\Phi$).

```rust
let phi = Vec2 { x: 1., y: 0. }.rotated(theta) * 0.001;
for i in 0..MAX_ITERS {
    let v = pos + ini_y + delta_y * i as f32;
    if map.get_at((v + phi).floor()) {
        return v.norm();
    }
}
f32::INFINITY
```
