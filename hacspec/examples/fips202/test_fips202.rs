use hacspec::prelude::*;

use crate::fips202::*;

#[test]
fn test_sha3224() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3224(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 28] = [
        0xfb, 0xbf, 0xfb, 0xcb, 0x8a, 0x3e, 0x5b, 0x13, 0xb4, 0x2f, 0x51, 0x69, 0x9d, 0x0f, 0x64,
        0xe8, 0x4e, 0x27, 0x1b, 0xe3, 0xfc, 0xad, 0x11, 0xd6, 0xd7, 0x19, 0x66, 0xcf,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3224(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 28] = [
        0xf2, 0x74, 0x4e, 0xe2, 0x38, 0xaf, 0x95, 0xd3, 0x7d, 0x89, 0x3f, 0x69, 0x53, 0xfa, 0x25,
        0x8e, 0x72, 0xd3, 0x13, 0x61, 0x11, 0x77, 0x28, 0x24, 0xf9, 0x43, 0x60, 0x76,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}

#[test]
fn test_sha3256() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3256(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 32] = [
        0x2e, 0x62, 0xa5, 0xf1, 0x5b, 0x88, 0x2f, 0x43, 0x14, 0x1a, 0xbd, 0xcc, 0x81, 0xf5, 0x0a,
        0x7a, 0xbc, 0xe7, 0x8c, 0x57, 0x84, 0xd9, 0x2e, 0xdc, 0x22, 0xaf, 0xac, 0xe9, 0x3d, 0x7a,
        0xd9, 0x6d,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3256(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 32] = [
        0x78, 0xa0, 0xa9, 0x8b, 0xf5, 0x24, 0xf6, 0xdb, 0xea, 0xcd, 0xda, 0xfd, 0x30, 0x2e, 0x52,
        0x9a, 0xbe, 0x4b, 0x41, 0xbf, 0x28, 0x7b, 0x15, 0x47, 0xfb, 0x8d, 0xf6, 0x8a, 0x13, 0xe2,
        0x25, 0x3f,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}

#[test]
fn test_sha3384() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3384(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 48] = [
        0x39, 0x00, 0x07, 0xfc, 0xd0, 0xe4, 0x00, 0x1d, 0xd0, 0xa1, 0x7d, 0xfc, 0x21, 0xcc, 0xbb,
        0xcd, 0x8c, 0x47, 0x15, 0xa8, 0xc2, 0x82, 0xae, 0x10, 0x85, 0x51, 0x5d, 0xa6, 0xea, 0xa5,
        0xd2, 0x33, 0x1f, 0xfe, 0xf9, 0x03, 0x68, 0xb5, 0x2b, 0x93, 0x64, 0xdc, 0x5d, 0xd7, 0x5b,
        0x5b, 0x33, 0x81,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3384(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 48] = [
        0x3b, 0x8d, 0xd3, 0x50, 0xbf, 0x8d, 0x2b, 0x2f, 0xd5, 0x9c, 0x84, 0x10, 0xd8, 0xdd, 0x43,
        0xa7, 0xe5, 0xae, 0xe0, 0xf9, 0x91, 0x0b, 0xd5, 0xcc, 0x3c, 0xcc, 0x82, 0xdd, 0x30, 0x43,
        0x47, 0x09, 0x02, 0x65, 0x07, 0x4f, 0x79, 0x4a, 0xa5, 0x31, 0x61, 0xd7, 0x40, 0x34, 0x25,
        0x5e, 0xd9, 0x30,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}

#[test]
fn test_sha3512() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3512(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 64] = [
        0x1b, 0x12, 0x41, 0xe1, 0xcb, 0x22, 0xea, 0x62, 0x3f, 0x8e, 0xe4, 0x83, 0x87, 0x88, 0xc1,
        0x3a, 0x3d, 0xc6, 0x5c, 0x36, 0x98, 0xd3, 0x75, 0x85, 0xf3, 0xdf, 0xe2, 0x97, 0x3b, 0x38,
        0x29, 0xf3, 0x40, 0xae, 0xb3, 0x88, 0x0e, 0xe8, 0x0f, 0x14, 0x07, 0xec, 0x8e, 0xe5, 0xb9,
        0x35, 0x8d, 0x76, 0xa3, 0xbd, 0x0d, 0xa3, 0xe2, 0x93, 0x68, 0xa8, 0x1e, 0xda, 0xc6, 0x41,
        0x5e, 0x42, 0x8a, 0x4d,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = sha3512(ByteSeq::from_array(
        &m.into_bytes()
            .iter()
            .map(|x| U8::classify(*x))
            .collect::<Vec<_>>(),
    ));
    let expected: [u8; 64] = [
        0x6f, 0xce, 0x24, 0x1a, 0x00, 0x15, 0xfc, 0x0b, 0xcb, 0x76, 0x28, 0x23, 0x5d, 0x41, 0x2c,
        0x45, 0x6c, 0x79, 0x15, 0x8e, 0xd0, 0x66, 0x79, 0xfd, 0x5e, 0xe1, 0xb4, 0x87, 0xeb, 0x7d,
        0xf7, 0xe9, 0x51, 0x50, 0x84, 0xb6, 0x80, 0x28, 0xf3, 0x3b, 0xed, 0x4a, 0x0e, 0x1f, 0x35,
        0xea, 0xab, 0xd0, 0xf7, 0x38, 0xa8, 0x1b, 0xef, 0x8e, 0x51, 0xb3, 0x60, 0x70, 0x63, 0x2b,
        0xf4, 0x7b, 0x08, 0xb8,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}

#[test]
fn test_shake128() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake128(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        168,
    );
    let expected: [u8; 168] = [
        0x98, 0x76, 0x1a, 0xab, 0x5c, 0x78, 0x8e, 0x67, 0xa5, 0x9a, 0x3c, 0x2e, 0xb9, 0x25, 0x2d,
        0x14, 0x03, 0x7f, 0xae, 0xa0, 0xff, 0xc7, 0xf8, 0xe4, 0xb1, 0xab, 0xc6, 0x90, 0x44, 0x2d,
        0x96, 0x28, 0x8e, 0xc5, 0x28, 0x49, 0xb2, 0x17, 0xc3, 0x2e, 0xaf, 0xd6, 0xe5, 0x63, 0xd1,
        0x35, 0x50, 0xbb, 0x2f, 0x4a, 0xc6, 0x52, 0xcc, 0x88, 0xd1, 0x55, 0xa6, 0xbc, 0xee, 0xfc,
        0x89, 0x65, 0x61, 0x72, 0xa9, 0x16, 0x69, 0x36, 0xb6, 0x08, 0xa5, 0x5f, 0x18, 0x11, 0xa5,
        0xf4, 0xb4, 0xd5, 0xa1, 0x28, 0x5d, 0x59, 0x13, 0x14, 0x4c, 0xc4, 0x53, 0xd1, 0xa7, 0x9f,
        0x23, 0x12, 0xa6, 0x40, 0x5a, 0xff, 0x05, 0xac, 0xde, 0xb9, 0xd1, 0x7d, 0x60, 0xdc, 0x89,
        0x59, 0x91, 0x4e, 0xdb, 0x0e, 0xc9, 0xb0, 0x9d, 0xdf, 0x5c, 0x87, 0x39, 0x41, 0x57, 0x21,
        0xf9, 0x97, 0xd6, 0xec, 0x12, 0xdd, 0x15, 0xe7, 0x7a, 0x0f, 0x85, 0x4a, 0x97, 0xbc, 0x16,
        0xb3, 0x13, 0xee, 0xa3, 0x81, 0xa6, 0x32, 0xf8, 0xfa, 0x92, 0x24, 0xd4, 0xad, 0x70, 0xa5,
        0xa0, 0xb0, 0x29, 0x47, 0x2a, 0x56, 0xfa, 0xc4, 0x3c, 0x9b, 0xf3, 0x79, 0x95, 0x4a, 0x4a,
        0x0f, 0xfb, 0xe1,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake128(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        400,
    );
    let expected: [u8; 400] = [
        0x98, 0x76, 0x1a, 0xab, 0x5c, 0x78, 0x8e, 0x67, 0xa5, 0x9a, 0x3c, 0x2e, 0xb9, 0x25, 0x2d,
        0x14, 0x03, 0x7f, 0xae, 0xa0, 0xff, 0xc7, 0xf8, 0xe4, 0xb1, 0xab, 0xc6, 0x90, 0x44, 0x2d,
        0x96, 0x28, 0x8e, 0xc5, 0x28, 0x49, 0xb2, 0x17, 0xc3, 0x2e, 0xaf, 0xd6, 0xe5, 0x63, 0xd1,
        0x35, 0x50, 0xbb, 0x2f, 0x4a, 0xc6, 0x52, 0xcc, 0x88, 0xd1, 0x55, 0xa6, 0xbc, 0xee, 0xfc,
        0x89, 0x65, 0x61, 0x72, 0xa9, 0x16, 0x69, 0x36, 0xb6, 0x08, 0xa5, 0x5f, 0x18, 0x11, 0xa5,
        0xf4, 0xb4, 0xd5, 0xa1, 0x28, 0x5d, 0x59, 0x13, 0x14, 0x4c, 0xc4, 0x53, 0xd1, 0xa7, 0x9f,
        0x23, 0x12, 0xa6, 0x40, 0x5a, 0xff, 0x05, 0xac, 0xde, 0xb9, 0xd1, 0x7d, 0x60, 0xdc, 0x89,
        0x59, 0x91, 0x4e, 0xdb, 0x0e, 0xc9, 0xb0, 0x9d, 0xdf, 0x5c, 0x87, 0x39, 0x41, 0x57, 0x21,
        0xf9, 0x97, 0xd6, 0xec, 0x12, 0xdd, 0x15, 0xe7, 0x7a, 0x0f, 0x85, 0x4a, 0x97, 0xbc, 0x16,
        0xb3, 0x13, 0xee, 0xa3, 0x81, 0xa6, 0x32, 0xf8, 0xfa, 0x92, 0x24, 0xd4, 0xad, 0x70, 0xa5,
        0xa0, 0xb0, 0x29, 0x47, 0x2a, 0x56, 0xfa, 0xc4, 0x3c, 0x9b, 0xf3, 0x79, 0x95, 0x4a, 0x4a,
        0x0f, 0xfb, 0xe1, 0x45, 0x90, 0xd4, 0x9f, 0xbe, 0x55, 0x47, 0x3b, 0x0a, 0xf3, 0x34, 0xe4,
        0xd9, 0xb3, 0xa7, 0x70, 0x1f, 0x53, 0xb0, 0xcd, 0x88, 0x4b, 0xa1, 0x1d, 0xa2, 0xb3, 0xfe,
        0xc1, 0x3e, 0x97, 0xbe, 0xe9, 0x19, 0xc7, 0x94, 0x4b, 0x05, 0xe5, 0x24, 0x4d, 0x6f, 0xc1,
        0x54, 0x71, 0x30, 0xb3, 0xf1, 0x9b, 0x47, 0x34, 0x2c, 0xe6, 0x60, 0x6d, 0x3f, 0xcf, 0x5d,
        0xed, 0x1c, 0xc8, 0xf6, 0x08, 0x1d, 0x9a, 0xd3, 0x55, 0x3d, 0x63, 0xdd, 0xf8, 0x4e, 0x75,
        0xa0, 0x81, 0xf5, 0x9e, 0xc2, 0xce, 0xe0, 0x2c, 0x3f, 0x6e, 0x3e, 0x9a, 0x44, 0xcf, 0xf8,
        0x8f, 0x94, 0xe9, 0x23, 0xe6, 0x35, 0xe1, 0x3e, 0x29, 0x00, 0xdc, 0xc1, 0x81, 0x7d, 0xc7,
        0xf6, 0x5a, 0x8d, 0x29, 0xd1, 0x10, 0x58, 0x26, 0x68, 0x33, 0xdd, 0x10, 0xdb, 0x68, 0x44,
        0x04, 0x9a, 0xc3, 0x6b, 0xbe, 0x89, 0xe3, 0x81, 0xb2, 0xf9, 0x77, 0xf1, 0x78, 0xdb, 0xdb,
        0xf2, 0x8f, 0x97, 0xfb, 0xb9, 0xf9, 0x70, 0x90, 0xb6, 0x37, 0x4f, 0x92, 0x15, 0x0a, 0xa5,
        0xc9, 0x62, 0xfa, 0xf0, 0xe5, 0xb0, 0x00, 0x8e, 0x2b, 0xa7, 0x14, 0xf2, 0xe2, 0x91, 0xd1,
        0x37, 0xcf, 0xbc, 0x78, 0x82, 0x6e, 0x71, 0x7f, 0x49, 0xd7, 0x9f, 0x51, 0x3f, 0x2e, 0x49,
        0xeb, 0x2b, 0x48, 0x87, 0xf8, 0xff, 0x86, 0x1a, 0xf2, 0xa9, 0xc9, 0x5e, 0x52, 0xd3, 0x55,
        0x18, 0xd7, 0xd9, 0x72, 0x34, 0x63, 0x29, 0x7f, 0x3a, 0x44, 0xe1, 0x85, 0x35, 0x8b, 0x8d,
        0xaf, 0x16, 0xb5, 0x9d, 0x6a, 0xfd, 0x67, 0x80, 0x4e, 0x7e, 0xfc, 0x1f, 0xe5, 0x80, 0x23,
        0x4e, 0xd0, 0xdd, 0x94, 0x55, 0x65, 0xb6, 0xbf, 0x4e, 0x32,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake128(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        168,
    );
    let expected: [u8; 168] = [
        0x20, 0xd1, 0x41, 0xbb, 0xfc, 0xb3, 0xce, 0x24, 0x78, 0x6c, 0x39, 0x8f, 0xd4, 0xe3, 0x06,
        0x5e, 0x81, 0x05, 0x94, 0x56, 0xc8, 0xd4, 0xd6, 0x50, 0xc3, 0x46, 0xe4, 0x11, 0x15, 0x2f,
        0x4c, 0x02, 0xa8, 0xff, 0x4a, 0x5d, 0xc8, 0x4a, 0xb3, 0x97, 0x05, 0xd9, 0x54, 0xdf, 0x66,
        0x9a, 0x67, 0xf7, 0x08, 0xf7, 0x16, 0xa5, 0xb1, 0x7f, 0x5b, 0xdd, 0xd0, 0x97, 0x5c, 0x69,
        0xc3, 0x4c, 0x97, 0xf1, 0xfb, 0x77, 0xac, 0x13, 0xdd, 0x75, 0x50, 0x31, 0xbb, 0x73, 0x68,
        0x71, 0x7e, 0x3b, 0x75, 0x93, 0xfb, 0x30, 0xda, 0x16, 0x6a, 0x7d, 0x80, 0xfe, 0xdb, 0x0e,
        0x27, 0x95, 0x05, 0x06, 0x90, 0x8d, 0x55, 0x42, 0x91, 0x1d, 0x11, 0x11, 0x51, 0x6a, 0x57,
        0x40, 0x83, 0xa4, 0xe9, 0xe1, 0xaa, 0xd9, 0x7d, 0x82, 0x3f, 0x28, 0x86, 0x58, 0x5e, 0xe8,
        0xbc, 0x71, 0xae, 0x25, 0x57, 0xa9, 0xc5, 0xf1, 0x5b, 0x13, 0xd6, 0x79, 0x0b, 0xc7, 0xee,
        0x6f, 0xbb, 0xe3, 0xd8, 0x74, 0x2a, 0xcb, 0x72, 0x6a, 0xaa, 0xe2, 0x42, 0xf9, 0x1c, 0x30,
        0x7b, 0x2c, 0xa1, 0x7e, 0x1a, 0x27, 0x1c, 0x9c, 0x6a, 0xa1, 0xd0, 0x9b, 0x59, 0x56, 0xaa,
        0xfb, 0xd6, 0xaf,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake128(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        400,
    );
    let expected: [u8; 400] = [
        0x20, 0xd1, 0x41, 0xbb, 0xfc, 0xb3, 0xce, 0x24, 0x78, 0x6c, 0x39, 0x8f, 0xd4, 0xe3, 0x06,
        0x5e, 0x81, 0x05, 0x94, 0x56, 0xc8, 0xd4, 0xd6, 0x50, 0xc3, 0x46, 0xe4, 0x11, 0x15, 0x2f,
        0x4c, 0x02, 0xa8, 0xff, 0x4a, 0x5d, 0xc8, 0x4a, 0xb3, 0x97, 0x05, 0xd9, 0x54, 0xdf, 0x66,
        0x9a, 0x67, 0xf7, 0x08, 0xf7, 0x16, 0xa5, 0xb1, 0x7f, 0x5b, 0xdd, 0xd0, 0x97, 0x5c, 0x69,
        0xc3, 0x4c, 0x97, 0xf1, 0xfb, 0x77, 0xac, 0x13, 0xdd, 0x75, 0x50, 0x31, 0xbb, 0x73, 0x68,
        0x71, 0x7e, 0x3b, 0x75, 0x93, 0xfb, 0x30, 0xda, 0x16, 0x6a, 0x7d, 0x80, 0xfe, 0xdb, 0x0e,
        0x27, 0x95, 0x05, 0x06, 0x90, 0x8d, 0x55, 0x42, 0x91, 0x1d, 0x11, 0x11, 0x51, 0x6a, 0x57,
        0x40, 0x83, 0xa4, 0xe9, 0xe1, 0xaa, 0xd9, 0x7d, 0x82, 0x3f, 0x28, 0x86, 0x58, 0x5e, 0xe8,
        0xbc, 0x71, 0xae, 0x25, 0x57, 0xa9, 0xc5, 0xf1, 0x5b, 0x13, 0xd6, 0x79, 0x0b, 0xc7, 0xee,
        0x6f, 0xbb, 0xe3, 0xd8, 0x74, 0x2a, 0xcb, 0x72, 0x6a, 0xaa, 0xe2, 0x42, 0xf9, 0x1c, 0x30,
        0x7b, 0x2c, 0xa1, 0x7e, 0x1a, 0x27, 0x1c, 0x9c, 0x6a, 0xa1, 0xd0, 0x9b, 0x59, 0x56, 0xaa,
        0xfb, 0xd6, 0xaf, 0x96, 0x2e, 0x1a, 0xe0, 0x5a, 0x3f, 0xcf, 0x5b, 0x98, 0x40, 0x4c, 0xba,
        0x18, 0x33, 0x7d, 0x44, 0x60, 0x29, 0x24, 0x65, 0x07, 0xc2, 0x1a, 0xc9, 0x12, 0x14, 0x0f,
        0xdf, 0xc1, 0x90, 0xf6, 0xed, 0xc4, 0x78, 0x1d, 0xf7, 0x9f, 0x00, 0xa7, 0x6a, 0xc7, 0xd7,
        0x4c, 0x89, 0xd9, 0x3e, 0x7a, 0xaf, 0xca, 0x55, 0x03, 0x31, 0x88, 0x48, 0x2e, 0xae, 0x89,
        0xd7, 0x05, 0x66, 0xcc, 0x0a, 0x40, 0x9f, 0x19, 0x9b, 0x60, 0xb7, 0x1a, 0xa3, 0xa5, 0x15,
        0x8c, 0x33, 0x06, 0x78, 0x38, 0x6e, 0xcc, 0x11, 0xd5, 0xd1, 0x90, 0xd5, 0xfe, 0x58, 0x8b,
        0x92, 0x98, 0xec, 0x25, 0x61, 0x99, 0x11, 0x52, 0x09, 0x71, 0xad, 0x0f, 0x67, 0x76, 0x99,
        0x9e, 0x61, 0xd3, 0xcc, 0xa2, 0x52, 0xdb, 0x78, 0x65, 0xf2, 0xa7, 0xf1, 0x1e, 0xf0, 0xf7,
        0xed, 0xbf, 0x76, 0x3c, 0x47, 0x81, 0x02, 0x4d, 0x30, 0xdd, 0x37, 0xc4, 0x2b, 0xd0, 0x02,
        0xf5, 0x3f, 0xae, 0x63, 0xd3, 0xaf, 0x66, 0x41, 0x9b, 0x3b, 0x9b, 0xb9, 0xae, 0x79, 0xdc,
        0x25, 0x58, 0x26, 0x0f, 0xb0, 0x51, 0x10, 0xc0, 0x56, 0xb9, 0xe7, 0xf3, 0xaa, 0xbb, 0x73,
        0xbb, 0x34, 0x82, 0x75, 0xc8, 0x7d, 0x50, 0xd1, 0xb3, 0xa6, 0xb3, 0xc1, 0x96, 0xea, 0x7c,
        0xb3, 0x66, 0x71, 0x03, 0x55, 0xb8, 0xec, 0x5d, 0x99, 0xe8, 0x3f, 0xa1, 0x79, 0xf0, 0xc8,
        0x69, 0x17, 0x74, 0x64, 0xda, 0x60, 0xe9, 0x0c, 0x0d, 0xf3, 0x0b, 0x60, 0x8d, 0xae, 0xab,
        0x17, 0xaa, 0xca, 0xca, 0xf0, 0x4e, 0x2e, 0x1e, 0x87, 0xf6, 0x8a, 0xb9, 0xb7, 0xd3, 0x11,
        0xc1, 0x64, 0x3a, 0x92, 0xcb, 0xc8, 0x59, 0xf9, 0x7a, 0xbc,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}

#[test]
fn test_shake256() {
    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake256(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        136,
    );
    let expected: [u8; 136] = [
        0xdb, 0x3d, 0x93, 0xd7, 0xa4, 0xe8, 0x0c, 0xc3, 0xba, 0x50, 0xd2, 0x58, 0x5b, 0x54, 0xb8,
        0x25, 0x41, 0xcf, 0xc3, 0x45, 0xbe, 0x68, 0xb0, 0x96, 0x4a, 0x65, 0x7c, 0xe8, 0xb3, 0xfd,
        0x68, 0xe0, 0x1c, 0x2b, 0x24, 0xc6, 0x24, 0xfb, 0xd9, 0xdf, 0x88, 0x92, 0x9b, 0x50, 0xb5,
        0xec, 0xc2, 0x17, 0xc3, 0x6a, 0xfd, 0x29, 0xdf, 0xa3, 0x45, 0xba, 0x83, 0xbd, 0x6a, 0x07,
        0x49, 0xf0, 0x52, 0x0f, 0x34, 0x9c, 0xd6, 0xc2, 0x09, 0x53, 0xcc, 0x7a, 0x4f, 0x27, 0x34,
        0x8d, 0xfc, 0x68, 0x8a, 0x1c, 0xcb, 0xfc, 0x5a, 0x6f, 0xc1, 0xb7, 0x8f, 0x74, 0xb1, 0x02,
        0x4e, 0x89, 0x32, 0x42, 0x0a, 0x36, 0x3d, 0x8b, 0x47, 0x95, 0xd7, 0x07, 0x36, 0x2a, 0x2c,
        0x31, 0xb4, 0xb6, 0x8f, 0x69, 0x1f, 0x62, 0x4f, 0x32, 0x85, 0x42, 0xb1, 0x72, 0xb1, 0x28,
        0x9b, 0x26, 0xf8, 0xb6, 0x24, 0x0c, 0x0c, 0x18, 0x58, 0x4e, 0xc7, 0x72, 0x58, 0x8c, 0xc6,
        0x8a,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake256(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        400,
    );
    let expected: [u8; 400] = [
        0xdb, 0x3d, 0x93, 0xd7, 0xa4, 0xe8, 0x0c, 0xc3, 0xba, 0x50, 0xd2, 0x58, 0x5b, 0x54, 0xb8,
        0x25, 0x41, 0xcf, 0xc3, 0x45, 0xbe, 0x68, 0xb0, 0x96, 0x4a, 0x65, 0x7c, 0xe8, 0xb3, 0xfd,
        0x68, 0xe0, 0x1c, 0x2b, 0x24, 0xc6, 0x24, 0xfb, 0xd9, 0xdf, 0x88, 0x92, 0x9b, 0x50, 0xb5,
        0xec, 0xc2, 0x17, 0xc3, 0x6a, 0xfd, 0x29, 0xdf, 0xa3, 0x45, 0xba, 0x83, 0xbd, 0x6a, 0x07,
        0x49, 0xf0, 0x52, 0x0f, 0x34, 0x9c, 0xd6, 0xc2, 0x09, 0x53, 0xcc, 0x7a, 0x4f, 0x27, 0x34,
        0x8d, 0xfc, 0x68, 0x8a, 0x1c, 0xcb, 0xfc, 0x5a, 0x6f, 0xc1, 0xb7, 0x8f, 0x74, 0xb1, 0x02,
        0x4e, 0x89, 0x32, 0x42, 0x0a, 0x36, 0x3d, 0x8b, 0x47, 0x95, 0xd7, 0x07, 0x36, 0x2a, 0x2c,
        0x31, 0xb4, 0xb6, 0x8f, 0x69, 0x1f, 0x62, 0x4f, 0x32, 0x85, 0x42, 0xb1, 0x72, 0xb1, 0x28,
        0x9b, 0x26, 0xf8, 0xb6, 0x24, 0x0c, 0x0c, 0x18, 0x58, 0x4e, 0xc7, 0x72, 0x58, 0x8c, 0xc6,
        0x8a, 0xd7, 0xa9, 0x6d, 0xd8, 0x72, 0x9c, 0x6a, 0xa0, 0x29, 0x55, 0xd8, 0x3e, 0x7b, 0xc0,
        0x2c, 0x79, 0x49, 0x59, 0x25, 0x18, 0x6c, 0x28, 0x98, 0x01, 0xa6, 0x66, 0x5d, 0x61, 0x51,
        0xe1, 0xad, 0x61, 0x8c, 0x7c, 0x76, 0xd1, 0x6d, 0x88, 0x6f, 0x1e, 0x47, 0xad, 0x46, 0x0f,
        0x62, 0x34, 0xdd, 0x7c, 0x55, 0x1d, 0xc7, 0x9a, 0x41, 0x5b, 0xad, 0x99, 0x59, 0xd2, 0xd4,
        0x88, 0x35, 0x41, 0xe6, 0x67, 0xbe, 0x6a, 0x83, 0xcb, 0x91, 0x9a, 0x71, 0xcb, 0x48, 0xd8,
        0x89, 0x78, 0xd7, 0xc0, 0xb4, 0xb7, 0xe7, 0xe1, 0xf0, 0xd7, 0x03, 0xb2, 0x77, 0xf5, 0x0a,
        0x62, 0xee, 0xb5, 0x8f, 0x67, 0x56, 0xce, 0x80, 0x0f, 0xf9, 0x88, 0xed, 0xf8, 0xde, 0x8c,
        0xb7, 0xe2, 0xcc, 0x28, 0x45, 0x21, 0x13, 0xe7, 0x0c, 0x36, 0x55, 0x60, 0x7f, 0x9d, 0x65,
        0x94, 0xc1, 0xac, 0x89, 0x0e, 0x6c, 0xf0, 0xc7, 0xd3, 0xb9, 0xbc, 0xf8, 0x89, 0xe8, 0x8a,
        0xd5, 0xad, 0x3c, 0xde, 0xf1, 0x56, 0x21, 0xe3, 0x01, 0x43, 0xb7, 0x3d, 0x9b, 0xb6, 0x35,
        0x5a, 0x13, 0xb9, 0x1a, 0x84, 0x03, 0xb6, 0x1c, 0x18, 0x5d, 0xe9, 0xc5, 0x65, 0x5a, 0xe0,
        0x77, 0xdc, 0xf3, 0x4d, 0xa0, 0xef, 0x5f, 0x23, 0x92, 0x16, 0xbb, 0xa7, 0x9a, 0xc8, 0x5a,
        0xe9, 0x69, 0x74, 0x53, 0x86, 0xea, 0x6c, 0x86, 0xfc, 0x35, 0x16, 0x15, 0x3b, 0xe4, 0xf2,
        0xee, 0x01, 0xe4, 0x6c, 0xce, 0x09, 0x64, 0x95, 0x32, 0x16, 0xff, 0x7c, 0x59, 0xbf, 0x80,
        0x60, 0xad, 0x54, 0x4c, 0x56, 0x6b, 0xfa, 0xa9, 0x0d, 0x61, 0x9d, 0xf5, 0x32, 0x5d, 0x13,
        0x0c, 0x59, 0xfa, 0x41, 0x6c, 0xe8, 0xa3, 0x58, 0x0d, 0x98, 0x89, 0x69, 0xeb, 0x0f, 0xbd,
        0x04, 0x6f, 0x91, 0xcb, 0x71, 0x0a, 0xc8, 0x0a, 0x86, 0xe4, 0xe3, 0x3c, 0x50, 0xfa, 0xea,
        0xef, 0x25, 0xcc, 0x3d, 0x10, 0xcf, 0xdb, 0x68, 0x46, 0x34,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake256(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        136,
    );
    let expected: [u8; 136] = [
        0x38, 0x18, 0xa3, 0x34, 0x5d, 0x86, 0x5c, 0x9b, 0x72, 0x00, 0x5b, 0x87, 0x45, 0x4f, 0x9c,
        0x71, 0xac, 0x26, 0x25, 0x44, 0x7c, 0xca, 0xed, 0x97, 0x8d, 0xda, 0x85, 0x12, 0xdc, 0x35,
        0xa0, 0x6d, 0x9d, 0x1a, 0xcc, 0xeb, 0x1e, 0x9a, 0xa3, 0xdd, 0x5a, 0xee, 0xd4, 0x2e, 0xa2,
        0x95, 0x9c, 0xde, 0x18, 0x6f, 0xa1, 0x8d, 0xe4, 0x66, 0x4a, 0x2c, 0x60, 0x6f, 0xa7, 0xb6,
        0x16, 0x9a, 0xa5, 0x5c, 0xe3, 0xaa, 0xce, 0xd7, 0x38, 0x78, 0xa7, 0x2a, 0x1d, 0xcc, 0x7a,
        0xbf, 0xdc, 0x4e, 0x46, 0xd0, 0x68, 0x99, 0x3a, 0x23, 0xf1, 0x5c, 0x23, 0x4c, 0x61, 0x0c,
        0x06, 0x8c, 0xa2, 0xe0, 0x4a, 0xb6, 0xa5, 0xac, 0x28, 0x52, 0x07, 0x4f, 0xea, 0x3c, 0xea,
        0x5a, 0x30, 0x2f, 0x20, 0xe1, 0x56, 0xcd, 0x29, 0xf7, 0x59, 0x68, 0xd8, 0x0c, 0xe9, 0x24,
        0x82, 0x42, 0xd6, 0x81, 0xfb, 0xe0, 0xac, 0xb2, 0x6b, 0x81, 0x41, 0x2a, 0x90, 0xb4, 0xf3,
        0x66,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );

    let m = String::from("qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789qwertzuiopasdfghjklyxcvbnm123456789");
    let h = shake256(
        ByteSeq::from_array(
            &m.into_bytes()
                .iter()
                .map(|x| U8::classify(*x))
                .collect::<Vec<_>>(),
        ),
        400,
    );
    let expected: [u8; 400] = [
        0x38, 0x18, 0xa3, 0x34, 0x5d, 0x86, 0x5c, 0x9b, 0x72, 0x00, 0x5b, 0x87, 0x45, 0x4f, 0x9c,
        0x71, 0xac, 0x26, 0x25, 0x44, 0x7c, 0xca, 0xed, 0x97, 0x8d, 0xda, 0x85, 0x12, 0xdc, 0x35,
        0xa0, 0x6d, 0x9d, 0x1a, 0xcc, 0xeb, 0x1e, 0x9a, 0xa3, 0xdd, 0x5a, 0xee, 0xd4, 0x2e, 0xa2,
        0x95, 0x9c, 0xde, 0x18, 0x6f, 0xa1, 0x8d, 0xe4, 0x66, 0x4a, 0x2c, 0x60, 0x6f, 0xa7, 0xb6,
        0x16, 0x9a, 0xa5, 0x5c, 0xe3, 0xaa, 0xce, 0xd7, 0x38, 0x78, 0xa7, 0x2a, 0x1d, 0xcc, 0x7a,
        0xbf, 0xdc, 0x4e, 0x46, 0xd0, 0x68, 0x99, 0x3a, 0x23, 0xf1, 0x5c, 0x23, 0x4c, 0x61, 0x0c,
        0x06, 0x8c, 0xa2, 0xe0, 0x4a, 0xb6, 0xa5, 0xac, 0x28, 0x52, 0x07, 0x4f, 0xea, 0x3c, 0xea,
        0x5a, 0x30, 0x2f, 0x20, 0xe1, 0x56, 0xcd, 0x29, 0xf7, 0x59, 0x68, 0xd8, 0x0c, 0xe9, 0x24,
        0x82, 0x42, 0xd6, 0x81, 0xfb, 0xe0, 0xac, 0xb2, 0x6b, 0x81, 0x41, 0x2a, 0x90, 0xb4, 0xf3,
        0x66, 0x2f, 0x58, 0xd3, 0xdc, 0x1a, 0xa1, 0xa5, 0x39, 0xda, 0x2a, 0xc6, 0xd1, 0xfb, 0x45,
        0x11, 0xd9, 0xcd, 0x48, 0xfe, 0x62, 0xb4, 0xf0, 0x79, 0x2e, 0xbc, 0x6b, 0x4a, 0x00, 0x8c,
        0x56, 0xef, 0x1c, 0xf6, 0x2f, 0x49, 0x5b, 0x35, 0xb1, 0x46, 0xcb, 0x8a, 0x81, 0x1b, 0x8e,
        0x19, 0xfa, 0x17, 0xd2, 0xaf, 0x2f, 0x08, 0x4b, 0x6d, 0xfd, 0x31, 0x9f, 0x01, 0xe2, 0x30,
        0xe1, 0xb2, 0x33, 0xe0, 0xf6, 0x3f, 0x97, 0xad, 0x9b, 0x5a, 0xca, 0x82, 0xc8, 0xa2, 0x88,
        0x2b, 0xd5, 0x4d, 0x08, 0x56, 0x6c, 0xac, 0xba, 0x8f, 0x59, 0xaa, 0x4c, 0xa3, 0x47, 0x92,
        0xe8, 0x91, 0xc3, 0x7b, 0x72, 0xa6, 0x7e, 0x2d, 0xbb, 0x74, 0x17, 0x1b, 0x8f, 0x78, 0x53,
        0xf8, 0x4b, 0xc4, 0x31, 0x52, 0x38, 0x50, 0x6f, 0xd3, 0xdb, 0x53, 0x8c, 0x46, 0x58, 0x25,
        0x30, 0xd8, 0x54, 0xce, 0xa5, 0xc3, 0xc9, 0xbe, 0x53, 0x11, 0xa4, 0x55, 0x4e, 0x3a, 0x5c,
        0x34, 0x5d, 0xe7, 0xe3, 0x00, 0x18, 0x26, 0xa0, 0x99, 0x14, 0xad, 0xa2, 0x18, 0x8d, 0xd8,
        0x3c, 0xea, 0xb9, 0x60, 0xa5, 0x29, 0xbf, 0x8d, 0x84, 0x67, 0x53, 0xf4, 0xb2, 0x67, 0xf3,
        0xa1, 0x6e, 0x00, 0x5b, 0x43, 0xc4, 0x0f, 0x7b, 0x71, 0xe7, 0x5c, 0xaf, 0x9c, 0x00, 0xdd,
        0x18, 0xd3, 0x33, 0x85, 0x51, 0xa0, 0xb2, 0x31, 0xed, 0x06, 0x70, 0x2c, 0x19, 0xfc, 0x68,
        0xc1, 0xe7, 0xf8, 0xe1, 0x0d, 0x00, 0xc3, 0x5a, 0x22, 0x29, 0x1a, 0xfc, 0x0a, 0x04, 0x34,
        0xdb, 0x07, 0x8e, 0x47, 0x96, 0x76, 0x95, 0xe1, 0xc9, 0x01, 0x11, 0x5d, 0xa2, 0x85, 0xfb,
        0xba, 0xf8, 0x94, 0x99, 0x37, 0xed, 0x4e, 0xf3, 0x65, 0xfc, 0x7d, 0xba, 0x67, 0x8f, 0x4f,
        0x6e, 0xc7, 0x65, 0x77, 0x15, 0x66, 0x98, 0xe2, 0xc9, 0x0b, 0x59, 0x4d, 0xc7, 0x60, 0x53,
        0x7a, 0x6f, 0x22, 0xbd, 0x1e, 0xef, 0xed, 0x7b, 0x73, 0xbe,
    ];
    assert_eq!(
        expected.iter().map(|x| *x).collect::<Vec<_>>(),
        h.iter().map(|x| U8::declassify(*x)).collect::<Vec<_>>()
    );
}
