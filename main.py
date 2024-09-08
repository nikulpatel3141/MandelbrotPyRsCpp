import sys
from typing import Literal
from time import time

parameters = {
    'x_min': -2.0,
    'x_max': 2.0,
    'y_min': -2.0,
    'y_max': 2.0,
    'n_steps': 10000,
    'max_iter': 100,
}
n_runs = 10


def time_language(language: Literal['rust', 'cpp']) -> [list[int], float]:
    match language:
        case 'rust':
            from mandelbrot_rs import generate_mandelbrot
        case 'cpp':
            from mandelbrot_cpp import generate_mandelbrot
        case _:
            raise ValueError(
                f'Invalid language {language}, must be one of "rust", "cpp"'
            )

    start = time()
    for _ in range(n_runs):
        result = generate_mandelbrot(*list(parameters.values()))
    avg_time = (time() - start) / n_runs
    return result, avg_time


if __name__ == "__main__":
    assert (language := sys.argv[1]) in ['rust', 'cpp']
    result, avg_time = time_language(language)

    print(f"Language {language} took: {avg_time:.2f}s avg over {n_runs} runs")
