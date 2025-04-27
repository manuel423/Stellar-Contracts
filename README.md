# Stellar Smart Contracts

This project contains two smart contracts designed for the Stellar blockchain:

1. **Contribution Weight Token (CWT) Contract**: For minting, transferring, and burning tokens
2. **Monetization Distribution Token (MDT) Contract**: For distributing tokens based on CWT balances

## Project Structure

- **cwt_contract/**: Contains the source code for the Contribution Weight Token smart contract.
- **mdt_contract/**: Contains the source code for the Monetization Distribution Token smart contract.

## Prerequisites

- Rust (latest stable version)
- Soroban CLI (v22.0.7 or later)
- WebAssembly target for Rust

### Install Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install soroban-cli
```

## Build the Contracts

```bash
# Build CWT contract
cd cwt_contract
soroban contract build
cd ..

# Build MDT contract
cd mdt_contract
soroban contract build
cd ..
```

After building, the WASM files will be located at:
- `cwt_contract/target/wasm32-unknown-unknown/release/cwt_contract.wasm`
- `mdt_contract/target/wasm32-unknown-unknown/release/mdt_contract.wasm`

## Local Testing

### 1. Start Local Network and Create Test Accounts

```bash
# Start a local Soroban network
soroban network start

# Create test accounts
soroban keys generate sac
soroban keys generate user1
soroban keys generate user2

# Fund test accounts
soroban keys fund sac
soroban keys fund user1
soroban keys fund user2
```

### 2. Deploy Contracts

```bash
# Deploy CWT contract
CWT_ID=$(soroban contract deploy \
  --wasm cwt_contract/target/wasm32-unknown-unknown/release/cwt_contract.wasm \
  --source-account sac \
  --network local)
echo "CWT Contract ID: $CWT_ID"

# Deploy MDT contract
MDT_ID=$(soroban contract deploy \
  --wasm mdt_contract/target/wasm32-unknown-unknown/release/mdt_contract.wasm \
  --source-account sac \
  --network local)
echo "MDT Contract ID: $MDT_ID"
```

### 3. Initialize Contracts

```bash
# Get account addresses
SAC_ADDRESS=$(soroban keys address sac)
USER1_ADDRESS=$(soroban keys address user1)
USER2_ADDRESS=$(soroban keys address user2)

# Initialize CWT contract
soroban contract invoke --id $CWT_ID --source-account sac --network local -- initialize --sac $SAC_ADDRESS

# Initialize MDT contract
soroban contract invoke --id $MDT_ID --source-account sac --network local -- initialize --sac $SAC_ADDRESS --cwt_address $CWT_ID
```

### 4. Test CWT Contract

```bash
# Mint CWT tokens to users
soroban contract invoke --id $CWT_ID --source-account sac --network local -- mint --to $USER1_ADDRESS --amount 100
soroban contract invoke --id $CWT_ID --source-account sac --network local -- mint --to $USER2_ADDRESS --amount 50

# Check balances
soroban contract invoke --id $CWT_ID --source-account user1 --network local -- bal_of --address $USER1_ADDRESS
soroban contract invoke --id $CWT_ID --source-account user2 --network local -- bal_of --address $USER2_ADDRESS

# Check total supply
soroban contract invoke --id $CWT_ID --source-account user1 --network local -- tot_sup

# Test transfers
soroban contract invoke --id $CWT_ID --source-account user1 --network local -- transfer --from $USER1_ADDRESS --to $USER2_ADDRESS --amount 10

# Check updated balances
soroban contract invoke --id $CWT_ID --source-account user1 --network local -- bal_of --address $USER1_ADDRESS
soroban contract invoke --id $CWT_ID --source-account user2 --network local -- bal_of --address $USER2_ADDRESS
```

### 5. Test MDT Contract

```bash
# Distribute MDT tokens based on CWT holdings
soroban contract invoke --id $MDT_ID --source-account sac --network local -- distribute --amount 1000 --holders '[ "'$USER1_ADDRESS'", "'$USER2_ADDRESS'" ]'

# Check MDT balances
soroban contract invoke --id $MDT_ID --source-account user1 --network local -- balance_of --address $USER1_ADDRESS
soroban contract invoke --id $MDT_ID --source-account user2 --network local -- balance_of --address $USER2_ADDRESS

# Check MDT total supply
soroban contract invoke --id $MDT_ID --source-account user1 --network local -- total_supply

# Test MDT burning (transfer to contract)
soroban contract invoke --id $MDT_ID --source-account user2 --network local -- transfer --from $USER2_ADDRESS --to $MDT_ID --amount 50

# Check updated balances and supply
soroban contract invoke --id $MDT_ID --source-account user2 --network local -- balance_of --address $USER2_ADDRESS
soroban contract invoke --id $MDT_ID --source-account user1 --network local -- total_supply
```

### 6. Clean Up Local Environment

```bash
# Stop the local network when done
soroban network stop
```

## Production Deployment

### 1. Configure Network

```bash
# Set up for testnet (for testing before mainnet)
soroban network add --global testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"

# Set up for mainnet (for production)
soroban network add --global mainnet --rpc-url https://soroban.stellar.org:443 --network-passphrase "Public Global Stellar Network ; September 2015"

# Select network for deployment
soroban network use testnet  # or mainnet
```

### 2. Prepare Production Account

```bash
# Generate and fund a deployment account (for testnet)
soroban keys generate deployer
soroban keys fund deployer --network testnet

# For mainnet, you need to transfer XLM from an exchange or existing wallet
# Then import it (example with secret key)
soroban keys add admin-mainnet SCYOURPRIVATEKEY
```

### 3. Build for Production

```bash
# Build with optimization for production
cd cwt_contract
soroban contract build --profile release
cd ..

cd mdt_contract
soroban contract build --profile release
cd ..
```

### 4. Deploy to Production

```bash
# Deploy CWT contract (example shown for testnet)
CWT_PROD_ID=$(soroban contract deploy \
  --wasm cwt_contract/target/wasm32-unknown-unknown/release/cwt_contract.wasm \
  --source-account deployer \
  --network testnet)
echo "Production CWT Contract ID: $CWT_PROD_ID"

# Deploy MDT contract
MDT_PROD_ID=$(soroban contract deploy \
  --wasm mdt_contract/target/wasm32-unknown-unknown/release/mdt_contract.wasm \
  --source-account deployer \
  --network testnet)
echo "Production MDT Contract ID: $MDT_PROD_ID"

# IMPORTANT: Save these contract IDs securely!
```

### 5. Initialize Production Contracts

```bash
# Get your admin address
ADMIN_ADDRESS=$(soroban keys address admin)

# Initialize CWT contract 
soroban contract invoke --id $CWT_PROD_ID --source-account admin --network testnet -- initialize --sac $ADMIN_ADDRESS

# Initialize MDT contract
soroban contract invoke --id $MDT_PROD_ID --source-account admin --network testnet -- initialize --sac $ADMIN_ADDRESS --cwt_address $CWT_PROD_ID
```

## Contract Interaction Reference

### CWT Contract Functions

1. **Initialize**: Set up the contract with the SAC address
   ```bash
   soroban contract invoke --id $CWT_ID --source-account sac --network local -- initialize --sac $SAC_ADDRESS
   ```

2. **Mint**: Create and send tokens to a user (SAC only)
   ```bash
   soroban contract invoke --id $CWT_ID --source-account sac --network local -- mint --to $USER_ADDRESS --amount 100
   ```

3. **Transfer**: Send tokens between accounts
   ```bash
   soroban contract invoke --id $CWT_ID --source-account user1 --network local -- transfer --from $USER1_ADDRESS --to $USER2_ADDRESS --amount 10
   ```

4. **Burn**: Send tokens to contract address
   ```bash
   soroban contract invoke --id $CWT_ID --source-account user1 --network local -- transfer --from $USER1_ADDRESS --to $CWT_ID --amount 20
   ```

5. **Check Balance**: View token balance
   ```bash
   soroban contract invoke --id $CWT_ID --source-account user1 --network local -- bal_of --address $USER1_ADDRESS
   ```

6. **Check Total Supply**: View total tokens in circulation
   ```bash
   soroban contract invoke --id $CWT_ID --source-account user1 --network local -- tot_sup
   ```

### MDT Contract Functions

1. **Initialize**: Set up contract with SAC and CWT addresses
   ```bash
   soroban contract invoke --id $MDT_ID --source-account sac --network local -- initialize --sac $SAC_ADDRESS --cwt_address $CWT_ID
   ```

2. **Distribute**: Calculate and distribute tokens based on CWT holdings (SAC only)
   ```bash
   soroban contract invoke --id $MDT_ID --source-account sac --network local -- distribute --amount 1000 --holders '[ "'$USER1_ADDRESS'", "'$USER2_ADDRESS'" ]'
   ```

3. **Transfer**: Send tokens between accounts
   ```bash
   soroban contract invoke --id $MDT_ID --source-account user1 --network local -- transfer --from $USER1_ADDRESS --to $USER2_ADDRESS --amount 50
   ```

4. **Burn**: Send tokens to contract address
   ```bash
   soroban contract invoke --id $MDT_ID --source-account user2 --network local -- transfer --from $USER2_ADDRESS --to $MDT_ID --amount 50
   ```

5. **Check Balance**: View token balance
   ```bash
   soroban contract invoke --id $MDT_ID --source-account user1 --network local -- balance_of --address $USER1_ADDRESS
   ```

6. **Check Total Supply**: View total tokens in circulation
   ```bash
   soroban contract invoke --id $MDT_ID --source-account user1 --network local -- total_supply
   ``` 