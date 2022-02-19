# Astreuos Node

## Usage

```

Usage:
    rust-astreuos [command] [arguments]

```

## Header

```

    *      .       *    .               *     .    *          *
     .        .           *    .     *  .            .
         *   .      *           *               * .       *    .   .
     .                     *    .    * .            .         .   .   .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.   .vvv.    .vvv.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v   v.  .v
    .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v   v.   .vv.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v.      v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.    .vvv.   .vvv.   .v.

    Astreuos Node

    version 0.2.0

```

## Help

```

Commands:

    Wallet ..........................................................................................

    wt create [password]                                            generates your private key
    wt private                                                      view encrypted private key
    wt public                                                       view public key
    wt recover [encrypted private key] [password]                   recover a wallet

    Transactions ....................................................................................

    tx new [receipient] [amount] [solar price] [solar limit]        create, sign & send tx message
    tx cancel [tx hash]                                             send cancel tx message

    Nova ............................................................................................

    nv add [amount]                                                 add to stake balance
    nv stake                                                        check stake balance

```

2022-02-19
