# IC-Kit Playground

This is a canister playground for experimenting with the IC. It provides a base template for any single canister project using the ic-kit (`0.5.0`)

## Getting started

1) Initialize dependencies

```
make init
```

2) Deploy the canister locally

> This will automatically start the local replica and deploy, or redeploy the canister on the already running replica.

```
make local
```

## Developing in the repository

You can manually generate the candid by running

```
make candid
```

> The kit automatically generates a rust test `save_candid` for each canister which will generate a candid file to the path specified. The make script runs these cargo tests to save the candid. 

You can also clean your environment, which will stop dfx and clean cargo, using

```
make clean
```
