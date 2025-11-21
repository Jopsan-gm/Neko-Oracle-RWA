# RWA Token Contract

This is a Fungible Token contract that wraps a Real-World Asset (RWA), creates a stability pool for it, and allows creating Collateralized Debt Positions (CDPs) against it.

The contract implements:
- **Fungible Token Interface**: Standard token operations (transfer, balance, allowance)
- **Collateralized Debt Positions (CDPs)**: Users can deposit collateral (XLM, USDC, USDT, etc.) and borrow RWA tokens
- **Stability Pool**: A pool where users can stake RWA tokens to earn rewards from liquidations
- **Interest Calculation**: Accrues interest on borrowed RWA tokens over time

## Key Components

### Storage

The contract stores:
- Token metadata (name, symbol, decimals)
- Oracle contract addresses (collateral oracle for collateral assets, RWA oracle for RWA token prices)
- Stability pool state (total RWA tokens, total collateral, product constants)
- CDP data for each user
- Fees and interest collected

### CDP (Collateralized Debt Position)

Users can:
- **Open a CDP**: Deposit collateral and borrow RWA tokens
- **Add collateral**: Increase the collateralization ratio
- **Withdraw collateral**: Remove collateral (must maintain minimum ratio)
- **Borrow more**: Mint additional RWA tokens against existing collateral
- **Repay debt**: Burn RWA tokens to reduce debt
- **Close CDP**: Repay all debt and withdraw all collateral

### Stability Pool

Users can:
- **Stake**: Deposit RWA tokens into the pool
- **Withdraw**: Remove RWA tokens from the pool
- **Claim rewards**: Receive collateral rewards from liquidations

## Oracle Integration

The contract uses two oracles:
- **Collateral Oracle** (Reflector Oracle): Provides prices for collateral assets (XLM, USDC, USDT, etc.)
- **RWA Oracle**: Provides prices for the RWA token being lent

## Interest Calculation

Interest is calculated based on:
- Annual interest rate (in basis points)
- Time elapsed since last interest accrual
- Amount of RWA tokens borrowed

## Liquidation

When a CDP's collateralization ratio falls below the minimum:
- The CDP can be liquidated by any user
- RWA tokens from the stability pool are used to repay debt
- Collateral is distributed to stability pool stakers as rewards
