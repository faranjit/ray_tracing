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

## Session 3 - 13.09.2026

**Book progress:** Chapter 10.5, "A Scene with Metal Spheres"

**What I did:**

- Added anti-aliasing.
- Created materials(Metal & Lambertian) and reflection calculated.
- Implemented the `Material` traits.
- Rendered a PPM with 3 spheres at 512x288 resolution.

---

## Session 4 - 16.09.2026

**Book progress:** Finished the book! (Chapters 11 - 14)

**What I did:**

- Implemented `Dielectric` materials to simulate glass and water, complete with refraction, Snell's law, and Schlick's approximation for total internal reflection.
- Overhauled the `Camera` to support arbitrary positioning (`look_from`, `look_at`, `vup`) and adjustable Field of View (FOV).
- Added defocus blur (depth of field) by simulating a physical lens with a defocus disk (`defocus_angle` and `focus_dist`).
- Refactored `CameraConfig` initialization to use the Builder pattern, eliminating a massive telescoping constructor and keeping `main.rs` extremely clean.
- Generated the final cover image featuring a generated scene of hundreds of random spheres composed of Lambertian, Metal, and Dielectric materials.

**Things I ran into / fixed:**

- As expected, render times hit a massive wall with the final random scene. The $O(N)$ intersection checks for every ray against hundreds of spheres brought the single-threaded CPU calculation to a crawl.
- Decided to hold off on introducing external crates (like `rayon` for parallelization) or advanced algorithms (like BVH) to keep the codebase strictly aligned with the end-state of the first book.

---

## Session 5 - 17.09.2026

**Post-book Optimizations: Performance & Multithreading**

**What I did:**

- **Devirtualization:** Refactored `Material` and `Hittable` (`Object`) from dynamic trait objects (`Arc<dyn Trait>`) to static enums.
- **Factory Methods:** Implemented factory functions (e.g., `Material::lambertian`, `Object::sphere`) to keep the scene instantiation clean and hide the enum boilerplate.
- **Multithreading:** Integrated the `rayon` crate and parallelized the pixel rendering loop using `into_par_iter()`.
- **Result:** Smashed the render time of the massive final scene (1200x800, 500 samples) from a single-threaded crawl to under 2 minutes.

**Things I ran into / fixed:**

- Got a compiler error trying to use `Arc<dyn Object>` after switching to enums; fixed by removing the `dyn` keyword since enums use static dispatch.
- To prevent thread racing on stdout during parallel rendering, split the render pipeline into two stages: computing all pixels into a `Vec<Color>` concurrently, and then writing them to the PPM format sequentially.

---

## Session 6 - 17.09.2026

**Book progress:** Ray Tracing: The Next Week - Chapter 4.6, "Rendering The Image Texture"

**What I did:**

- Created a new Git branch named `the-next-week` to keep the second book's changes separate.
- Added a texture mapping system with `SolidColor`, `Checker`, and `ImageTexture`.
- Used a Rust enum for the `Texture` type to maintain static dispatch and keep performance high.
- Replaced the C++ `stb_image` library with the Rust `image` crate to load image files.
- Added the `get_sphere_uv` function to calculate texture coordinates for 3D spheres.
- Rendered a sphere with an Earth texture map.

**Things I ran into / fixed:**

- Fixed a bug in the bounding box (AABB) calculation for moving spheres. The box for the second time step was calculating from the wrong center. This fixed the visual clipping and made the motion blur look correct.
- Reading pixels using the `image` crate's `get_pixel()` method was slowing down the render. I optimized this by converting the image into a flat 1D byte array (`Vec<u8>`) on load. Reading pixels directly using `(y * width + x) * 3` improved the speed significantly.
