## Jambo Solana Token Creator
- Supports command line building of tokens.
- Supports configuration metadata.

# Ready

### First, we need to prepare two wallet accounts:
- payer: a wallet account to pay for gasfee.
- owner: a wallet account is used to manage tokens and has ownership of the tokens.

### Next, we need to set the keypair file paths of the two wallet accounts into the tool.
- edit the **LOCAL_PAYER_JSON_ABSPATH** and **LOCAL_OWNER_JSON_ABSPATH** environment variables in the **.env** file in the project.

for example:

    # absolute path for a local keypair file
    LOCAL_PAYER_JSON_ABSPATH=/home/<username>/.config/solana/payer.json

    # absolute path for a local keypair file
    LOCAL_OWNER_JSON_ABSPATH=/home/<username>/.config/solana/owner.json

- if you want to create tokens more easily, simply place the two keypair files into the **.local_keys** directory of the project.
*Note: The file naming convention must be set to **payer.json** and **owner.json***

### Finally, we need to configure metadata information.

```json
{
    "name": "Jambo Token",
    "symbol": "JAMBO",
    "description": "Jambo Solana SPL token :)",       // Jambo Token introduction
    "image": "<IPFS>"                                              // Jambo Token logo
}
```

After uploading this json file to IPFS, enter the **IPFS_URL** into the uri.

```javascript
{
  name: "Jambo Token",
  symbol: "JAMBO",
  uri: "<IPFS_URL>",
  decimals: 8,                     // Fixed to 8
  supply: 100000000,        // 100M JAMBO
}
```

### Clusters and Endpoints
Solana platform provides **devnet** **testnet** **mainnet-beta** environment. The network is rate limits.

We need to use custom RPC cluster and endpoint. [click to apply](https://www.quicknode.com "click to apply")
- edit the **RPC_URL** environment variables in the **.env** file in the project.

for example:

    # RPC endpoint
    RPC_URL=https://skilled-misty-uranium.solana-devnet.quiknode.pro/80e3c29726a005c52217e71463a38f4057b4b2fd/

Great, job done. Let's start creating.


# Start

### Requires Node environment
command line: `node -v`

If you don’t have a node environment, [click to download](https://nodejs.org/en "click to download").

### Execute script
Enter the **jambo-spl** project:

1. Install dependency packages.

```
npm install
```

2. Create tokens.

```
npm run deploy ./scripts/create-token-with-metadata.ts
```

3. Console output.

```
Solana Token Creator

===============================================
===============================================

Payer address: 9Zv2gfE5kLivXe5uh2ycswUUNW3RsQwbwsDrgTX55jbt
Owner address: 93vCRYyyoxWajz6M58dpNEXm1bQRCvwZSFtnuZeZ9vrv
Token address: GsYv3XJPQ7bQWhcsnL7kKeTbYHVekFDzQ4bG8UXiHMSv

===============================================
===============================================

Transaction completed.
https://explorer.solana.com/tx/89MH2opzEiusKnBKPEinh7L9L62UAP87gUkkWEunwZK4S6pvVdpa1uSSMqfGKw6oTxyB35DgN2RgZNVVptUHbib?cluster=devnet

```