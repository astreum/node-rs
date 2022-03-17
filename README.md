```

.       *    .               *     .    *          *
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

```

This is the Official Node for the Astreuos Blockchain.

## Features

- wallet creation
- wallet recovery
- view all and single address accounts
- create and send transactions
- send cancel transaction message
- view all and single address stakes
- blockchain validation

## Help

```

Commands:

    Wallet ...................................................................................................

    wallet new [password]                                                          create a new wallet
    wallet key                                                                     view encrypted key
    wallet address                                                                 view address
    wallet recover [encrypted key] [password]                                      recover your wallet

    Accounts ...................................................................................................

    accounts all                                                                   view all accounts
    accounts one [address]                                                         view one account

    Transaction ...............................................................................................

    tx new [password] [chain] [recipient] [amount] [solar limit] [solar price]     create & send a transaction
    tx cancel [password] [tx hash]                                                 send cancel tx message

    Nova .......................................................................................................

    nova stakes                                                                    view all stakes
    nova stake [address]                                                           view address stake
    nova validate [password] [chain]                                               validate the blockchain

```

2022-03-17
