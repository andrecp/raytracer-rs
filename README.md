# Ray Tracing in one weekend book

At the core of a ray tracer is to send rays through pixels and compute what color is seen in the direction of those rays.

Calculate which ray goes from the eye to a pixel, compute what the ray intersects, and compute a color for that intersection point.

The camera is the "eye".

* A ray as a function `p(t) = A + t * B` where 
    * `p` is a 3d position along a line in 3D, stands for point at paremeter
    * `A` is the point of origin of the ray
    * `B` is the point of direction of the ray

## Generate docs

`cargo doc --open --document-private-items`

# TODO

* How to make Vec3 a generic trait with default implementations and have RGB and XYZ have its default?!
* Write tests.
* Add more rust docstrings.
