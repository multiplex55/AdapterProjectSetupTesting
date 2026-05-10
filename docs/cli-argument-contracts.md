# App CLI argument contract and startup diagnostics

All app binaries keep startup logic thin: select a `ProfileId`, build `StartupConfig`, then call `runtime::startup`.

## Argument contract

### `target5-app`
- Profile: `target5-real`.
- Supported flags: none currently.
- Wiring intent: target5 + ethernet providers only.

### `target10-app`
- Profile: `target10-real`.
- Supported flags: none currently.
- Wiring intent: target10 + ethernet + commtype1 + commtype2 providers.

### `windows-target5-sim`
- Profile: `windows-target5-sim`.
- Supported flags:
  - `--input replay`
  - `--input manual`
  - `--input ethernet`
- Default mode when omitted: `manual`.
- Wiring intent: windows-sim target5 + ethernet transport provider.

### `windows-target10-sim`
- Profile: `windows-target10-sim`.
- Supported flags:
  - `--input replay`
  - `--input manual`
  - `--input ethernet`
- Default mode when omitted: `manual`.
- Wiring intent: windows-sim target10 + ethernet + simulated commtype1/commtype2 providers.

## Startup diagnostics examples

```text
profile=target5-real startup_ok selected=[(Compute, "adapter://target5"), (Transport, "adapter://ethernet"), (Clock, "builtin://optimized/Clock")]
```

```text
profile=windows-target10-sim input=replay startup_ok selected=[(Compute, "adapter://windows-sim-target10"), (Transport, "adapter://ethernet+sim-commtype1+sim-commtype2?input=replay"), (Clock, "builtin://optimized/Clock")]
```
