use cardano_slotting::epoch_info::{
    epoch_info_first, epoch_info_range, epoch_info_size, epoch_info_slot_to_relative_time,
};
use cardano_slotting::epoch_info::{fixed::fixed_epoch_info, unsafe_linear_extend_epoch_info};
use cardano_slotting::slot::{
    EpochInterval, EpochNo, EpochSize, SlotNo, WithOrigin, add_epoch_interval, at, bin_op_epoch_no,
    origin,
};
use cardano_slotting::time::{
    diff_relative_time, slot_length_from_millisec, slot_length_to_millisec,
};
use time::Duration;

#[test]
fn with_origin_conversions() {
    assert_eq!(origin::<u64>(), WithOrigin::Origin);
    assert_eq!(at(3u64), WithOrigin::At(3));
    let maybe = cardano_slotting::slot::with_origin_to_maybe(at(5));
    assert_eq!(maybe, Some(5));
    let back = cardano_slotting::slot::with_origin_from_maybe::<u64>(None);
    assert_eq!(back, origin());
}

#[test]
fn bin_op_epoch_addition() {
    let a = EpochNo(10);
    let b = EpochNo(3);
    let result = bin_op_epoch_no(|x, y| x + y, a, b);
    assert_eq!(result, EpochNo(13));
}

#[test]
fn epoch_interval_adds() {
    let epoch = EpochNo(5);
    let interval = EpochInterval(7);
    assert_eq!(add_epoch_interval(epoch, interval), EpochNo(12));
}

#[test]
fn fixed_epoch_info_behaves_like_constant_schedule() {
    let epoch_size = EpochSize(20);
    let slot_length = slot_length_from_millisec(1000);
    let info = fixed_epoch_info(epoch_size, slot_length);

    assert_eq!(epoch_info_size(&info, EpochNo(0)).unwrap(), epoch_size);
    assert_eq!(epoch_info_first(&info, EpochNo(3)).unwrap(), SlotNo(60));

    let reftime = epoch_info_slot_to_relative_time(&info, SlotNo(5)).unwrap();
    assert_eq!(reftime.duration().whole_seconds(), 5);

    let range = epoch_info_range(&info, EpochNo(1)).unwrap();
    assert_eq!(range, (SlotNo(20), SlotNo(39)));
}

#[test]
fn linear_extension_matches_underlying_schedule() {
    let epoch_size = EpochSize(10);
    let slot_length = slot_length_from_millisec(2000);
    let base_info = fixed_epoch_info(epoch_size, slot_length);
    let basis_slot = SlotNo(25);
    let extended = unsafe_linear_extend_epoch_info(basis_slot, base_info.clone());

    assert_eq!(epoch_info_first(&extended, EpochNo(5)).unwrap(), SlotNo(50));

    let rel = epoch_info_slot_to_relative_time(&extended, SlotNo(30)).unwrap();
    let diff = diff_relative_time(
        rel,
        epoch_info_slot_to_relative_time(&base_info, SlotNo(30)).unwrap(),
    );
    assert_eq!(diff, Duration::ZERO);
}

#[test]
fn slot_length_roundtrips() {
    let length = slot_length_from_millisec(1500);
    assert_eq!(slot_length_to_millisec(length), 1500);
}
