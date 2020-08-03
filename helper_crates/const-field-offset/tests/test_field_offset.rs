use const_field_offset::*;
use memoffset::offset_of;

#[derive(FieldOffsets)]
#[repr(C)]
struct MyStruct {
    a: u8,
    b: u16,
    c: u8,
    d: u16,
}

#[derive(FieldOffsets)]
#[repr(C)]
struct MyStruct2 {
    k: core::cell::Cell<isize>,
    xx: MyStruct,
    v: u32,
}

#[derive(FieldOffsets)]
#[repr(C)]
struct MyStruct3 {
    ms2: MyStruct2,
}

const XX_CONST: usize = MyStruct2::FIELD_OFFSETS.xx.get_byte_offset();
static D_STATIC: usize = MyStruct::FIELD_OFFSETS.d.get_byte_offset();

#[test]
fn test() {
    assert_eq!(offset_of!(MyStruct, a), MyStruct::FIELD_OFFSETS.a.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, b), MyStruct::FIELD_OFFSETS.b.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, c), MyStruct::FIELD_OFFSETS.c.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, d), MyStruct::FIELD_OFFSETS.d.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, xx), MyStruct2::FIELD_OFFSETS.xx.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, v), MyStruct2::FIELD_OFFSETS.v.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, k), MyStruct2::FIELD_OFFSETS.k.get_byte_offset());

    assert_eq!(XX_CONST, offset_of!(MyStruct2, xx));
    assert_eq!(D_STATIC, offset_of!(MyStruct, d));

    assert_eq!(offset_of!(MyStruct, a), MyStruct_field_offsets::a.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, b), MyStruct_field_offsets::b.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, c), MyStruct_field_offsets::c.get_byte_offset());
    assert_eq!(offset_of!(MyStruct, d), MyStruct_field_offsets::d.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, xx), MyStruct2_field_offsets::xx.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, v), MyStruct2_field_offsets::v.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2, k), MyStruct2_field_offsets::k.get_byte_offset());

    assert_eq!(core::mem::size_of::<MyStruct_field_offsets::c>(), 0);

    let d_in_ms2 = MyStruct2_field_offsets::xx + MyStruct_field_offsets::d;
    assert_eq!(offset_of!(MyStruct2, xx) + offset_of!(MyStruct, d), d_in_ms2.get_byte_offset());
    assert_eq!(core::mem::size_of_val(&d_in_ms2), 0);

    let a = MyStruct3_field_offsets::ms2 + d_in_ms2;
    let b = MyStruct3_field_offsets::ms2 + MyStruct2_field_offsets::xx + MyStruct_field_offsets::d;
    assert_eq!(a.get_byte_offset(), b.get_byte_offset());
}

#[derive(FieldOffsets)]
#[repr(C)]
#[pin]
struct MyStructPin {
    phantom: core::marker::PhantomPinned,
    a: u8,
    b: u16,
    c: u8,
    d: u16,
}

#[derive(FieldOffsets)]
#[repr(C)]
#[pin]
struct MyStruct2Pin {
    phantom: core::marker::PhantomPinned,
    k: core::cell::Cell<isize>,
    xx: MyStruct,
    v: u32,
}

const XX_CONST_PIN: usize = MyStruct2Pin::FIELD_OFFSETS.xx.get_byte_offset();
static D_STATIC_PIN: usize = MyStructPin::FIELD_OFFSETS.d.get_byte_offset();

#[test]
fn test_pin() {
    assert_eq!(offset_of!(MyStructPin, a), MyStructPin::FIELD_OFFSETS.a.get_byte_offset());
    assert_eq!(offset_of!(MyStructPin, b), MyStructPin::FIELD_OFFSETS.b.get_byte_offset());
    assert_eq!(offset_of!(MyStructPin, c), MyStructPin::FIELD_OFFSETS.c.get_byte_offset());
    assert_eq!(offset_of!(MyStructPin, d), MyStructPin::FIELD_OFFSETS.d.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2Pin, xx), MyStruct2Pin::FIELD_OFFSETS.xx.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2Pin, v), MyStruct2Pin::FIELD_OFFSETS.v.get_byte_offset());
    assert_eq!(offset_of!(MyStruct2Pin, k), MyStruct2Pin::FIELD_OFFSETS.k.get_byte_offset());

    assert_eq!(XX_CONST_PIN, offset_of!(MyStruct2Pin, xx));
    assert_eq!(D_STATIC_PIN, offset_of!(MyStructPin, d));
}
