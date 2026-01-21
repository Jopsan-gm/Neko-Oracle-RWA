# Frontend Demo

UI for visualizing stock price feeds and oracle status.

## Overview

The frontend application provides:
- Real-time visualization of stock price feeds
- Oracle health and status dashboard
- Historical price charts
- Contract interaction interface

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

Edit `.env` with your API configuration if needed.

### Running the Application

#### Development Mode

```bash
npm run dev
```

The application will start on `http://localhost:3000`.

Open [http://localhost:3000](http://localhost:3000) in your browser to see the result.

#### Production Mode

First, build the application:

```bash
npm run build
```

Then start the production server:

```bash
npm start
```

### Linting

Check code style:

```bash
npm run lint
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

## Project Structure

```
apps/frontend/
├── app/
│   ├── layout.tsx      # Root layout
│   ├── page.tsx        # Home page
│   └── globals.css     # Global styles
├── .env.example        # Example environment variables
├── next.config.js      # Next.js configuration
├── package.json        # Dependencies and scripts
├── tsconfig.json       # TypeScript configuration
└── README.md          # This file
```

## Status

🚧 Under construction - Features will be implemented in subsequent issues.
