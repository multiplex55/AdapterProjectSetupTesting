# How to Implement Ethernet Between Apps

See also: [How to add ethernet message](./how-to-add-ethernet-message.md) · [How to add new port and adapter](./how-to-add-new-port-and-adapter.md)

Use this chain for new ethernet-backed app-to-app behavior:

`messages -> core algorithms -> core flows -> effects -> runtime -> ethernet adapter -> app wiring`

## 1) Define message contracts (`crates/messages`)

- Add request/response/event DTOs needed by both sides.
- Keep payloads transport-agnostic and serializable.
- Version/extend contracts deliberately; avoid per-adapter drift.

## 2) Implement pure business logic (`crates/core/src/algorithms`)

- Add deterministic transforms/validation for the message data.
- No network, timing, retries, or socket code.

## 3) Add orchestration flow (`crates/core/src/flows`)

- Create a flow that sequences algorithm + effect requests through ports.
- Return explicit typed flow errors for each failure stage.

## 4) Define effects through ports (`crates/ports`)

- Add/extend port traits for send/receive/query operations the flow needs.
- Keep ports intent-focused (business effect), not protocol-detail heavy.

## 5) Wire flow + ports in runtime (`crates/runtime`)

- Runtime instantiates flow dependencies and drives execution lifecycle.
- Map runtime events/commands to flow invocations and typed outcomes.

## 6) Implement concrete ethernet behavior (`adapters/ethernet`)

- Implement required port traits in the ethernet adapter.
- Keep protocol/session/socket handling local to adapter code.
- Surface adapter errors explicitly so runtime/flow can report them.

## 7) Compose in app entrypoints (`apps/*`)

- Select ethernet adapter implementation and inject into runtime wiring.
- Keep `main` composition-only: config + wiring + start.

## Validation checklist

- `core` has no dependency on `runtime`/`adapters`/`ffi`.
- DTOs shared across boundaries live in `crates/messages`.
- Flow errors are explicit; no silent fallbacks.
- App mains contain no domain logic.

## Optional/later

For offline simulation or deterministic playback over equivalent message streams, you can later add scenario/replay coverage via [Replay scenario format](./replay-scenario-format.md).
