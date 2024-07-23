# Cryptography-Research
![CI Status](https://github.com/supragya/research/actions/workflows/ci.yaml/badge.svg)

A toy project that intends to implement "in-code" a bunch of stuff. This includes but is not limited to:
- Groups, Finite Fields, Prime Fields, Binary Towers etc.
- Frontend compiler for zero-knowledge backends
- Components of Zero Knowledge systems: R1CS, PLONK, FRI etc.
- Algorithms such as Shamir's Secret Sharing, Verifiable Secret Sharing, Shnorr Discrete Log Proof of Knowledge

## Browsing the codebase
The codebase is mostly split into different directories depending on the specific ideas that they tackle. While mostly consistent with this scheme, there may be deviations. Download the codebase locally and run as follows:
```sh
git clone git@github.com:supragya/Cryptography-Research.git
cd Cryptography-Research && cargo test
```

## References
- **\[Sha97\]**: [Shamir's secret sharing](https://apps.dtic.mil/sti/pdfs/ADA069397.pdf)
- **\[Fel87\]**: [Feldman's verifiable secret sharing](https://www.zkdocs.com/docs/zkdocs/protocol-primitives/verifiable-secret-sharing/)
- **\[Sch91\]**: [Schnorr's DLog PoK](https://www.zkdocs.com/docs/zkdocs/zero-knowledge-protocols/schnorr/)
- **\[rfc1321\]**: [Message Digest v5 hashing function](https://datatracker.ietf.org/doc/html/rfc1321)
