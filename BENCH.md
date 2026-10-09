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

## Rust port

The Rust port was measured on the `rust-port` branch, with the release binary. Hyperfine
2.0 runs a command without a shell, so `--env` sets `CHECK`.

| Measurement | Command | Rust port | Baseline (bash) |
| --- | --- | --- | --- |
| Checker on one file | `hyperfine -i --warmup 3 'target/release/doc-style-check tests/fail/idiom.md'` | 5.4 ms ± 0.7 ms | 38.2 ms ± 5.7 ms |
| Test suite | `hyperfine --env "CHECK=$PWD/target/release/doc-style-check" --warmup 1 --runs 5 'tests/run'` | 2.227 s ± 0.007 s | 3.903 s ± 0.019 s |
| Processors | `nproc` | 32 | 32 |

- The Rust checker takes 7.1 times less time than the bash checker on one file.
- The test suite takes 1.75 times less time with the Rust checker than with the bash checker.
- The processor count is the same in both measurements, so the ratio is 1.

The Rust checker takes 1.1 ms on an empty file and 4.5 ms on a file of one line. The
difference is the compilation of the seven rule expressions. The compilation runs at the
first line of a file. The test suite runs the checker about 70 times. These runs take
about 0.4 s of the 2.227 s. The rest is bats-core and the test scripts.

## Mutation testing

`cargo mutants --jobs 32` ran on the Rust port in a systemd scope with a memory limit of
16 GB. The conformance test runs the bats suite for each mutant.

| Item | Value |
| --- | --- |
| Mutants | 268 |
| Caught | 184 |
| Missed | 67 |
| Unviable | 10 |
| Timeouts | 7 |
| Wall-clock time | 110.8 s |

For comparison, `tests/mutate` on the bash checker ran 194 mutants with 32 jobs in 60.2 s,
and the test suite detected all of them.
