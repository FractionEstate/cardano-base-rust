# Changelog for `cardano-slotting`

## Unreleased

* Add `Debug` stub for `EpochInfo` matching the Haskell `Show` instance.
* Cover `fixed_epoch_info` and `unsafe_linear_extend_epoch_info` with unit tests.

## 0.2.0.2

*

## 0.2.0.1

* Drop GHC <= 9.4 support

## 0.2.0.0

* Add `EpochInterval` and `addEpochInterval` from `cardano-ledger`.
* Add `binOpEpochNo` helper function to facilitate binary operations on
  `EpochNo`.
* Remove numeric instances (`Num`, `Integral`, `Real`) of `EpochNo` and
  `EpochSize` for safety.
  They are still available for testing from the `testlib` as orphans.
* New `Test.Cardano.Slotting.TreeDiff` module extracted from
  `cardano-ledger-binary`. It lives in a new public sublibrary `testlib`.

### `testlib`

* Add numeric instances (`Num`, `Integral`, `Real`) of `EpochNo` and
  `EpochSize` as orphans.

## 0.1.1.1

* GHC-9.6 compatibility

## 0.1.1.0

* Remove `development` flag: #372
* Addition of `ToJSON`/`FromJSON` instances for:
  * `WithOrigin`
  * `BlockNo`
  * `SystemStart`
  * `RelativeTime` and `SlotLength`

## 0.1.0.1

* Initial release
