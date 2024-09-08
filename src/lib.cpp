#include <pybind11/pybind11.h>
#include <pybind11/stl.h>

namespace py = pybind11;

#include <algorithm>
#include <complex>
#include <vector>

int calc_escape_iter(const std::complex<double> &c, const int &max_iter) {
  std::complex<double> z = 0;
  int iter = 0;

  while ((std::pow(c.imag(), 2) + std::pow(c.real(), 2)) < 4 &&
         ++iter < max_iter) {
    z = std::pow(z, 2) + c;
  }
  return iter;
}

std::vector<int> generate_mandelbrot(double x_min, double x_max, double y_min,
                                       double y_max, int n_steps,
                                       int max_iter) {

  auto result = std::vector<int>(pow(n_steps, 2));

  auto _map_iter = [&](auto i, auto a, auto b) {
    return a + i * (b - a) / n_steps;
  };

  auto i = 0;
  for (auto x = 0; x < n_steps; ++x) {
    for (auto y = 0; y < n_steps; ++y) {
      auto z = std::complex<double>(_map_iter(x, x_min, x_max),
                                    _map_iter(x, y_min, y_max));
      result[i++] = calc_escape_iter(z, max_iter);
    }
  }
  return result;
}

PYBIND11_MODULE(mandelbrot_cpp, m) {
    m.doc() = "pybind11 example plugin"; // optional module docstring

    m.def("generate_mandelbrot", &generate_mandelbrot, "A function that adds two numbers");
}
