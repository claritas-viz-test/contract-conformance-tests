# claritas_flutter

Flutter presentation primitives for Claritas Viz clients.

## Scope

This package owns client-side Flutter presentation concerns only. It must not become a second authority for shared Claritas data models, RPC contracts, persistence schemas, or wire formats. Those contracts belong in the Claritas interface/contract layer and should be consumed here through generated or published client interfaces once that dependency is available.

The initial public API provides:

- `claritasFlutterContractVersion` — a stable presentation-package contract identifier;
- `ClaritasSurface` — a reusable semantic boundary around an embedded Claritas visualization.

`ClaritasSurface` deliberately preserves the caller's child widget instead of owning chart rendering, routing, transport, or data semantics. That keeps visualization engines and generated contract clients swappable behind one Flutter presentation boundary.

## Verification

CI uses a pinned Flutter SDK and immutable action revisions, then runs:

```text
flutter pub get
dart format --output=none --set-exit-if-changed lib test
flutter analyze --fatal-infos
flutter test
```
