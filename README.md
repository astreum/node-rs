
    *      .       *    .               *     .    *          *
     .        .           *    .     *  .            .
         *   .      *           *               * .       *    .   .
     .                     *    .    * .            .         .   .   .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
    .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

    Rust Astreuos

    version 1.0.0

This is the Official Node for the Astreuos Blockchain.

## Features

- Key Management
- Transactions
- Blockchain Validation

## Usage


## Help

```

Commands:

    Wallet .............................................................................................................

    wt create [password] [repeat password]                                           generates your key and address
    wt key                                                                           view encrypted key
    wt address                                                                       view address
    wt recover [encrypted key] [password]                                            recover your wallet

    Transactions ........................................................................................................

    tx new [password] [network] [receipient] [amount] [solar limit] [solar price]    create, sign & broadcast tx message
    tx cancel [password] [tx hash]                                                   send cancel tx message

    Nova ................................................................................................................

    nv add [amount]                                                                  add to stake balance
    nv stake                                                                         check stake balance
    nv validate [password] [network]                                                 create new blocks
  
```

2022-02-21
