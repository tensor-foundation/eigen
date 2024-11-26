## Eigen CLI

A CLI for interacting with Tensor Foundation Solana programs.

### Installation

From crates.io:

```
cargo install tensor-eigen
```

From source:

```
git clone https://github.com/tensor-foundation/eigen
cd eigen
cargo install --path .
```

Self update (versions >= 0.0.13):

```
eigen self update
```

### Usage

See available commands with:

```
eigen -h
```

Eigen picks up RPC url and keypair from the Solana config file (`~/.config/solana/cli/config.yml`) by default.
These values can be overridden with the `--rpc-url` and `--keypair-path` flags, where the former is the url string and the latter is the path to the keypair file.

#### Decode Accounts

Eigen supports decoding most types of Tensor Foundation accounts and automatically determines the type of account based on the data.


```
eigen decode <address>
```

Example, decoding an AMM Pool account:

```
% eigen decode C9TAnmPKKjskgrYep4obJYYyiytMFmW5bRgNuEYB5w3U
Pool---------------------
discriminator            : f19a6d0411b16dbc
version                  : 1
bump                     : 255
pool_id                  : �J�L�W�e\���F
~g��y��0[�
�:u
created_at               : 2024-11-22T11:17:26+00:00
updated_at               : 2024-11-22T11:17:26+00:00
expiry                   : 2025-11-22T11:17:26+00:00
owner                    : 9LgJfhhBgATPQEDDQLeZCXNvooXK5jHmD8pbJssTykL7
whitelist                : 9nzCneMrkJ91FM8b5y3j93dw5WQVvqwUAMXaLGfCDfLE
rent_payer               : 5EDuQNiCKKQX2miovFD8KZsH83qaiLgoy4wBLzX8Bsah
currency                 : SOL
amount                   : 0
price_offset             : 0
nfts_held                : 0
--PoolStats-------------
  taker_sell_count       : 0
  taker_buy_count        : 0
  accumulated_mm_profit  : 0
-------------------------
shared_escrow            : None
cosigner                 : None
maker_broker             : None
max_taker_sell_count     : 0
--PoolConfig--------------
  pool_type              : Trade
  curve_type             : Exponential
  starting_price         : 10000000
  delta                  : 500
  mm_compound_fees       : false
  mm_fee_bps             : None
-------------------------
reserved                 : [all zeros]
```

This account turns out to be a wallet:

```
% eigen decode 5EDuQNiCKKQX2miovFD8KZsH83qaiLgoy4wBLzX8Bsah
Wallet---------
lamports       : 268646234563
SOL            : 268.646234563
pda            : false
```

### Download Accounts

Same as decode but saves them to JSON files:

```
Usage: eigen download [OPTIONS] <ADDRESS> [OUTPUT_DIR]

Arguments:
  <ADDRESS>     Address to download
  [OUTPUT_DIR]  Output directory

Options:
  -r, --rpc-url <RPC_URL>  RPC URL for the Solana cluster
  -h, --help               Print help
```


### Lookup Errors

Eigen supports looking up Anchor and Tensor Foundation program errors by their code:

```
eigen error <error-code>
```

```
% eigen error 0xbbc
Anchor ErrorCode:
Error Code: 3004
Error Type: AccountDidNotSerialize
```

```
% eigen error 0x1770
TensorWhitelistError:
Error Code: 6000
Error Type: BadCosigner
```

```
% eigen error 0x2ee0
TensorAmmError:
Error Code: 12000
Error Type: WrongWhitelist
```


### Program Specific Commands

#### Pool

Create and edit pools:

```
Usage: eigen pool create [OPTIONS] <WHITELIST> <POOL_CONFIG_PATH>

Arguments:
  <WHITELIST>         Whitelist public key
  <POOL_CONFIG_PATH>  Path to the pool config file

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

```
Usage: eigen pool edit [OPTIONS] <POOL> <EDIT_POOL_CONFIG_PATH>

Arguments:
  <POOL>                   Pool public key
  <EDIT_POOL_CONFIG_PATH>  Path to the edit pool config file

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

#### Whitelist

Create and update whitelists:

```
Usage: eigen whitelist create [OPTIONS] <WHITELIST_CONFIG_PATH> [NAMESPACE_PATH]

Arguments:
  <WHITELIST_CONFIG_PATH>  Whitelist config path
  [NAMESPACE_PATH]         Namespace path

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

```
Usage: eigen whitelist update [OPTIONS] <WHITELIST_ADDRESS>

Arguments:
  <WHITELIST_ADDRESS>  Whitelist address

Options:
  -k, --keypair-path <KEYPAIR_PATH>                            Path to the keypair file
  -r, --rpc-url <RPC_URL>                                      RPC URL for the Solana cluster
  -c, --new-conditions-path <NEW_CONDITIONS_PATH>              New conditions path
  -u, --new-update-authority-path <NEW_UPDATE_AUTHORITY_PATH>  New update authority json file path
  -f, --new-freeze-authority <NEW_FREEZE_AUTHORITY>            New freeze authority
  -h, --help                                                   Print help
```

#### Fees

Get balances of all the Tensor Foundation fee shards:

```
eigen fees balances
```

Save all the shard addresses to `fee_shards.json` file:

```
eigen fees shards
```

Fund all the shards with minimum rent lamports on the current cluster, shards already funded are skipped:

```
eigen fees fund
```
