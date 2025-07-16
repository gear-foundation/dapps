# Example Frontend for Foundable Token  

This project demonstrates a minimal frontend for a Foundable Token (VFT) contract using [sails-js](https://wiki.gear.foundation/docs/sails-js/react-hooks) React hooks.

## Key Components

- **API Hooks:** Modularized contract logic, all reusable hooks are in `hooks/api.ts`.
- **libts:** Auto-generated contract interface description (TypeScript), generated using `sails-js-cli` (see below).
- **Home:** The main view component showcasing token operations, transfer, and balance queries.

## How to generate `lib.ts`

Generate TypeScript interface files for your contract using the CLI:

```bash
npm install -g sails-js-cli

sails-js generate path/to/sails.idl -o path/to/out/dir --no-project
```

More details: [SailsJS Client Generation Guide](https://wiki.gear.foundation/docs/sails-js/client-generation)

## Contract Standard

Full description of the Foundable Token Extended (VFT) contract standard and API:

- [VFT standard on wiki.gear.foundation](https://wiki.gear.foundation/docs/examples/Standards/vft)

## Environment Variables

Your `.env` file must include:

```
VITE_NODE_ADDRESS=wss://testnet.vara.network
VITE_CONTRACT=<your_vft_contract_address>
```

## How to run

```bash
yarn install
yarn start
```

---

All hooks, the view component, and the interface are fully modular for easy learning and extension.  
See `hooks/api.ts`, `lib.ts`, and the main `Home` component for usage patterns.