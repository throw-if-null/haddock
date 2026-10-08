# Benchmarks

This file records the run time of the bash checker and of the Rust port. All numbers come
from one machine:

| Item | Value |
| --- | --- |
| CPU | AMD Ryzen 9 8940HX |
| OS | Fedora, Linux 7.2.9 |
| hyperfine | 2.0.0 |

Each time is the hyperfine mean and standard deviation.

## Baseline (bash)

The baseline was measured on the `main` checkout at commit `9177486`.

| Measurement | Command | Result |
| --- | --- | --- |
| Checker on one file | `hyperfine -i --warmup 3 'skills/doc-style/scripts/check tests/fail/idiom.md'` | 38.2 ms ± 5.7 ms |
| Test suite | `hyperfine --warmup 1 --runs 5 'tests/run'` | 3.903 s ± 0.019 s |
| Processors | `nproc` | 32 |

The checker exits 1 on `tests/fail/idiom.md`, because the fixture holds findings. Without
`-i`, hyperfine stops at the first warmup run.
