# DEX Wallet - Decentralized Token Exchange on Solana

A decentralized token exchange (DEX) wallet built on the **Solana blockchain** using the **Anchor framework** (v0.32+). Swap tokens, add/remove liquidity, and manage your assets through a modern web interface.

## Features

- **Token Swap**: Swap between Token A and Token B using constant product (x*y=k) AMM
- **Liquidity Pools**: Add and remove liquidity to earn LP tokens
- **Configurable Fees**: Pool creators can set swap fees (up to 10%)
- **Wallet Integration**: Connect Phantom and other Solana wallets
- **Slippage Protection**: Set minimum output amount for swaps

## Project Structure

```
dex_wallet/
├── programs/
│   └── dex_wallet/          # Anchor program (Rust)
│       └── src/
│           ├── lib.rs       # Program entry point
│           ├── instructions/ # Swap, add/remove liquidity
│           ├── state/       # Pool account structure
│           └── error.rs
├── app/                     # React frontend
│   └── src/
│       ├── components/      # Swap UI, Wallet balance
│       └── App.tsx
├── tests/                   # TypeScript tests
└── Anchor.toml
```

## Prerequisites

- **Rust** (1.75+)
- **Solana CLI** (2.x)
- **Anchor CLI** (0.32+)
- **Node.js** (18+)
- **Yarn** or **npm**

### Installation (Windows)

1. Install Rust: https://rustup.rs/
2. Install Solana: https://docs.solana.com/cli/install-solana-cli-tools
3. Install Anchor: `cargo install --git https://github.com/coral-xyz/anchor avm --locked && avm install 0.32.1 && avm use 0.32.1`
4. Or use WSL for the official install script

## Quick Start

### Program ID / keypair

This repo includes a generated program keypair at `target/deploy/dex_wallet-keypair.json`, and the program id is set to:

- `EKUTuQVm6JVNc7LwtuMjpCyRFBkQQgDFpsBMzXavfk1D`

### 1. Build the Program

```bash
cd dex_wallet
anchor build
```

### 2. Run Tests

```bash
anchor test
```

### 3. Deploy to Devnet

```bash
# Configure for devnet in Anchor.toml
# [provider]
# cluster = "Devnet"

anchor deploy
```

### 4. Run the Frontend

```bash
cd app
npm install
npm run dev
```

## Program Instructions

| Instruction       | Description                          |
|-------------------|--------------------------------------|
| `initialize_pool` | Create a new liquidity pool          |
| `add_liquidity`   | Deposit tokens and receive LP tokens |
| `remove_liquidity`| Burn LP tokens and withdraw          |
| `swap_a_for_b`    | Swap Token A for Token B             |
| `swap_b_for_a`    | Swap Token B for Token A             |

## Environment Variables

Create `.env` in the `app` folder:

```
VITE_SOLANA_RPC=https://api.devnet.solana.com
```

## Security Notes

- This is a **demo/educational** project. Audit before mainnet use.
- Use devnet for testing. Get devnet SOL from [faucet.solana.com](https://faucet.solana.com)

## License

MIT
