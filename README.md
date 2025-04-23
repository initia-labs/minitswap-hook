# minitswap-hook

minitswap hook contract

# Publish Guide

## Wasmvm

### 1. Move to the `wasm` directory

```bash
cd wasm
```

### 2. Build the contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Publish the contract without an admin

```typescript
import {
  MsgStoreCode,
  MsgInstantiateContract,
  MnemonicKey,
  RESTClient,
  Wallet,
} from '@initia/initia.js';

import * as fs from 'fs';

const rest = new RESTClient('<REST-URI>');

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function storeCode() {
  const codeBytes = fs.readFileSync('<PATH_OF_CODEBYTES>').toString('base64'); // get .wasm file

  const msg = new MsgStoreCode(key.accAddress, codeBytes);

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}

async function instantiateContract() {
  const msg = new MsgInstantiateContract(
    key.accAddress,
    undefined, // Contract should not be upgradeable
    CODE_ID,
    'minitswap-hook',
    Buffer.from('{}').toString('base64'),
    [],
  );

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

### 4. Update ACL (Allow IBC Hook)

```typescript
import {
  MsgExecuteMessages,
  MsgUpdateACL,
  MnemonicKey,
  RESTClient,
  Wallet,
} from '@initia/initia.js';

const rest = new RESTClient('<REST-URI>');

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // The key must be an admin key
    new MsgUpdateACL(
      'init1gz9n8jnu9fgqw7vem9ud67gqjk5q4m2w0aejne',
      '<HOOK_MODULE_ADDR>',
      true,
    ),
  ]);

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

## Movevm

### 1. Move to the `move` directory

```bash
cd move
```

### 2. Update the module address in `Move.toml`

It is highly recommended to use a new address to ensure that minitswap is the only module for that address.

```toml
[package]
name = "minitswap-hook"
version = "0.0.1"

[addresses]
std = "0x1"
minitia_std = "0x1"
publisher = "<YOUR-MODULE-ADDR>"

[dependencies]
InitiaStdlib = { git = "https://github.com/initia-labs/move-natives.git", subdir = "initia_stdlib", rev = "0a6aa67b41087c56b6fe7ae54e75c0ecceb388a8" }
```

### 3. Build the module

```bash
initiad move build
```

### 4. Publish the module with the immutable option

```typescript
import { MsgPublish, MnemonicKey, RESTClient, Wallet } from '@initia/initia.js';
import * as fs from 'fs';

const rest = new RESTClient('<REST-URI>');

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function publishModule() {
  const codeBytes = fs.readFileSync('<PATH_OF_CODEBYTES>').toString('base64'); // get .mv file

  const msg = new MsgPublish(sender, codeBytes, 2 /* IMMUTABLE */);

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

### 5. Update ACL (Allow IBC Hook)

```typescript
import {
  MsgExecuteMessages,
  MsgUpdateACL,
  MnemonicKey,
  RESTClient,
  Wallet,
  AccAddress,
} from '@initia/initia.js';

const rest = new RESTClient('<REST-URI>');

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // The key must be an admin key
    new MsgUpdateACL(
      'init1gz9n8jnu9fgqw7vem9ud67gqjk5q4m2w0aejne',
      // Address must be bech32 address
      AccAddress.fromHex('<HOOK_MODULE_ADDR>'),
      true,
    ),
  ]);

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

## Evm

### 1. Move to the `evm` directory

```bash
cd evm
```

### 2. Build the contract

```bash
solcjs ./MinitswapHook.sol --bin
```

### 3. Publish the contract

```typescript
import {
  MsgCreate,
  MnemonicKey,
  RESTClient,
  Wallet,
  AccAddress,
} from '@initia/initia.js';
import * as fs from 'fs';

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function instantiateContract() {
  const msg = new MsgCreate(
    key.accAddress,
    '0x' + fs.readFileSync('<PATH_OF_CODEBYTES>').toString(),
    '',
    [],
  );

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

### 4. Update ACL (Allow IBC Hook)

```typescript
import {
  MsgExecuteMessages,
  MsgUpdateACL,
  MnemonicKey,
  RESTClient,
  Wallet,
  AccAddress,
} from '@initia/initia.js';

const rest = new RESTClient('<REST-URI>');

const key = new MnemonicKey({
  mnemonic: '<YOUR-MNEMONIC>',
  // If you do not change cointype when you create, use 118 and false
  coinType: 118,
  eth: false,
});

const wallet = new Wallet(rest, key);

async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // The key must be an admin key
    new MsgUpdateACL(
      'init1gz9n8jnu9fgqw7vem9ud67gqjk5q4m2w0aejne',
      // Address must be bech32 address
      AccAddress.fromHex('<HOOK_CONTRACT_ADDR>'),
      true,
    ),
  ]);

  const signedTx = await wallet.createAndSignTx({ msgs: [msg] });
  const broadcastRes = await rest.tx.broadcastSync(signedTx);
  console.log(broadcastRes);
}
```

## Update ACL with minitiad

### 1. Create Msg File

```json
{
  "messages": [
    {
      "@type": "/initia.ibchooks.v1.MsgUpdateACL",
      "authority": "init1gz9n8jnu9fgqw7vem9ud67gqjk5q4m2w0aejne",
      "address": "init120va2qy4f0u3l0y5hkjr7hzpgwhs76rnrqhz7g",
      "allowed": true
    }
  ]
}
```

### 2. Execute Msg

```bash
minitiad tx opchild execute-messages ./execute-allow-hook.json --from {admin-key} --chain-id {chain-id} --node {rpc-endpoint} --gas-adjustment 1.4 --gas auto
```
