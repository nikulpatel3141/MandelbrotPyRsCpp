use num::complex::Complex64;
use pyo3::prelude::*;

fn calc_escape_iter(c: Complex64, max_iter: &i32) -> i32 {
    let mut z = Complex64::new(0.0, 0.0);
    let mut iter = 0;
    while (z.norm_sqr() < 4.0) & (iter < *max_iter) {
        z = z.powi(2) + c;
        iter += 1;
    }
    iter
}

#[pyfunction]
fn generate_mandelbrot(
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    n_steps: i32,
    max_iter: i32,
) -> Vec<i32> {
    let _iter_value = |min, max, i| min + (i as f64) * (max - min) / (n_steps as f64);

    let re_sides = (0..n_steps).map(|i| _iter_value(x_min, x_max, i));
    let im_sides = (0..n_steps).map(|i| _iter_value(y_min, y_max, i));

    let to_map = im_sides.flat_map(|x| {
        std::iter::repeat(x.clone())
            .zip(re_sides.clone())
            .map(|a| Complex64::new(a.1, a.0))
    });
    to_map.map(|x| calc_escape_iter(x, &max_iter)).collect()
}

#[pymodule(name = "mandelbrot_rs")]
fn my_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_mandelbrot, m)?)?;
    Ok(())
}
