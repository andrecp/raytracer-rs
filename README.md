# Ray Tracing in one weekend book

At the core of a ray tracer is to send rays through pixels and compute what color is seen in the direction of thise rays.

Calculate which ray goes from the eye to a pixel, compute what the ray intersects, and compute a color for that intersection point.

The camera is the "eye".

* https://en.wikipedia.org/wiki/Netpbm, will use the ppm format
* A ray as a function `p(t) = A + t * B` where 
    * `p` is a 3d position along a line in 3D, stands for point at paremeter
    * `A` is the ray of origin
    * `B` is the ray of direction
    

# TODO

* How to make Vec3 a generic trait with default implementations ?!
