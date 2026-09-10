# Ray Tracer - Work Log

A running log of progress on this project. Written after each work session - mainly for myself, to track what was done, what was learned, and what's next.

---

## Session 1 - 10.09.2026

**Book progress:** Ray Tracing in One Weekend - Chapter 4, "Sending Rays Into the Scene"

**What I did:**
- Set up the Rust project with `cargo new`
- Implemented `Vec3` (vec3.rs) with operator overloading: `Add`, `AddAssign`, `Sub`, `Mul` (both `Vec3*Vec3` and `Vec3*f64`), `MulAssign`, `Div`, `DivAssign`, `Neg`, `Index`/`IndexMut`
- Added `dot`, `cross`, `len`, `len_squared`, and a free `unit_vector` function
- Implemented `Ray` (ray.rs) with `origin()`, `direction()`, and `at(t)`
- Implemented `Color` as a type alias for `Vec3`, with `write_color` (color.rs)
- Set up the camera/viewport in main.rs: aspect ratio, image dimensions, viewport size, pixel delta vectors, `pixel00_loc`
- Rendered the first gradient sky image (blue to white, based on ray direction's y component) - output matches the book's expected image
- Pushed the project to a private GitHub repo (github.com/faranjit/ray_tracing)

**Things I ran into / fixed:**
- Rust's `struct` needed `#[derive(Clone, Copy)]` on `Vec3` to avoid move errors when reusing vectors in expressions
- C++'s `Mul<Vec3> for u64`/`Div<Vec3> for u64` don't exist by default in Rust - had to add explicit trait impls to multiply/divide a `Vec3` by an integer (pixel indices `i`, `j` are `u64`)
- Caught an integer-division bug before it caused a problem: `image_width / image_height` as `u64` truncates before the `f64` cast - fixed by casting both operands to `f64` first

---

## Session 2 - 10.09.2026

**Book progress:** Chapter 6.7, "Common Constants and Utility Functions"

**What I did:**
- Created the `Hittable` trait and `HitRecord` struct.
- Converted the C++ mutable reference pattern (`out parameters`) to Rust's `Option<HitRecord>`. 
- Implemented the `Sphere` struct.
- Added `HittableList` to manage multiple objects in the scene using `Vec<Box<dyn Hittable>>`.
- Added common math constants (infinity, pi) and utilities.

**Things I ran into / fixed:**

**Notes to self:**
- Using `Option` instead of mutating a passed reference makes the Rust implementation much cleaner than the C++ original.
- Chose `Arc<dyn Hittable>` for the object list for now as later multi-threading might be needed.

---