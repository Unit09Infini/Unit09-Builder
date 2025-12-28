# Unit09 Program – Program ID Reference

This file defines the canonical on-chain addresses (Program IDs) used by the
Unit09 protocol across different Solana clusters.  
These values MUST remain stable once deployed, as changing a Program ID breaks
compatibility for clients, SDKs, dApps, and deployed instances that rely on it.

---

## 🎯 What is a Program ID?

A **Program ID** is the public key used to identify the compiled Solana BPF
program on-chain.  
All client interactions – instructions, account seeds, CPI calls, signature
validation – route through this address.

If you fork Unit09 or deploy your own environment, **you MUST replace these IDs**.

---

## 🧠 Unit09 Program ID (Default Layout)

> ⚠ Do NOT use these values for real deployments.  
> Replace them after you run `anchor build && anchor deploy`.

| Cluster | Program ID | Notes |
|---------|------------|-------|
| Local Validator (anchor test / localhost) | `UNIT9mB7Z2F8cUXa11111111111111111111111111` | Auto-generated example for local development |
| Devnet | `UNIT9DEV111111111111111111111111111111111` | Replace after devnet deployment |
| Testnet | `UNIT9TEST1111111111111111111111111111111` | Optional – only if Testnet is used |
| Mainnet-Beta | `UNIT9MAIN1111111111111111111111111111111` | Final immutable address once deployed |

---

## 🏗️ How to Replace These IDs

After compiling and deploying:

```bash
anchor build
anchor deploy
solana address  # prints deployed program address
