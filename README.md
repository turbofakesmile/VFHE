# Verifiable FHE

I used Zama's tfhe-rs library to write fully homomorphic encryption-backed code and ran it in Risc0 ZKVM to prove simple cryptographic operations. The code is not production-ready, you need to reconfigure it yourself for your specific research or points of observation. The idea came to me at the ZkHack Krakow hackathon, where I was trying to integrate the specs of the tfhe-rs library with the Risc0 stack and verify the receipt externally. The project was strongly supported by Risc0 engineers, who helped me execute this.



Check the docs (all tutorials on how to set up the environment and download Risc0 crates):

- https://www.risczero.com/get-started
- https://www.risczero.com/zkvm
- https://github.com/zama-ai/tfhe-rs

