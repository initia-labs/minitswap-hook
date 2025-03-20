# minitswap-hook

minitswap hook contract

# Publish Guide

## Wasmvm

### 1. Move directory to wasm

```bash
cd wasm
```

### 2. Build contract

```bash
cargo build --target wasm32-unknown-unknown --release
```

### 3. Publish contract without admin

```typescript
import { MsgStoreCode, MsgInstantiateContract } from '@initia/initia.js';
import * as fs from 'fs';

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
    undefined, // Contract should not upgradable
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
import { MsgExecuteMessages, MsgUpdateACL } from '@initia/initia.js';
async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // key must be admin key
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

### 1. Move directory to move

```bash
cd move
```

### 2. Update module address of Move.toml

Highly recommand to use new address, to make sure miniswap is the only module of the address

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

### 3. Build module

```bash
initiad move build
```

### 4. Publish module with immutable option

```typescript
import { MsgPublish } from '@initia/initia.js';
import * as fs from 'fs';

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
import { MsgExecuteMessages, MsgUpdateACL } from '@initia/initia.js';
async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // key must be admin key
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

## Evm

### 1. Move directory to evm

```bash
cd evm
```

### 2. Build contract

```bash
solcjs ./MinitswapHook.sol --bin
```

### 3. Publish contract

```typescript
import { MsgCreate } from '@initia/initia.js';
import * as fs from 'fs';

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
import { MsgExecuteMessages, MsgUpdateACL } from '@initia/initia.js';
async function updateACL() {
  const msg = new MsgExecuteMessages(key.accAddress, [
    // key must be admin key
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
