# Smart Contracts for Stellar

This project contains two smart contracts designed for the Stellar blockchain:

1. **Contribution Weight Token (CWT) Contract**
2. **Monetization Distribution Token (MDT) Contract**

## Project Structure

- **cwt_contract/**: Contains the source code for the Contribution Weight Token smart contract.
- **mdt_contract/**: Contains the source code for the Monetization Distribution Token smart contract.

## Requirements

- Rust (latest stable version)
- Soroban CLI for deploying and testing smart contracts

## Building the Contracts

### Prerequisites

1. **Install Rust**:
   - Visit [rustup.rs](https://rustup.rs/) and follow the instructions to install Rust.

2. **Install Soroban CLI**:
   - Run the following command to install the Soroban CLI:
     ```bash
     cargo install soroban-cli
     ```

3. **Install WebAssembly Target**:
   - Install the WebAssembly target for Rust:
     ```bash
     rustup target add wasm32-unknown-unknown
     ```

### Build the Contracts

1. **Build the CWT Contract**:
   - Navigate to the CWT contract directory:
     ```bash
     cd cwt_contract
     ```
   - Build the contract:
     ```bash
     cargo build --target wasm32-unknown-unknown --release
     ```

2. **Build the MDT Contract**:
   - Navigate to the MDT contract directory:
     ```bash
     cd mdt_contract
     ```
   - Build the contract:
     ```bash
     cargo build --target wasm32-unknown-unknown --release
     ```

## Deploying the Contracts

1. **Start a Local Network**:
   - Run the following command to start a local Soroban network:
     ```bash
     soroban network start
     ```

2. **Deploy the CWT Contract**:
   - Deploy the CWT contract:
     ```bash
     soroban contract deploy --wasm target/wasm32-unknown-unknown/release/cwt_contract.wasm
     ```
   - Note the contract ID that is returned.

3. **Deploy the MDT Contract**:
   - Deploy the MDT contract:
     ```bash
     soroban contract deploy --wasm target/wasm32-unknown-unknown/release/mdt_contract.wasm
     ```
   - Note the contract ID that is returned.

## Testing the Contracts

1. **Initialize the Contracts**:
   - Initialize the CWT contract with the SAC address:
     ```bash
     soroban contract invoke --id <CWT_CONTRACT_ID> --function initialize --arg <SAC_ADDRESS>
     ```
   - Initialize the MDT contract with the SAC address and CWT contract address:
     ```bash
     soroban contract invoke --id <MDT_CONTRACT_ID> --function initialize --arg <SAC_ADDRESS> --arg <CWT_CONTRACT_ADDRESS>
     ```

2. **Test Minting in CWT Contract**:
   - Invoke the `mint` function to mint tokens:
     ```bash
     soroban contract invoke --id <CWT_CONTRACT_ID> --function mint --arg <TO_ADDRESS> --arg <AMOUNT>
     ```

3. **Test Distribution in MDT Contract**:
   - Invoke the `distribute` function to distribute MDT tokens:
     ```bash
     soroban contract invoke --id <MDT_CONTRACT_ID> --function distribute --arg <AMOUNT> --arg <HOLDERS_LIST>
     ```

4. **Test Burning in Both Contracts**:
   - Invoke the `transfer` function to burn tokens:
     ```bash
     soroban contract invoke --id <CONTRACT_ID> --function transfer --arg <FROM_ADDRESS> --arg <TO_ADDRESS> --arg <AMOUNT>
     ```

## Monitoring Events

- Use the Soroban CLI to monitor events emitted by the contracts:
  ```bash
  soroban events
  ```

## Additional Notes

- Ensure that all dependencies are up to date and compatible with the `wasm32-unknown-unknown` target.
- If you encounter any issues, refer to the troubleshooting section in the README or seek help from the Rust community. 