# Transactor Service

Submits signed stock price data to the Stellar network oracle contract.

## Overview

The transactor service is responsible for:
- Receiving signed price data from the signer
- Submitting transactions to the Stellar oracle smart contract
- Managing transaction retries and error handling
- Monitoring on-chain state and contract events

## Getting Started

### Prerequisites

- Node.js >= 18
- npm >= 9

### Installation

Install dependencies:

```bash
npm install
```

### Environment Setup

Copy the example environment file and configure it:

```bash
cp .env.example .env
```

Edit `.env` with your Stellar network configuration and credentials.

### Running the Service

#### Development Mode

```bash
npm run start:dev
```

The service will start on `http://localhost:3003` (or the port specified in `.env`).

#### Production Mode

First, build the application:

```bash
npm run build
```

Then start the service:

```bash
npm start
```

### Testing

Run tests:

```bash
npm test
```

Run tests in watch mode:

```bash
npm run test:watch
```

Run tests with coverage:

```bash
npm run test:cov
```

### Linting

Check code style:

```bash
npm run lint
```

## Project Structure

```
apps/transactor/
├── src/
│   ├── main.ts          # Application entry point
│   ├── app.module.ts    # Root module
│   └── app.service.ts   # Main service
├── .env.example         # Example environment variables
├── nest-cli.json        # NestJS CLI configuration
├── package.json         # Dependencies and scripts
├── tsconfig.json        # TypeScript configuration
└── README.md           # This file
```

## Status

🚧 Under construction - Blockchain transaction logic will be implemented in subsequent issues.
