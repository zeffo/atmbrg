# atmbrg

## Atomberg fan control CLI

**WIP**, currently supports only the LAN API.

For single fan setups like I have, you can have a wrapper like so:
```nushell
def fan [...args] {
  let dev = atmbrg scan-one
  atmbrg --device $dev ...$args
}
```

For multi-fan setups, an ideal workflow is still unclear to me.
If you have any ideas I would appreciate them!

## tested devices

1. Aris Contour
