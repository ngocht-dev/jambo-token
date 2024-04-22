## Jambo Bank Dapp
- Supports creating bank account
- Supports deleting bank account
- Supports intra-bank transfers
- Supports bank deposit and withdrawal

# State
### BankAccount
```javascript
interface BankAccount {
    id: Buffer,
    balance: number, // 2 decimal places eg: 1.02 * 100 = 102
}
```

# API
### initialize
Build a banking application.

accounts:
- vaultAuthority
**vault account owner.**
- vault
**vault account**
- owner
**owner account**
- payer
**gas fee**
- mint
**token address**

Create two accounts, the owner of the vault account and the vault account. The cost of creating two accounts is paid by the payer.
```javascript
    // token address
    const mint = new web3.PublicKey(
      "CMonjg2DBfeS1MwL1KvSjt5o9CQtQLvDozg6hVrkCjJ6"
    );

    // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // vault authority
    let [vaultAuthority] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault_authority"), walletKeypair.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );

    // vault
    let [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault"), mint.toBuffer()],
      pg.PROGRAM_ID
    );

    const txHash = await pg.program.methods
      .initialize()
      .accounts({
        vaultAuthority,
        vault,
        owner: walletKeypair.publicKey,
        payer: pg.wallet.publicKey, // gas fee
        mint,
      })
      .signers([walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);
```

### createAccount
When an App user registers, this method is called to create an on-chain bank account.

args:
- id
**the unique identifier of the bank account.**

accounts:
- bankAccount
- owner
- payer

Create a bank account, The cost of creating an account is paid by the payer.
```javascript
    // id
    const userID = Buffer.from("user1");

    // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // bank account
    let [bankAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), userID],
      pg.PROGRAM_ID
    );

    const txHash = await pg.program.methods
      .createAccount(userID)
      .accounts({
        bankAccount,
        owner: walletKeypair.publicKey,
        payer: pg.wallet.publicKey,
      })
      .signers([walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account balance
    const account = await pg.program.account.bankAccount.fetch(bankAccount);

    console.log("On-chain balance is:", account.balance);
```

### removeAccount
When the App user deletes his or her account, this method is called to destroy the account data on the chain.

args:
- id
**the unique identifier of the bank account.**

accounts:
- bankAccount
- owner
- payer

Destroy a bank account, The fee will be returned to the payer.
```javascript
    // id
    const userID = Buffer.from("user1");

     // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // bank account
    let [bankAccount] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), userID],
      pg.PROGRAM_ID
    );

    const txHash = await pg.program.methods
      .removeAccount(userID)
      .accounts({
        bankAccount,
        owner: walletKeypair.publicKey,
        payer: pg.wallet.publicKey,
      })
      .signers([walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);
```

### transferBalance
We will have some built-in bank accounts, Jambo Points is one of them.

For internal bank transfers:
- scene one:
Users use their balance to purchase Jambo Points.
Transfer money from the user's bank account to the Jambo Points bank account.
- scene two:
Users exchange Jambo Points for balance.
Jambo Points bank account transfers money to the user's bank account.
- Transfer money between users

args:
- tid
**source**
- fid
**target**
- amount
**use 2 decimal places for the amount eg: 1.02*100=102**

accounts:
- fBankAccount
- tBankAccount
- owner

```javascript
    // id
    const user1ID = Buffer.from("user1");
    const user2ID = Buffer.from("user2");

    // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // bank account
    let [bankAccount1] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), user1ID],
      pg.PROGRAM_ID
    );

    let [bankAccount2] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), user2ID],
      pg.PROGRAM_ID
    );

    const txHash = await pg.program.methods
      .transferBalance(user1ID, user2ID, 10222) // 102.22
      .accounts({
        fBankAccount: bankAccount1,
        tBankAccount: bankAccount2,
        owner: walletKeypair.publicKey,
      })
      .signers([walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account balance
    const account = await pg.program.account.bankAccount.fetch(bankAccount2);

    console.log("On-chain balance is:", account.balance);
```

### deposit
Users can use jambo token to recharge their bank account balance.
Users can only recharge by importing their wallet.

The user's wallet account transfers jambo token to the vault account, and then updates the user's bank account balance.

args:
- id
**the unique identifier of the bank account.**
- amount
**use 2 decimal places for the amount eg: 1.02*100=102**

accounts:
- vault
**vault account**
- walletAuthority
**the user's wallet account needs to be signed**
- wallet
**wallet account**
- bankAccount
**bank account**
- owner
**owner account**
- mint
**token address**

```javascript
    // id
    const user1ID = Buffer.from("user1");

    // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // token address
    const mint = new web3.PublicKey(
      "CMonjg2DBfeS1MwL1KvSjt5o9CQtQLvDozg6hVrkCjJ6"
    );

    // bank account
    let [bankAccount1] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), user1ID],
      pg.PROGRAM_ID
    );

    // vault authority
    let [vaultAuthority] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault_authority"), walletKeypair.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );

    // vault
    let [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault"), mint.toBuffer()],
      pg.PROGRAM_ID
    );

    // wallet signed
    const userKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // wallet account
    const wallet = await getOrCreateAssociatedTokenAccount(
      pg.connection,
      pg.wallet.keypair,
      mint,
      userKeypair.publicKey
    );

    const txHash = await pg.program.methods
      .deposit(user1ID, 10256) // 102.56
      .accounts({
        vaultAuthority,
        vault,
        wallet: wallet.address,
        walletAuthority: userKeypair.publicKey,
        bankAccount: bankAccount1,
        owner: walletKeypair.publicKey,
        mint,
      })
      .signers([userKeypair, walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account balance
    const account = await pg.program.account.bankAccount.fetch(bankAccount1);

    console.log("On-chain balance is:", account.balance);
```

### withdraw
The balance of the user's bank account can be withdrawn to the Jambo token in the wallet account.

args:
- id
**the unique identifier of the bank account.**
- amount
**use 2 decimal places for the amount eg: 1.02*100=102**

accounts:
- vaultAuthority
**vault account owner**
- vault
**vault account**
- wallet
**wallet account**
- bankAccount
**bank account**
- owner
**owner account**
- mint
**token address**

```javascript
    // id
    const user1ID = Buffer.from("user1");

    // owner signed
    const walletKeypair = web3.Keypair.fromSecretKey(
      new Uint8Array([
        53, 37, 70, 221, 163, 39, 172, 62, 250, 130, 190, 149, 170, 158, 4, 51,
        94, 37, 219, 96, 192, 153, 16, 44, 185, 106, 26, 140, 184, 114, 29, 172,
        127, 76, 10, 154, 83, 69, 245, 11, 204, 138, 15, 73, 204, 111, 135, 46,
        220, 110, 164, 35, 89, 95, 18, 22, 185, 197, 22, 127, 33, 22, 146, 111,
      ])
    );

    // token address
    const mint = new web3.PublicKey(
      "CMonjg2DBfeS1MwL1KvSjt5o9CQtQLvDozg6hVrkCjJ6"
    );

    // bank account
    let [bankAccount1] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_bank_account"), walletKeypair.publicKey.toBuffer(), user1ID],
      pg.PROGRAM_ID
    );

    // vault authority
    let [vaultAuthority] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault_authority"), walletKeypair.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );

    // vault
    let [vault] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("jambo_vault"), mint.toBuffer()],
      pg.PROGRAM_ID
    );

    // wallet account
    const wallet = await getOrCreateAssociatedTokenAccount(
      pg.connection,
      pg.wallet.keypair,
      mint,
      userKeypair.publicKey
    );

    const txHash = await pg.program.methods
      .withdraw(user1ID, 10256)
      .accounts({
        vaultAuthority,
        vault,
        wallet: wallet.address,
        bankAccount: bankAccount1,
        owner: walletKeypair.publicKey,
        mint,
      })
      .signers([walletKeypair])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch the created account balance
    const account = await pg.program.account.bankAccount.fetch(bankAccount1);

    console.log("On-chain balance is:", account.balance);
```