# Astreuos Terminal

## Usage

```

Usage:
    astreuos-terminal [command] [argument]

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

    Astreuos Terminal

    version 0.1.0

```

## Help
```

Commands:

    Wallet ...........................................................................

    create wallet                 generates a seed phrase and master key
    recover wallet                recover a wallet through a seed phrase
    remove wallet                 remove master key
    show wallet                   view wallet information
    
    Accounts .........................................................................

    accounts                      view all accounts
    new account                   create a new account
    show account [account]        view account information

    Address ..........................................................................
        
    new address [account]         get a new address for a transaction
    show address [address]        view address information

    Transactions .....................................................................

    new transaction [account]     craft, sign and send a new transaction
    show transaction [tx_hash]    view transaction information
    cancel transaction [tx_hash]  remove a transaction from the tx pool

    Blockchain .......................................................................

    sync                          get the latest blocks
    mint                          validate the blockchain by minting new blocks

    Nova .............................................................................

    stake                         add quanta to the treasury and start minting
    withdraw                      remove quanta from the treasury

    pools                         view staking pools

    Nebula ...........................................................................

    get                           get an object
    store                         store an object
    
    serve                         start file server

```

2021-11-03
