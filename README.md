# HFSP — Human Friendly String Parser

_Use Human Friendly String Parser or Have Fun Staying Poor!_

## Why

For **global bitcoin/LN adoption** we need to make use of bitcoin applications
easy for non-technical people.

We want bitcoin applications to give useful feedback to the user even when an
application does not directly support what the user is about to do.
(E.g. scan an on-chain bitcoin address with a lightning wallet.)

## How

Bitcoin applications should give helpful feedback to the users.

## What

The aim of this project is to craft **a well structured list of different address
formats** (on-chain, ln invoice, lnurl, …).
Such that wallets implementations can uses it and give wallets' users helpful
hints when scanning QR code or pasting address/request.

## Rust Library

A rust library which implements parsing and validation.
Bitcoin applications developers can easily start using the library and improve
their users experience.
