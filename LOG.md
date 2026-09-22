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

---

## Sessin 7 - 19.09.2026

**Book progress:** Ray Tracing: The Next Week - finished the book (Chapters 5 - 10)

**What I did:**

- Added Perlin noise, turbulence, and the marble texture.
- Implemented Quad and built the Cornell Box.
- Added DiffuseLight and a background color on the camera.
- Added Translate and RotateY for instances.
- Added ConstantMedium and Isotropic for smoke, rendered cornell_smoke.
- Rendered the final scene of the book.
- Replaced println! per pixel with a BufWriter on stdout.
- Added render timing and a progress counter on stderr.

**Things I ran into / fixed:**

- Perlin noise came out dark and veiny instead of the book's camo pattern. I was using `Vec3::random()` for the gradient vectors, which only gives the positive octant. Book uses random(-1,1). All vectors pointing the same way kills the noise.
- Cornell Box was much darker and noisier than the book's. `Quad::hit` was passing a zero normal into HitRecord. With a zero normal, Lambertian scatters over the whole sphere instead of the hemisphere, so half the rays go through the wall and hit the black background.
- After that I changed `HitRecord::new` to take the ray and work out `front_face` itself. Now callers can't get it wrong.
- Boxes disappeared after `RotateY` but still cast shadows. Wrong sign on x when rotating the ray origin into object space. Direction had the right sign, origin didn't, so the ray was not a valid ray anymore.
- Also had `(i - 1)` instead of `(1 - i)` in the `RotateY` bbox loop. Doesn't show in this scene because every box starts at the origin, but it would break anything else.
- Broke Perlin again when I switched the permutation tables to fixed arrays. Arrays are `Copy`, so `permute(perm)` shuffled a copy and dropped it. Changed it to `&mut [usize]`.

**Performance work:**

- Replaced HitRecord's owned Material with a &'a Material borrow. Most of those clones were redundant because of HittableList::hit when a closer hit was found.
- Final scene rendered in 2270.6s on the first attempt. To be measured again...

**Notes to self:**

- Using `map` when dealing with `Option`s makes life easier.
- Final scene was rendered in 2270.6s in the first attempt, will be measured again after performance improvements.
- `map` only works when there is a single exit. `ConstantMedium::hit` has four separate early returns, and trying to express it with nested map calls goes nowhere. Luckily, Rust has `?` operator: returning `None` if it is none or unwrapping the value.
- `let Some(rec) = ... else { return ... };` is another useful thing.
- Lifetimes are not only for structs that look like parsers. Lifetime elision covers almost every call site. `fn hit(&self, ...) -> Option<HitRecord<'_>>` needs writing, and `&HitRecord in Material::scatter` needs no change.
- `split_at_mut` lets a recursive tree builder borrow both halves of a slice at once, which removes the need to pass start and end indices around like the C++ code does.

---

## Session 8 - 20.09.2026

**Profiling and performance**

**What I did:**

- Tried `cargo flamegraph` first. Doesn't work on macOS without Xcode, `xctrace` is missing with only Command Line Tools installed. Switched to `samply`, which needs nothing extra.
- Profiled `final_scene(200, 100, 10)` and looked at the inverted call stack.
- Replaced `powf(5.0)` in `Dielectric::reflectance` with plain multiplication.
- Made UV calculation lazy. Added `needs_uv()` on `Texture` and `Material`, so `Sphere::hit` only calls `get_sphere_uv` when something actually reads u and v.
- BVH now tries the near child first, based on the ray's direction along the split axis. Store `axis` on the node for that.
- Dropped `Ray` from `Sphere`. Now it holds `center` and `center_vec`, and `center_at` is just `center + time * center_vec`. Removed the `is_moving` flag too, the multiply is cheaper than the branch.
- Tried `Box<Quad>` in the `Object` enum to shrink it from 112 to 80 bytes. Measured worse and reverted.
- Tried `with_min_len` on the rayon iterator. No difference.

**What the profile said:**

- 80% self time in `Object::hit`. Everything is inlined into it: `ray_color`, `Sphere::hit`, `Quad::hit`, `Aabb::hit`, `BVHNode::hit`. Can't see inside, but at least LTO is doing its job.
- 11% in `libsystem_m`, all called from `Object::hit`. That's `powf`, `acos` and `atan2`.
- 6% rayon.

**Things I ran into / fixed:**

- Found a real bug while rewriting `reflectance`. The book does `r0 = r0*r0;` and then uses the squared value on the next line. I had `(r0 * r0) + (1.0 - r0) * ...`, so the second `r0` was not squared. Wrong Fresnel term on every glass surface.
- `with_min_len` doesn't exist for `u64` ranges. Rayon only implements the indexed iterator for `usize`, `i32` and friends.
- Deadlocked myself while adding debug prints. `println!` inside the rayon closure wants the stdout lock, but the main thread was already holding it through a `BufWriter`. Every worker blocked, main waited on `collect()`, nothing moved. Now stdout is only locked after the render is done.
- The progress counter was also wrong for a while: `fetch_max` called twice, and `<=` instead of `<`, so it printed on every pixel and stderr contention ate the render.

**Measurements** (`final_scene(400, 400, 20)`, M1 Pro):

- 13.1s wall, 111s user time.
- `Box<Quad>` version: same 13.1s wall but 118s user. Slower, reverted.
- CPU sits around 810%. Looks low for 10 cores but the M1 Pro has 8 performance and 2 efficiency cores, so the performance cores are full.

**Notes to self:**

- Guessing at optimizations is a waste of time. `powf` and the UV work were both reasonable guesses and neither showed up in the timings. Learning to read a profiler is probably the most useful thing I got out of this project.
- Wall time hides things. Both enum versions finished in 13.1s, but user time said one of them burned 7% more CPU. Look at both.
- Don't hold a stdout lock while parallel code runs.
- Never use `println!` for debugging in a parallel render. `eprintln!` only, and even that will drown the render if it's in the inner loop.

---