# Scripts and Reference Implementations of Poseidon and Starkad
This repository contains the source code of reference implementations for various versions of Poseidon [1]. The source code is available in Sage. Moreover, scripts to calculate the round numbers, the round constants, and the MDS matrices are also included.

### Update from 02/05/2023
The script `generate_params_poseidon.sage` should be used to compute the round numbers and to generate the round constants and matrices. The scripts `calc_round_numbers.py` and `generate_parameters_grain.sage` are deprecated and should not be used anymore.

### Update from 07/03/2021
We fixed several bugs in the implementation. First, the linear layer was computed as `state = state * M` instead of `state = M * state`, and secondly the final matrix multiplication was missing. The test vectors were also changed accordingly.

<br>

[1] *Poseidon: A New Hash Function for Zero-Knowledge Proof Systems*. Cryptology ePrint Archive, Report 2019/458. [https://eprint.iacr.org/2019/458](https://eprint.iacr.org/2019/458). Accepted at USENIX'21.
