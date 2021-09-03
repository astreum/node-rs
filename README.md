# Astreuos Node

## Usage

```

Usage:
    rust-astreuos [command] [argument]

```

## Startup
```

*      .       *    .               *     .    *          *
    .        .           *    .     *  .            .
        *   .      *           *               * .       *    .   .
    .                     *    .    * .            .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
    .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

Astreuos Node

version 0.1.0

```

## Help
```

Commands:
    create wallet                 Generates a seed phrase and master key
    recover wallet                Recover a wallet through a seed phrase
    remove wallet                 Remove master key (Recover through seed phrase)
    show wallet                   View wallet information
    
    accounts                      View all accounts
    new account                   Create a new account
    show account [account]        View account information
        
    new address [account]         Get a new address for a transaction
    show address [address]        View address information

    new transaction [account]     Craft, sign and send a new transaction
    show transaction [tx_hash]    View transaction information

    sync                          Get the latest blocks and transform the astreuos state
    mint                          Validate the blockchain by minting new blocks

```
