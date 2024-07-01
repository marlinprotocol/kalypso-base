# kalypso-executable-base

The Repo contains template/base code to create`prover-gateway`, `prover-executable` and `input-verification-executable` required by Kalypso

## 1. Build `prover-gateway`
Required for provers that do not operate inside enclave
Edit `prover/main.rs`
Build *prover-gateway*
```
cargo build --release --bin prover
```

Copy *prover*
```
cp ./target/x86_64-unknown-linux-musl/release/prover prover-gateway
```

## 2. Build `prover-executable`
Required for provers that operate inside enclave
Edit `prover/main.rs`
Build *prover-gateway*
```
cargo build --release --bin prover --features confidential
```
Copy *prover*
```
cp ./target/x86_64-unknown-linux-musl/release/prover prover-executable
```

## 3. Build `input-verification-executable`
Required for provers that do not operate proof generation inside enclave. This executable only verifies input and generates attestations for invalid inputs.
Edit `ivs/main.rs`
Build *input-verification-service*
```
cargo build --release --bin ivs --features confidential
```

Copy *ivs*
```
cp ./target/x86_64-unknown-linux-musl/release/ivs input-verification-executable
```

## 4. Build simplest `prover-executable`
This is the simplest and most recommnended prover setup. This contains all the end points which makes enclave work as both Prover and IVS
Edit `ivs/main.rs`
Edit `prover/main.rs`
Build *prover-executable*
```
cargo build --release --bin prover-executable --features confidential
```