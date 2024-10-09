# Useful Tools

## Taylor's Theorem

[Taylor's Theorem](https://en.wikipedia.org/wiki/Taylor%27s_theorem) seems to give an accurate polynomial approximation of a function based on it's derivatives.
Though only work on a fixed domain.

# Root functions

## Inverse Square Root

### Quake's fast inverse square root

[Binary manipulation](https://en.wikipedia.org/wiki/Fast_inverse_square_root) that does the job.

```rust
/// Quake's Fast Inverse Square Root
fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}
```

## Square Root

### From the Inverse Sqrt

Just $1 / \sqrt{x}$.

```rust
/// sqrt derived from fast inv sqrt
fn sqrt(x: f32) -> f32 {
    1. / inv_sqrt(x)
}
```

# Trigonometric Functions

## Cosine

### Polynomial Approximation + mirroring

Due to it's periodicity, the $cos()$ function can just be defined on $[0, \pi / 2]$ and then mirrored to extend the domain to $[-\infty, +\infty]$.

This polynomial can be fitted with mathematica.
Here is a 5th degree one.

```functionplot
---
title: Cosine and it's approximation
xLabel: x
yLabel: polynom_cos(x) & cos(x) & err(x)
bounds: [-0.5, 2.07, -1.5, 1.5]
disableZoom: false
grid: true
---
f(x)=1.00002-0.000447247*x-0.497081*x^2-0.00767104*x^3+0.0512404*x^4-0.00575391*x^5
c(x)=cos(x)
e(x)=abs(1.00002-0.000447247*x-0.497081*x^2-0.00767104*x^3+0.0512404*x^4-0.00575391*x^5 - cos(x))
```

```rust
/// 5th degree polynomial interpolation of the first quarter (0 to pi/2) of the cosine function
fn quarter_cos(x: f32) -> f32 {
    1.00002 - 0.000447247 * x - 0.497081 * x * x - 0.00767104 * x * x * x
        + 0.0512404 * x * x * x * x
        - 0.00575391 * x * x * x * x * x
    }
```

And the complete function, with the mirroring.

```rust
/// cosine function (approx)
fn cos(x: f32) -> f32 {
    // transformations
    let mut x = ((x % f32::consts::TAU) + f32::consts::TAU) % f32::consts::TAU; // bc rust modulo can be neg ):<
    if x > f32::consts::PI {
        x = f32::consts::TAU - x;
    }
    let mut multiplier = 1.;
    if x > f32::consts::FRAC_PI_2 {
        multiplier = -1.;
        x = f32::consts::PI - x;
    }
    //sample
    quarter_cos(x) * multiplier
}
```

## Sine

### Using the Cosine Approximation

$sin(x) = cos(x - \pi/2)$ so we can easily define the sine from the already defined cosine.

```rust
 /// sine function (approx)
fn sin(x: f32) -> f32 {
    cos(x - f32::consts::FRAC_PI_2)
}
```

## Tangent

### From the Sine and c=Cosine

We know that $tan(x) = \frac{sin(x)}{cos(x)}$.

```rust
/// tangent function (approx)
fn tan(x: f32) -> f32 {
    sin(x) / cos(x)
}
```

## Inverse Tangent

### From the Sine and Cosine

We can define the inverse tangent to avoid doing some calculations.

```rust
/// inverse tangent (approx)
fn inv_tan(x: f32) -> f32 {
    cos(x) / sin(x)
}
```

## Arc Tangent

The opposite function of the tangent, so that $tan(atan(x)) = x$.

### Using 2 hyperbolas

This ugly function tries to emulate the arc tangent function.

$$
atan(x) =
\begin{cases} 
	\frac{-1}{0.636x + \frac{2}{\pi}} + \frac{\pi}{2} & x \ge 0 \\
	\frac{-1}{0.636x - \frac{2}{\pi}} - \frac{\pi}{2} & x \lt 0
\end{cases}
$$

```functionplot
---
title: Arc Tangent and it's approximation
xLabel: x
yLabel: atan_approx(x) & atan(x) & err(x)
bounds: [-4, 4, -1.5, 1.5]
disableZoom: false
grid: true
---
f(x)= x < 0 ? -1/(0.636x-2/3.1415926)-(3.1415926/2) : -1/(0.636x+2/3.1415926)+(3.1415926/2)
c(x)=atan(x)
e(x)=abs((x < 0 ? -1/(0.636x-2/3.1415926)-(3.1415926/2) : -1/(0.636x+2/3.1415926)+(3.1415926/2)) - atan(x))
```

```rust
// https://www.desmos.com/calculator/ymtgipxmdg
/// atan function  (very approx)
fn atan(x: f32) -> f32 {
    if x >= 0. {
        -1. / (0.636 * x + f32::consts::FRAC_2_PI) + f32::consts::FRAC_PI_2
    } else {
        -1. / (0.636 * x - f32::consts::FRAC_2_PI) - f32::consts::FRAC_PI_2
    }
}
```

# Other Useful Functions

## Step Function

Same as in glsl : $0$ if less than threshold, $1$ otherwise.

```functionplot
---
title: Step function (threshold=0)
xLabel: x
yLabel: step(x, )
bounds: [-2, 2, -0.5, 1.5]
disableZoom: true
grid: true
---
f(x)= x < 0 ? 0 : 1
```

```rust
/// step function (range [0, 1])
fn step(x: f32, threshold: f32) -> f32 {
    if x < threshold {
        0.
    } else {
        1.
    }
}
```

## Large Step Function

A bit different than the Step Function : $-1$ if less than threshold, $1$ otherwise.

```functionplot
---
title: Large Step function (threshold=0)
xLabel: x
yLabel: large_step(x, 0)
bounds: [-2, 2, -1.5, 1.5]
disableZoom: true
grid: true
---
f(x)= x < 0 ? -1 : 1
```

```rust
/// step function (range [-1, 1])
fn large_step(x: f32, threshold: f32) -> f32 {
    if x < threshold {
        -1.
    } else {
        1.
    }
}
```
