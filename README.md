# Astreum Node

```
    *      .       *    .               *     .    *          *
    .  .        .           *    .     *  .            .
        *   .      *           *               * .       *    .   .
        .                     *    .    * .            .         .   .   .

     .vvv.    .vvv.  .vvvvv.  .vvvv.   .vvvv.  .v   v.  .v.     .v.
    .v   v.  .v         v     .v   v.  .v      .v   v.  .v v   v v.
    .vvvvv.   .vv.      v     .vvvv.   .vvv.   .v   v.  .v  v v  v.
    .v   v.      v.     v     .v  v.   .v      .v   v.  .v   v   v.
    .v   v.  .vvv.      v     .v   v.  .vvvv.   .vvv.   .v       v.  .v.
    
    Node v0.0.1
```

## About

This is the Official Node for the Astreum Blockchain.

## Install

### Dependencies

Rust

```
git clone

cargo install

```

## Usage

```
    account new                                                 create & store a new private key

    account view [chain] [address]                              shows account information from local & peers 

    transaction new [chain] [address] [recipient] [value]       create, sign & submit a transaction     

    block view [chain] [number]                                 shows block information from local & peers

    stake fund [chain] [address] [value]                        create, sign & submit stake funding transaction

    stake withdraw [chain] [address] [value]                    create, sign & submit stake withdrawl transaction

    stake view [chain] [address]                                shows staking information from local & peers 

    validate [chain] [address]                                  create, sign & submit blocks

    sync [chain]                                                get new blocks & update accounts
```

Copyright © Astreum Foundation Established 12023 HE
