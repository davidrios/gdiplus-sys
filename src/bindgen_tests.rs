#![allow(deref_nullptr, unaligned_references)]

use crate::*;

#[test]
fn bindgen_test_layout_Gdiplus_GdiplusBase() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GdiplusBase>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GdiplusBase))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GdiplusBase>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GdiplusBase))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_SizeF() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_SizeF>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_SizeF))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_SizeF>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_SizeF))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_SizeF>())).Width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_SizeF),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_SizeF>())).Height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_SizeF),
            "::",
            stringify!(Height)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_PointF() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_PointF>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_PointF))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_PointF>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_PointF))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PointF>())).X as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PointF),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PointF>())).Y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PointF),
            "::",
            stringify!(Y)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Point() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Point>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_Point))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Point>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_Point))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Point>())).X as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Point),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Point>())).Y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Point),
            "::",
            stringify!(Y)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_RectF() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_RectF>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_RectF))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_RectF>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_RectF))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_RectF>())).X as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_RectF),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_RectF>())).Y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_RectF),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_RectF>())).Width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_RectF),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_RectF>())).Height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_RectF),
            "::",
            stringify!(Height)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Rect() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Rect>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Rect))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Rect>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_Rect))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Rect>())).X as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Rect),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Rect>())).Y as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Rect),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Rect>())).Width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Rect),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Rect>())).Height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Rect),
            "::",
            stringify!(Height)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_PathData() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_PathData>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_PathData))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_PathData>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_PathData))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PathData>())).Count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PathData),
            "::",
            stringify!(Count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PathData>())).Points as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PathData),
            "::",
            stringify!(Points)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PathData>())).Types as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PathData),
            "::",
            stringify!(Types)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_CharacterRange() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_CharacterRange>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_CharacterRange))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_CharacterRange>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_CharacterRange))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_CharacterRange>())).First as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CharacterRange),
            "::",
            stringify!(First)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_CharacterRange>())).Length as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CharacterRange),
            "::",
            stringify!(Length)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GdiplusStartupInput() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GdiplusStartupInput>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_GdiplusStartupInput))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GdiplusStartupInput>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_GdiplusStartupInput))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupInput>())).GdiplusVersion as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupInput),
            "::",
            stringify!(GdiplusVersion)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupInput>())).DebugEventCallback as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupInput),
            "::",
            stringify!(DebugEventCallback)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupInput>())).SuppressBackgroundThread
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupInput),
            "::",
            stringify!(SuppressBackgroundThread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupInput>())).SuppressExternalCodecs
                as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupInput),
            "::",
            stringify!(SuppressExternalCodecs)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GdiplusStartupOutput() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GdiplusStartupOutput>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_GdiplusStartupOutput))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GdiplusStartupOutput>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_GdiplusStartupOutput))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupOutput>())).NotificationHook as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupOutput),
            "::",
            stringify!(NotificationHook)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_GdiplusStartupOutput>())).NotificationUnhook as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GdiplusStartupOutput),
            "::",
            stringify!(NotificationUnhook)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_ColorPalette() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_ColorPalette>(),
        12usize,
        concat!("Size of: ", stringify!(Gdiplus_ColorPalette))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_ColorPalette>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_ColorPalette))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorPalette>())).Flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorPalette),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorPalette>())).Count as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorPalette),
            "::",
            stringify!(Count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorPalette>())).Entries as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorPalette),
            "::",
            stringify!(Entries)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Color() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Color>(),
        4usize,
        concat!("Size of: ", stringify!(Gdiplus_Color))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Color>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_Color))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Color>())).Argb as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Color),
            "::",
            stringify!(Argb)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_ENHMETAHEADER3() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_ENHMETAHEADER3>(),
        88usize,
        concat!("Size of: ", stringify!(Gdiplus_ENHMETAHEADER3))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_ENHMETAHEADER3>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_ENHMETAHEADER3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).iType as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(iType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nSize as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).rclBounds as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(rclBounds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).rclFrame as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(rclFrame)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).dSignature as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(dSignature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nVersion as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nVersion)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nBytes as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nBytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nRecords as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nRecords)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nHandles as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nHandles)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).sReserved as *const _ as usize
        },
        58usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(sReserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nDescription as *const _ as usize
        },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nDescription)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).offDescription as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(offDescription)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).nPalEntries as *const _ as usize
        },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(nPalEntries)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).szlDevice as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(szlDevice)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ENHMETAHEADER3>())).szlMillimeters as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ENHMETAHEADER3),
            "::",
            stringify!(szlMillimeters)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_PWMFRect16() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_PWMFRect16>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_PWMFRect16))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_PWMFRect16>(),
        2usize,
        concat!("Alignment of ", stringify!(Gdiplus_PWMFRect16))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PWMFRect16>())).Left as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PWMFRect16),
            "::",
            stringify!(Left)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PWMFRect16>())).Top as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PWMFRect16),
            "::",
            stringify!(Top)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PWMFRect16>())).Right as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PWMFRect16),
            "::",
            stringify!(Right)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PWMFRect16>())).Bottom as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PWMFRect16),
            "::",
            stringify!(Bottom)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_WmfPlaceableFileHeader() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_WmfPlaceableFileHeader>(),
        22usize,
        concat!("Size of: ", stringify!(Gdiplus_WmfPlaceableFileHeader))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_WmfPlaceableFileHeader>(),
        2usize,
        concat!("Alignment of ", stringify!(Gdiplus_WmfPlaceableFileHeader))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).Key as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(Key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).Hmf as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(Hmf)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).BoundingBox as *const _
                as usize
        },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(BoundingBox)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).Inch as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(Inch)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).Reserved as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(Reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_WmfPlaceableFileHeader>())).Checksum as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_WmfPlaceableFileHeader),
            "::",
            stringify!(Checksum)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_MetafileHeader__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_MetafileHeader__bindgen_ty_1>(),
        88usize,
        concat!(
            "Size of: ",
            stringify!(Gdiplus_MetafileHeader__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_MetafileHeader__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(Gdiplus_MetafileHeader__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader__bindgen_ty_1>())).WmfHeader as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader__bindgen_ty_1),
            "::",
            stringify!(WmfHeader)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader__bindgen_ty_1>())).EmfHeader as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader__bindgen_ty_1),
            "::",
            stringify!(EmfHeader)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_MetafileHeader() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_MetafileHeader>(),
        140usize,
        concat!("Size of: ", stringify!(Gdiplus_MetafileHeader))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_MetafileHeader>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_MetafileHeader))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Type as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Version as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Version)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).EmfPlusFlags as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(EmfPlusFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).DpiX as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(DpiX)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).DpiY as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(DpiY)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).X as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(X)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Y as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Y)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Width as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).Height as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(Height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).EmfPlusHeaderSize as *const _
                as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(EmfPlusHeaderSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).LogicalDpiX as *const _ as usize
        },
        132usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(LogicalDpiX)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_MetafileHeader>())).LogicalDpiY as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_MetafileHeader),
            "::",
            stringify!(LogicalDpiY)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_ImageCodecInfo() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_ImageCodecInfo>(),
        104usize,
        concat!("Size of: ", stringify!(Gdiplus_ImageCodecInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_ImageCodecInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_ImageCodecInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).Clsid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(Clsid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).FormatID as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(FormatID)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).CodecName as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(CodecName)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).DllName as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(DllName)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).FormatDescription as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(FormatDescription)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).FilenameExtension as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(FilenameExtension)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).MimeType as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(MimeType)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).Flags as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(Flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).Version as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(Version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).SigCount as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(SigCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).SigSize as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(SigSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).SigPattern as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(SigPattern)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ImageCodecInfo>())).SigMask as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ImageCodecInfo),
            "::",
            stringify!(SigMask)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_BitmapData() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_BitmapData>(),
        32usize,
        concat!("Size of: ", stringify!(Gdiplus_BitmapData))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_BitmapData>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_BitmapData))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).Width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(Width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).Height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(Height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).Stride as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(Stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).PixelFormat as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(PixelFormat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).Scan0 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(Scan0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_BitmapData>())).Reserved as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_BitmapData),
            "::",
            stringify!(Reserved)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_EncoderParameter() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_EncoderParameter>(),
        32usize,
        concat!("Size of: ", stringify!(Gdiplus_EncoderParameter))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_EncoderParameter>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_EncoderParameter))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_EncoderParameter>())).Guid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameter),
            "::",
            stringify!(Guid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_EncoderParameter>())).NumberOfValues as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameter),
            "::",
            stringify!(NumberOfValues)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_EncoderParameter>())).Type as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameter),
            "::",
            stringify!(Type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_EncoderParameter>())).Value as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameter),
            "::",
            stringify!(Value)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_EncoderParameters() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_EncoderParameters>(),
        40usize,
        concat!("Size of: ", stringify!(Gdiplus_EncoderParameters))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_EncoderParameters>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_EncoderParameters))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_EncoderParameters>())).Count as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameters),
            "::",
            stringify!(Count)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_EncoderParameters>())).Parameter as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_EncoderParameters),
            "::",
            stringify!(Parameter)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_PropertyItem() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_PropertyItem>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_PropertyItem))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_PropertyItem>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_PropertyItem))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PropertyItem>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PropertyItem),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PropertyItem>())).length as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PropertyItem),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PropertyItem>())).type_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PropertyItem),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_PropertyItem>())).value as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_PropertyItem),
            "::",
            stringify!(value)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_ColorMatrix() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_ColorMatrix>(),
        100usize,
        concat!("Size of: ", stringify!(Gdiplus_ColorMatrix))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_ColorMatrix>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_ColorMatrix))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorMatrix>())).m as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorMatrix),
            "::",
            stringify!(m)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_ColorMap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_ColorMap>(),
        8usize,
        concat!("Size of: ", stringify!(Gdiplus_ColorMap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_ColorMap>(),
        4usize,
        concat!("Alignment of ", stringify!(Gdiplus_ColorMap))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorMap>())).oldColor as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorMap),
            "::",
            stringify!(oldColor)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_ColorMap>())).newColor as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_ColorMap),
            "::",
            stringify!(newColor)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpGraphics() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpGraphics>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpGraphics))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpGraphics>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpGraphics))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpBrush() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpBrush>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpBrush))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpBrush>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpBrush))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpTexture() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpTexture>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpTexture))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpTexture>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpTexture))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpSolidFill() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpSolidFill>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpSolidFill))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpSolidFill>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpSolidFill))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpLineGradient() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpLineGradient>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpLineGradient))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpLineGradient>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpLineGradient))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpPathGradient() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpPathGradient>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpPathGradient))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpPathGradient>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpPathGradient))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpHatch() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpHatch>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpHatch))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpHatch>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpHatch))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpPen() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpPen>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpPen))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpPen>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpPen))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpCustomLineCap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpCustomLineCap>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpCustomLineCap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpCustomLineCap>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpCustomLineCap))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpAdjustableArrowCap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpAdjustableArrowCap>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpAdjustableArrowCap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpAdjustableArrowCap>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpAdjustableArrowCap))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpImage() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpImage>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpImage))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpImage>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpImage))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpBitmap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpBitmap>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpBitmap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpBitmap>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpBitmap))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpMetafile() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpMetafile>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpMetafile))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpMetafile>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpMetafile))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpImageAttributes() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpImageAttributes>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpImageAttributes))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpImageAttributes>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpImageAttributes))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpPath() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpPath>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpPath))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpPath>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpPath))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpRegion() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpRegion>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpRegion))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpRegion>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpRegion))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpPathIterator() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpPathIterator>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpPathIterator))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpPathIterator>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpPathIterator))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpFontFamily() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpFontFamily>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpFontFamily))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpFontFamily>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpFontFamily))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpFont() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpFont>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpFont))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpFont>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpFont))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpStringFormat() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpStringFormat>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpStringFormat))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpStringFormat>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpStringFormat))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GpFontCollection() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GpFontCollection>(),
        1usize,
        concat!("Size of: ", stringify!(Gdiplus_GpFontCollection))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GpFontCollection>(),
        1usize,
        concat!("Alignment of ", stringify!(Gdiplus_GpFontCollection))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Region() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Region>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Region))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Region>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Region))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Region>())).nativeRegion as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Region),
            "::",
            stringify!(nativeRegion)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Region>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Region),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_FontFamily() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_FontFamily>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_FontFamily))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_FontFamily>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_FontFamily))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_FontFamily>())).nativeFamily as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_FontFamily),
            "::",
            stringify!(nativeFamily)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_FontFamily>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_FontFamily),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Font() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Font>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Font))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Font>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Font))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Font>())).nativeFont as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Font),
            "::",
            stringify!(nativeFont)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Font>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Font),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_FontCollection() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_FontCollection>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_FontCollection))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_FontCollection>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_FontCollection))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_FontCollection>())).nativeFontCollection as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_FontCollection),
            "::",
            stringify!(nativeFontCollection)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_FontCollection>())).lastResult as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_FontCollection),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_InstalledFontCollection() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_InstalledFontCollection>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_InstalledFontCollection))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_InstalledFontCollection>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_InstalledFontCollection))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_PrivateFontCollection() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_PrivateFontCollection>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_PrivateFontCollection))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_PrivateFontCollection>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_PrivateFontCollection))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Image() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Image>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_Image))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Image>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Image))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Image>())).nativeImage as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Image),
            "::",
            stringify!(nativeImage)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Image>())).lastResult as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Image),
            "::",
            stringify!(lastResult)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Image>())).loadStatus as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Image),
            "::",
            stringify!(loadStatus)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Bitmap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Bitmap>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_Bitmap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Bitmap>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Bitmap))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_CustomLineCap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_CustomLineCap>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_CustomLineCap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_CustomLineCap>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_CustomLineCap))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_CustomLineCap>())).nativeCap as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CustomLineCap),
            "::",
            stringify!(nativeCap)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_CustomLineCap>())).lastResult as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CustomLineCap),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_CachedBitmap() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_CachedBitmap>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_CachedBitmap))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_CachedBitmap>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_CachedBitmap))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_CachedBitmap>())).nativeCachedBitmap as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CachedBitmap),
            "::",
            stringify!(nativeCachedBitmap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_CachedBitmap>())).lastResult as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_CachedBitmap),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Metafile() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Metafile>(),
        24usize,
        concat!("Size of: ", stringify!(Gdiplus_Metafile))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Metafile>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Metafile))
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Matrix() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Matrix>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Matrix))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Matrix>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Matrix))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Matrix>())).nativeMatrix as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Matrix),
            "::",
            stringify!(nativeMatrix)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Matrix>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Matrix),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Pen() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Pen>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Pen))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Pen>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Pen))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Pen>())).nativePen as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Pen),
            "::",
            stringify!(nativePen)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Pen>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Pen),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_StringFormat() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_StringFormat>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_StringFormat))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_StringFormat>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_StringFormat))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<Gdiplus_StringFormat>())).nativeFormat as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_StringFormat),
            "::",
            stringify!(nativeFormat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_StringFormat>())).lastError as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_StringFormat),
            "::",
            stringify!(lastError)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_GraphicsPath() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_GraphicsPath>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_GraphicsPath))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_GraphicsPath>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_GraphicsPath))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_GraphicsPath>())).nativePath as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GraphicsPath),
            "::",
            stringify!(nativePath)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_GraphicsPath>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_GraphicsPath),
            "::",
            stringify!(lastResult)
        )
    );
}
#[test]
fn bindgen_test_layout_Gdiplus_Graphics() {
    assert_eq!(
        ::std::mem::size_of::<Gdiplus_Graphics>(),
        16usize,
        concat!("Size of: ", stringify!(Gdiplus_Graphics))
    );
    assert_eq!(
        ::std::mem::align_of::<Gdiplus_Graphics>(),
        8usize,
        concat!("Alignment of ", stringify!(Gdiplus_Graphics))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Graphics>())).nativeGraphics as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Graphics),
            "::",
            stringify!(nativeGraphics)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Gdiplus_Graphics>())).lastResult as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Gdiplus_Graphics),
            "::",
            stringify!(lastResult)
        )
    );
}