# surepet-rs

Unofficial CLI for the [surepet API](https://www.surepetcare.io/).

> This is an experimental project to play with rust.
> It probably lacks a ton of feature that are not planned to be implemented.

## Usage

In order to authenticate to the API, please set the two following environment variables:

- `SUREPET_EMAIL`: email of your surepet account
- `SUREPET_PASSWORD`: password of your surepet account

### List devices

```shell
> surepet-rs devices
✅ Hub is online
✅ Buanderie is online (battery: 49.69%)
```

### List pets

```shell
> surepet-rs pets
🏡 Arlene is outside since 1days 5h 50m 12s
🏠 Garfield is inside since 1h 27m 8s
```
