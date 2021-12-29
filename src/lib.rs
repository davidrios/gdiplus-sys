#![cfg(windows)]
#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![cfg_attr(not(test), no_std)]

use core::mem::MaybeUninit;

use winapi::ctypes::{c_int, c_void};
use winapi::shared::basetsd::{INT16, UINT16, UINT32, UINT_PTR, ULONG_PTR};
use winapi::shared::guiddef::{CLSID, GUID};
use winapi::shared::minwindef::{
    BOOL, BYTE, DWORD, HINSTANCE, HMETAFILE, HRGN, LPBYTE, UINT, WORD,
};
use winapi::shared::ntdef::{CHAR, HANDLE, INT, LANGID, LPWSTR, ULONG, WCHAR};
use winapi::shared::windef::{
    HBITMAP, HDC, HENHMETAFILE, HFONT, HICON, HPALETTE, HWND, RECTL, SIZEL,
};
use winapi::shared::wtypes::PROPID;
use winapi::um::commctrl::IStream;
use winapi::um::wingdi::{BITMAPINFO, LOGFONTA, LOGFONTW, METAHEADER};
use winapi::vc::vcruntime::size_t;

#[cfg(test)]
mod bindgen_tests;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IDirectDrawSurface7 {
    _unused: [u8; 0],
}
extern "C" {
    #[link_name = "\u{1}GdipAlloc"]
    pub fn Gdiplus_DllExports_GdipAlloc(size: size_t) -> *mut c_void;
}
extern "C" {
    #[link_name = "\u{1}GdipFree"]
    pub fn Gdiplus_DllExports_GdipFree(ptr: *mut c_void);
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePath"]
    pub fn Gdiplus_DllExports_GdipCreatePath(
        brushMode: Gdiplus_GpFillMode,
        path: *mut *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePath2"]
    pub fn Gdiplus_DllExports_GdipCreatePath2(
        arg1: *const Gdiplus_GpPointF,
        arg2: *const BYTE,
        arg3: INT,
        arg4: Gdiplus_GpFillMode,
        path: *mut *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePath2I"]
    pub fn Gdiplus_DllExports_GdipCreatePath2I(
        arg1: *const Gdiplus_GpPoint,
        arg2: *const BYTE,
        arg3: INT,
        arg4: Gdiplus_GpFillMode,
        path: *mut *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipClonePath"]
    pub fn Gdiplus_DllExports_GdipClonePath(
        path: *mut Gdiplus_GpPath,
        clonePath: *mut *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeletePath"]
    pub fn Gdiplus_DllExports_GdipDeletePath(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetPath"]
    pub fn Gdiplus_DllExports_GdipResetPath(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPointCount"]
    pub fn Gdiplus_DllExports_GdipGetPointCount(
        path: *mut Gdiplus_GpPath,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathTypes"]
    pub fn Gdiplus_DllExports_GdipGetPathTypes(
        path: *mut Gdiplus_GpPath,
        types: *mut BYTE,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathPoints"]
    pub fn Gdiplus_DllExports_GdipGetPathPoints(
        arg1: *mut Gdiplus_GpPath,
        points: *mut Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathPointsI"]
    pub fn Gdiplus_DllExports_GdipGetPathPointsI(
        arg1: *mut Gdiplus_GpPath,
        points: *mut Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathFillMode"]
    pub fn Gdiplus_DllExports_GdipGetPathFillMode(
        path: *mut Gdiplus_GpPath,
        fillmode: *mut Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathFillMode"]
    pub fn Gdiplus_DllExports_GdipSetPathFillMode(
        path: *mut Gdiplus_GpPath,
        fillmode: Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathData"]
    pub fn Gdiplus_DllExports_GdipGetPathData(
        path: *mut Gdiplus_GpPath,
        pathData: *mut Gdiplus_GpPathData,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipStartPathFigure"]
    pub fn Gdiplus_DllExports_GdipStartPathFigure(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipClosePathFigure"]
    pub fn Gdiplus_DllExports_GdipClosePathFigure(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipClosePathFigures"]
    pub fn Gdiplus_DllExports_GdipClosePathFigures(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathMarker"]
    pub fn Gdiplus_DllExports_GdipSetPathMarker(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipClearPathMarkers"]
    pub fn Gdiplus_DllExports_GdipClearPathMarkers(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipReversePath"]
    pub fn Gdiplus_DllExports_GdipReversePath(path: *mut Gdiplus_GpPath) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathLastPoint"]
    pub fn Gdiplus_DllExports_GdipGetPathLastPoint(
        path: *mut Gdiplus_GpPath,
        lastPoint: *mut Gdiplus_GpPointF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathLine"]
    pub fn Gdiplus_DllExports_GdipAddPathLine(
        path: *mut Gdiplus_GpPath,
        x1: Gdiplus_REAL,
        y1: Gdiplus_REAL,
        x2: Gdiplus_REAL,
        y2: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathLine2"]
    pub fn Gdiplus_DllExports_GdipAddPathLine2(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathArc"]
    pub fn Gdiplus_DllExports_GdipAddPathArc(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathBezier"]
    pub fn Gdiplus_DllExports_GdipAddPathBezier(
        path: *mut Gdiplus_GpPath,
        x1: Gdiplus_REAL,
        y1: Gdiplus_REAL,
        x2: Gdiplus_REAL,
        y2: Gdiplus_REAL,
        x3: Gdiplus_REAL,
        y3: Gdiplus_REAL,
        x4: Gdiplus_REAL,
        y4: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathBeziers"]
    pub fn Gdiplus_DllExports_GdipAddPathBeziers(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurve"]
    pub fn Gdiplus_DllExports_GdipAddPathCurve(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurve2"]
    pub fn Gdiplus_DllExports_GdipAddPathCurve2(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurve3"]
    pub fn Gdiplus_DllExports_GdipAddPathCurve3(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathClosedCurve"]
    pub fn Gdiplus_DllExports_GdipAddPathClosedCurve(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathClosedCurve2"]
    pub fn Gdiplus_DllExports_GdipAddPathClosedCurve2(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathRectangle"]
    pub fn Gdiplus_DllExports_GdipAddPathRectangle(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathRectangles"]
    pub fn Gdiplus_DllExports_GdipAddPathRectangles(
        path: *mut Gdiplus_GpPath,
        rects: *const Gdiplus_GpRectF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathEllipse"]
    pub fn Gdiplus_DllExports_GdipAddPathEllipse(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathPie"]
    pub fn Gdiplus_DllExports_GdipAddPathPie(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathPolygon"]
    pub fn Gdiplus_DllExports_GdipAddPathPolygon(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathPath"]
    pub fn Gdiplus_DllExports_GdipAddPathPath(
        path: *mut Gdiplus_GpPath,
        addingPath: *const Gdiplus_GpPath,
        connect: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathString"]
    pub fn Gdiplus_DllExports_GdipAddPathString(
        path: *mut Gdiplus_GpPath,
        string: *const WCHAR,
        length: INT,
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        emSize: Gdiplus_REAL,
        layoutRect: *const Gdiplus_RectF,
        format: *const Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathStringI"]
    pub fn Gdiplus_DllExports_GdipAddPathStringI(
        path: *mut Gdiplus_GpPath,
        string: *const WCHAR,
        length: INT,
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        emSize: Gdiplus_REAL,
        layoutRect: *const Gdiplus_Rect,
        format: *const Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathLineI"]
    pub fn Gdiplus_DllExports_GdipAddPathLineI(
        path: *mut Gdiplus_GpPath,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathLine2I"]
    pub fn Gdiplus_DllExports_GdipAddPathLine2I(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathArcI"]
    pub fn Gdiplus_DllExports_GdipAddPathArcI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathBezierI"]
    pub fn Gdiplus_DllExports_GdipAddPathBezierI(
        path: *mut Gdiplus_GpPath,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
        x3: INT,
        y3: INT,
        x4: INT,
        y4: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathBeziersI"]
    pub fn Gdiplus_DllExports_GdipAddPathBeziersI(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurveI"]
    pub fn Gdiplus_DllExports_GdipAddPathCurveI(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurve2I"]
    pub fn Gdiplus_DllExports_GdipAddPathCurve2I(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathCurve3I"]
    pub fn Gdiplus_DllExports_GdipAddPathCurve3I(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathClosedCurveI"]
    pub fn Gdiplus_DllExports_GdipAddPathClosedCurveI(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathClosedCurve2I"]
    pub fn Gdiplus_DllExports_GdipAddPathClosedCurve2I(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathRectangleI"]
    pub fn Gdiplus_DllExports_GdipAddPathRectangleI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathRectanglesI"]
    pub fn Gdiplus_DllExports_GdipAddPathRectanglesI(
        path: *mut Gdiplus_GpPath,
        rects: *const Gdiplus_GpRect,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathEllipseI"]
    pub fn Gdiplus_DllExports_GdipAddPathEllipseI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathPieI"]
    pub fn Gdiplus_DllExports_GdipAddPathPieI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipAddPathPolygonI"]
    pub fn Gdiplus_DllExports_GdipAddPathPolygonI(
        path: *mut Gdiplus_GpPath,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFlattenPath"]
    pub fn Gdiplus_DllExports_GdipFlattenPath(
        path: *mut Gdiplus_GpPath,
        matrix: *mut Gdiplus_GpMatrix,
        flatness: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipWindingModeOutline"]
    pub fn Gdiplus_DllExports_GdipWindingModeOutline(
        path: *mut Gdiplus_GpPath,
        matrix: *mut Gdiplus_GpMatrix,
        flatness: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipWidenPath"]
    pub fn Gdiplus_DllExports_GdipWidenPath(
        nativePath: *mut Gdiplus_GpPath,
        pen: *mut Gdiplus_GpPen,
        matrix: *mut Gdiplus_GpMatrix,
        flatness: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipWarpPath"]
    pub fn Gdiplus_DllExports_GdipWarpPath(
        path: *mut Gdiplus_GpPath,
        matrix: *mut Gdiplus_GpMatrix,
        points: *const Gdiplus_GpPointF,
        count: INT,
        srcx: Gdiplus_REAL,
        srcy: Gdiplus_REAL,
        srcwidth: Gdiplus_REAL,
        srcheight: Gdiplus_REAL,
        warpMode: Gdiplus_WarpMode,
        flatness: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformPath"]
    pub fn Gdiplus_DllExports_GdipTransformPath(
        path: *mut Gdiplus_GpPath,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathWorldBounds"]
    pub fn Gdiplus_DllExports_GdipGetPathWorldBounds(
        path: *mut Gdiplus_GpPath,
        bounds: *mut Gdiplus_GpRectF,
        matrix: *const Gdiplus_GpMatrix,
        pen: *const Gdiplus_GpPen,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathWorldBoundsI"]
    pub fn Gdiplus_DllExports_GdipGetPathWorldBoundsI(
        path: *mut Gdiplus_GpPath,
        bounds: *mut Gdiplus_GpRect,
        matrix: *const Gdiplus_GpMatrix,
        pen: *const Gdiplus_GpPen,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisiblePathPoint"]
    pub fn Gdiplus_DllExports_GdipIsVisiblePathPoint(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisiblePathPointI"]
    pub fn Gdiplus_DllExports_GdipIsVisiblePathPointI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsOutlineVisiblePathPoint"]
    pub fn Gdiplus_DllExports_GdipIsOutlineVisiblePathPoint(
        path: *mut Gdiplus_GpPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        pen: *mut Gdiplus_GpPen,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsOutlineVisiblePathPointI"]
    pub fn Gdiplus_DllExports_GdipIsOutlineVisiblePathPointI(
        path: *mut Gdiplus_GpPath,
        x: INT,
        y: INT,
        pen: *mut Gdiplus_GpPen,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePathIter"]
    pub fn Gdiplus_DllExports_GdipCreatePathIter(
        iterator: *mut *mut Gdiplus_GpPathIterator,
        path: *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeletePathIter"]
    pub fn Gdiplus_DllExports_GdipDeletePathIter(
        iterator: *mut Gdiplus_GpPathIterator,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterNextSubpath"]
    pub fn Gdiplus_DllExports_GdipPathIterNextSubpath(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        startIndex: *mut INT,
        endIndex: *mut INT,
        isClosed: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterNextSubpathPath"]
    pub fn Gdiplus_DllExports_GdipPathIterNextSubpathPath(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        path: *mut Gdiplus_GpPath,
        isClosed: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterNextPathType"]
    pub fn Gdiplus_DllExports_GdipPathIterNextPathType(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        pathType: *mut BYTE,
        startIndex: *mut INT,
        endIndex: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterNextMarker"]
    pub fn Gdiplus_DllExports_GdipPathIterNextMarker(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        startIndex: *mut INT,
        endIndex: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterNextMarkerPath"]
    pub fn Gdiplus_DllExports_GdipPathIterNextMarkerPath(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        path: *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterGetCount"]
    pub fn Gdiplus_DllExports_GdipPathIterGetCount(
        iterator: *mut Gdiplus_GpPathIterator,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterGetSubpathCount"]
    pub fn Gdiplus_DllExports_GdipPathIterGetSubpathCount(
        iterator: *mut Gdiplus_GpPathIterator,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterIsValid"]
    pub fn Gdiplus_DllExports_GdipPathIterIsValid(
        iterator: *mut Gdiplus_GpPathIterator,
        valid: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterHasCurve"]
    pub fn Gdiplus_DllExports_GdipPathIterHasCurve(
        iterator: *mut Gdiplus_GpPathIterator,
        hasCurve: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterRewind"]
    pub fn Gdiplus_DllExports_GdipPathIterRewind(
        iterator: *mut Gdiplus_GpPathIterator,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterEnumerate"]
    pub fn Gdiplus_DllExports_GdipPathIterEnumerate(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        points: *mut Gdiplus_GpPointF,
        types: *mut BYTE,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPathIterCopyData"]
    pub fn Gdiplus_DllExports_GdipPathIterCopyData(
        iterator: *mut Gdiplus_GpPathIterator,
        resultCount: *mut INT,
        points: *mut Gdiplus_GpPointF,
        types: *mut BYTE,
        startIndex: INT,
        endIndex: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMatrix"]
    pub fn Gdiplus_DllExports_GdipCreateMatrix(
        matrix: *mut *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMatrix2"]
    pub fn Gdiplus_DllExports_GdipCreateMatrix2(
        m11: Gdiplus_REAL,
        m12: Gdiplus_REAL,
        m21: Gdiplus_REAL,
        m22: Gdiplus_REAL,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        matrix: *mut *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMatrix3"]
    pub fn Gdiplus_DllExports_GdipCreateMatrix3(
        rect: *const Gdiplus_GpRectF,
        dstplg: *const Gdiplus_GpPointF,
        matrix: *mut *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMatrix3I"]
    pub fn Gdiplus_DllExports_GdipCreateMatrix3I(
        rect: *const Gdiplus_GpRect,
        dstplg: *const Gdiplus_GpPoint,
        matrix: *mut *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneMatrix"]
    pub fn Gdiplus_DllExports_GdipCloneMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        cloneMatrix: *mut *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteMatrix"]
    pub fn Gdiplus_DllExports_GdipDeleteMatrix(matrix: *mut Gdiplus_GpMatrix) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetMatrixElements"]
    pub fn Gdiplus_DllExports_GdipSetMatrixElements(
        matrix: *mut Gdiplus_GpMatrix,
        m11: Gdiplus_REAL,
        m12: Gdiplus_REAL,
        m21: Gdiplus_REAL,
        m22: Gdiplus_REAL,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyMatrix"]
    pub fn Gdiplus_DllExports_GdipMultiplyMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        matrix2: *mut Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateMatrix"]
    pub fn Gdiplus_DllExports_GdipTranslateMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        offsetX: Gdiplus_REAL,
        offsetY: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScaleMatrix"]
    pub fn Gdiplus_DllExports_GdipScaleMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        scaleX: Gdiplus_REAL,
        scaleY: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotateMatrix"]
    pub fn Gdiplus_DllExports_GdipRotateMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipShearMatrix"]
    pub fn Gdiplus_DllExports_GdipShearMatrix(
        matrix: *mut Gdiplus_GpMatrix,
        shearX: Gdiplus_REAL,
        shearY: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipInvertMatrix"]
    pub fn Gdiplus_DllExports_GdipInvertMatrix(matrix: *mut Gdiplus_GpMatrix) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformMatrixPoints"]
    pub fn Gdiplus_DllExports_GdipTransformMatrixPoints(
        matrix: *mut Gdiplus_GpMatrix,
        pts: *mut Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformMatrixPointsI"]
    pub fn Gdiplus_DllExports_GdipTransformMatrixPointsI(
        matrix: *mut Gdiplus_GpMatrix,
        pts: *mut Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipVectorTransformMatrixPoints"]
    pub fn Gdiplus_DllExports_GdipVectorTransformMatrixPoints(
        matrix: *mut Gdiplus_GpMatrix,
        pts: *mut Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipVectorTransformMatrixPointsI"]
    pub fn Gdiplus_DllExports_GdipVectorTransformMatrixPointsI(
        matrix: *mut Gdiplus_GpMatrix,
        pts: *mut Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMatrixElements"]
    pub fn Gdiplus_DllExports_GdipGetMatrixElements(
        matrix: *const Gdiplus_GpMatrix,
        matrixOut: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsMatrixInvertible"]
    pub fn Gdiplus_DllExports_GdipIsMatrixInvertible(
        matrix: *const Gdiplus_GpMatrix,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsMatrixIdentity"]
    pub fn Gdiplus_DllExports_GdipIsMatrixIdentity(
        matrix: *const Gdiplus_GpMatrix,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsMatrixEqual"]
    pub fn Gdiplus_DllExports_GdipIsMatrixEqual(
        matrix: *const Gdiplus_GpMatrix,
        matrix2: *const Gdiplus_GpMatrix,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegion"]
    pub fn Gdiplus_DllExports_GdipCreateRegion(
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegionRect"]
    pub fn Gdiplus_DllExports_GdipCreateRegionRect(
        rect: *const Gdiplus_GpRectF,
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegionRectI"]
    pub fn Gdiplus_DllExports_GdipCreateRegionRectI(
        rect: *const Gdiplus_GpRect,
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegionPath"]
    pub fn Gdiplus_DllExports_GdipCreateRegionPath(
        path: *mut Gdiplus_GpPath,
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegionRgnData"]
    pub fn Gdiplus_DllExports_GdipCreateRegionRgnData(
        regionData: *const BYTE,
        size: INT,
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateRegionHrgn"]
    pub fn Gdiplus_DllExports_GdipCreateRegionHrgn(
        hRgn: HRGN,
        region: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneRegion"]
    pub fn Gdiplus_DllExports_GdipCloneRegion(
        region: *mut Gdiplus_GpRegion,
        cloneRegion: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteRegion"]
    pub fn Gdiplus_DllExports_GdipDeleteRegion(region: *mut Gdiplus_GpRegion) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetInfinite"]
    pub fn Gdiplus_DllExports_GdipSetInfinite(region: *mut Gdiplus_GpRegion) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetEmpty"]
    pub fn Gdiplus_DllExports_GdipSetEmpty(region: *mut Gdiplus_GpRegion) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCombineRegionRect"]
    pub fn Gdiplus_DllExports_GdipCombineRegionRect(
        region: *mut Gdiplus_GpRegion,
        rect: *const Gdiplus_GpRectF,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCombineRegionRectI"]
    pub fn Gdiplus_DllExports_GdipCombineRegionRectI(
        region: *mut Gdiplus_GpRegion,
        rect: *const Gdiplus_GpRect,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCombineRegionPath"]
    pub fn Gdiplus_DllExports_GdipCombineRegionPath(
        region: *mut Gdiplus_GpRegion,
        path: *mut Gdiplus_GpPath,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCombineRegionRegion"]
    pub fn Gdiplus_DllExports_GdipCombineRegionRegion(
        region: *mut Gdiplus_GpRegion,
        region2: *mut Gdiplus_GpRegion,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateRegion"]
    pub fn Gdiplus_DllExports_GdipTranslateRegion(
        region: *mut Gdiplus_GpRegion,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateRegionI"]
    pub fn Gdiplus_DllExports_GdipTranslateRegionI(
        region: *mut Gdiplus_GpRegion,
        dx: INT,
        dy: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformRegion"]
    pub fn Gdiplus_DllExports_GdipTransformRegion(
        region: *mut Gdiplus_GpRegion,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionBounds"]
    pub fn Gdiplus_DllExports_GdipGetRegionBounds(
        region: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionBoundsI"]
    pub fn Gdiplus_DllExports_GdipGetRegionBoundsI(
        region: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRect,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionHRgn"]
    pub fn Gdiplus_DllExports_GdipGetRegionHRgn(
        region: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        hRgn: *mut HRGN,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsEmptyRegion"]
    pub fn Gdiplus_DllExports_GdipIsEmptyRegion(
        region: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsInfiniteRegion"]
    pub fn Gdiplus_DllExports_GdipIsInfiniteRegion(
        region: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsEqualRegion"]
    pub fn Gdiplus_DllExports_GdipIsEqualRegion(
        region: *mut Gdiplus_GpRegion,
        region2: *mut Gdiplus_GpRegion,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionDataSize"]
    pub fn Gdiplus_DllExports_GdipGetRegionDataSize(
        region: *mut Gdiplus_GpRegion,
        bufferSize: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionData"]
    pub fn Gdiplus_DllExports_GdipGetRegionData(
        region: *mut Gdiplus_GpRegion,
        buffer: *mut BYTE,
        bufferSize: UINT,
        sizeFilled: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRegionPoint"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRegionPoint(
        region: *mut Gdiplus_GpRegion,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRegionPointI"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRegionPointI(
        region: *mut Gdiplus_GpRegion,
        x: INT,
        y: INT,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRegionRect"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRegionRect(
        region: *mut Gdiplus_GpRegion,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRegionRectI"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRegionRectI(
        region: *mut Gdiplus_GpRegion,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionScansCount"]
    pub fn Gdiplus_DllExports_GdipGetRegionScansCount(
        region: *mut Gdiplus_GpRegion,
        count: *mut UINT,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionScans"]
    pub fn Gdiplus_DllExports_GdipGetRegionScans(
        region: *mut Gdiplus_GpRegion,
        rects: *mut Gdiplus_GpRectF,
        count: *mut INT,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRegionScansI"]
    pub fn Gdiplus_DllExports_GdipGetRegionScansI(
        region: *mut Gdiplus_GpRegion,
        rects: *mut Gdiplus_GpRect,
        count: *mut INT,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneBrush"]
    pub fn Gdiplus_DllExports_GdipCloneBrush(
        brush: *mut Gdiplus_GpBrush,
        cloneBrush: *mut *mut Gdiplus_GpBrush,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteBrush"]
    pub fn Gdiplus_DllExports_GdipDeleteBrush(brush: *mut Gdiplus_GpBrush) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetBrushType"]
    pub fn Gdiplus_DllExports_GdipGetBrushType(
        brush: *mut Gdiplus_GpBrush,
        type_: *mut Gdiplus_GpBrushType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateHatchBrush"]
    pub fn Gdiplus_DllExports_GdipCreateHatchBrush(
        hatchstyle: Gdiplus_GpHatchStyle,
        forecol: Gdiplus_ARGB,
        backcol: Gdiplus_ARGB,
        brush: *mut *mut Gdiplus_GpHatch,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetHatchStyle"]
    pub fn Gdiplus_DllExports_GdipGetHatchStyle(
        brush: *mut Gdiplus_GpHatch,
        hatchstyle: *mut Gdiplus_GpHatchStyle,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetHatchForegroundColor"]
    pub fn Gdiplus_DllExports_GdipGetHatchForegroundColor(
        brush: *mut Gdiplus_GpHatch,
        forecol: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetHatchBackgroundColor"]
    pub fn Gdiplus_DllExports_GdipGetHatchBackgroundColor(
        brush: *mut Gdiplus_GpHatch,
        backcol: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateTexture"]
    pub fn Gdiplus_DllExports_GdipCreateTexture(
        image: *mut Gdiplus_GpImage,
        wrapmode: Gdiplus_GpWrapMode,
        texture: *mut *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateTexture2"]
    pub fn Gdiplus_DllExports_GdipCreateTexture2(
        image: *mut Gdiplus_GpImage,
        wrapmode: Gdiplus_GpWrapMode,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        texture: *mut *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateTextureIA"]
    pub fn Gdiplus_DllExports_GdipCreateTextureIA(
        image: *mut Gdiplus_GpImage,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        texture: *mut *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateTexture2I"]
    pub fn Gdiplus_DllExports_GdipCreateTexture2I(
        image: *mut Gdiplus_GpImage,
        wrapmode: Gdiplus_GpWrapMode,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        texture: *mut *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateTextureIAI"]
    pub fn Gdiplus_DllExports_GdipCreateTextureIAI(
        image: *mut Gdiplus_GpImage,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        texture: *mut *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetTextureTransform"]
    pub fn Gdiplus_DllExports_GdipGetTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetTextureTransform"]
    pub fn Gdiplus_DllExports_GdipSetTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        matrix: *const Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetTextureTransform"]
    pub fn Gdiplus_DllExports_GdipResetTextureTransform(
        brush: *mut Gdiplus_GpTexture,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyTextureTransform"]
    pub fn Gdiplus_DllExports_GdipMultiplyTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        matrix: *const Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateTextureTransform"]
    pub fn Gdiplus_DllExports_GdipTranslateTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScaleTextureTransform"]
    pub fn Gdiplus_DllExports_GdipScaleTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        sx: Gdiplus_REAL,
        sy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotateTextureTransform"]
    pub fn Gdiplus_DllExports_GdipRotateTextureTransform(
        brush: *mut Gdiplus_GpTexture,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetTextureWrapMode"]
    pub fn Gdiplus_DllExports_GdipSetTextureWrapMode(
        brush: *mut Gdiplus_GpTexture,
        wrapmode: Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetTextureWrapMode"]
    pub fn Gdiplus_DllExports_GdipGetTextureWrapMode(
        brush: *mut Gdiplus_GpTexture,
        wrapmode: *mut Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetTextureImage"]
    pub fn Gdiplus_DllExports_GdipGetTextureImage(
        brush: *mut Gdiplus_GpTexture,
        image: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateSolidFill"]
    pub fn Gdiplus_DllExports_GdipCreateSolidFill(
        color: Gdiplus_ARGB,
        brush: *mut *mut Gdiplus_GpSolidFill,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetSolidFillColor"]
    pub fn Gdiplus_DllExports_GdipSetSolidFillColor(
        brush: *mut Gdiplus_GpSolidFill,
        color: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetSolidFillColor"]
    pub fn Gdiplus_DllExports_GdipGetSolidFillColor(
        brush: *mut Gdiplus_GpSolidFill,
        color: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrush"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrush(
        point1: *const Gdiplus_GpPointF,
        point2: *const Gdiplus_GpPointF,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrushI"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrushI(
        point1: *const Gdiplus_GpPoint,
        point2: *const Gdiplus_GpPoint,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrushFromRect"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrushFromRect(
        rect: *const Gdiplus_GpRectF,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        mode: Gdiplus_LinearGradientMode,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrushFromRectI"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrushFromRectI(
        rect: *const Gdiplus_GpRect,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        mode: Gdiplus_LinearGradientMode,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrushFromRectWithAngle"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrushFromRectWithAngle(
        rect: *const Gdiplus_GpRectF,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        angle: Gdiplus_REAL,
        isAngleScalable: BOOL,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateLineBrushFromRectWithAngleI"]
    pub fn Gdiplus_DllExports_GdipCreateLineBrushFromRectWithAngleI(
        rect: *const Gdiplus_GpRect,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
        angle: Gdiplus_REAL,
        isAngleScalable: BOOL,
        wrapMode: Gdiplus_GpWrapMode,
        lineGradient: *mut *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineColors"]
    pub fn Gdiplus_DllExports_GdipSetLineColors(
        brush: *mut Gdiplus_GpLineGradient,
        color1: Gdiplus_ARGB,
        color2: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineColors"]
    pub fn Gdiplus_DllExports_GdipGetLineColors(
        brush: *mut Gdiplus_GpLineGradient,
        colors: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineRect"]
    pub fn Gdiplus_DllExports_GdipGetLineRect(
        brush: *mut Gdiplus_GpLineGradient,
        rect: *mut Gdiplus_GpRectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineRectI"]
    pub fn Gdiplus_DllExports_GdipGetLineRectI(
        brush: *mut Gdiplus_GpLineGradient,
        rect: *mut Gdiplus_GpRect,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineGammaCorrection"]
    pub fn Gdiplus_DllExports_GdipSetLineGammaCorrection(
        brush: *mut Gdiplus_GpLineGradient,
        useGammaCorrection: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineGammaCorrection"]
    pub fn Gdiplus_DllExports_GdipGetLineGammaCorrection(
        brush: *mut Gdiplus_GpLineGradient,
        useGammaCorrection: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineBlendCount"]
    pub fn Gdiplus_DllExports_GdipGetLineBlendCount(
        brush: *mut Gdiplus_GpLineGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineBlend"]
    pub fn Gdiplus_DllExports_GdipGetLineBlend(
        brush: *mut Gdiplus_GpLineGradient,
        blend: *mut Gdiplus_REAL,
        positions: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineBlend"]
    pub fn Gdiplus_DllExports_GdipSetLineBlend(
        brush: *mut Gdiplus_GpLineGradient,
        blend: *const Gdiplus_REAL,
        positions: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLinePresetBlendCount"]
    pub fn Gdiplus_DllExports_GdipGetLinePresetBlendCount(
        brush: *mut Gdiplus_GpLineGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLinePresetBlend"]
    pub fn Gdiplus_DllExports_GdipGetLinePresetBlend(
        brush: *mut Gdiplus_GpLineGradient,
        blend: *mut Gdiplus_ARGB,
        positions: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLinePresetBlend"]
    pub fn Gdiplus_DllExports_GdipSetLinePresetBlend(
        brush: *mut Gdiplus_GpLineGradient,
        blend: *const Gdiplus_ARGB,
        positions: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineSigmaBlend"]
    pub fn Gdiplus_DllExports_GdipSetLineSigmaBlend(
        brush: *mut Gdiplus_GpLineGradient,
        focus: Gdiplus_REAL,
        scale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineLinearBlend"]
    pub fn Gdiplus_DllExports_GdipSetLineLinearBlend(
        brush: *mut Gdiplus_GpLineGradient,
        focus: Gdiplus_REAL,
        scale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineWrapMode"]
    pub fn Gdiplus_DllExports_GdipSetLineWrapMode(
        brush: *mut Gdiplus_GpLineGradient,
        wrapmode: Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineWrapMode"]
    pub fn Gdiplus_DllExports_GdipGetLineWrapMode(
        brush: *mut Gdiplus_GpLineGradient,
        wrapmode: *mut Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineTransform"]
    pub fn Gdiplus_DllExports_GdipGetLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetLineTransform"]
    pub fn Gdiplus_DllExports_GdipSetLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        matrix: *const Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetLineTransform"]
    pub fn Gdiplus_DllExports_GdipResetLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyLineTransform"]
    pub fn Gdiplus_DllExports_GdipMultiplyLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        matrix: *const Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateLineTransform"]
    pub fn Gdiplus_DllExports_GdipTranslateLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScaleLineTransform"]
    pub fn Gdiplus_DllExports_GdipScaleLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        sx: Gdiplus_REAL,
        sy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotateLineTransform"]
    pub fn Gdiplus_DllExports_GdipRotateLineTransform(
        brush: *mut Gdiplus_GpLineGradient,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePathGradient"]
    pub fn Gdiplus_DllExports_GdipCreatePathGradient(
        points: *const Gdiplus_GpPointF,
        count: INT,
        wrapMode: Gdiplus_GpWrapMode,
        polyGradient: *mut *mut Gdiplus_GpPathGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePathGradientI"]
    pub fn Gdiplus_DllExports_GdipCreatePathGradientI(
        points: *const Gdiplus_GpPoint,
        count: INT,
        wrapMode: Gdiplus_GpWrapMode,
        polyGradient: *mut *mut Gdiplus_GpPathGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePathGradientFromPath"]
    pub fn Gdiplus_DllExports_GdipCreatePathGradientFromPath(
        path: *const Gdiplus_GpPath,
        polyGradient: *mut *mut Gdiplus_GpPathGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientCenterColor"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientCenterColor(
        brush: *mut Gdiplus_GpPathGradient,
        colors: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientCenterColor"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientCenterColor(
        brush: *mut Gdiplus_GpPathGradient,
        colors: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientSurroundColorsWithCount"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientSurroundColorsWithCount(
        brush: *mut Gdiplus_GpPathGradient,
        color: *mut Gdiplus_ARGB,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientSurroundColorsWithCount"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientSurroundColorsWithCount(
        brush: *mut Gdiplus_GpPathGradient,
        color: *const Gdiplus_ARGB,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientPath"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientPath(
        brush: *mut Gdiplus_GpPathGradient,
        path: *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientPath"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientPath(
        brush: *mut Gdiplus_GpPathGradient,
        path: *const Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientCenterPoint"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientCenterPoint(
        brush: *mut Gdiplus_GpPathGradient,
        points: *mut Gdiplus_GpPointF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientCenterPointI"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientCenterPointI(
        brush: *mut Gdiplus_GpPathGradient,
        points: *mut Gdiplus_GpPoint,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientCenterPoint"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientCenterPoint(
        brush: *mut Gdiplus_GpPathGradient,
        points: *const Gdiplus_GpPointF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientCenterPointI"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientCenterPointI(
        brush: *mut Gdiplus_GpPathGradient,
        points: *const Gdiplus_GpPoint,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientRect"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientRect(
        brush: *mut Gdiplus_GpPathGradient,
        rect: *mut Gdiplus_GpRectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientRectI"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientRectI(
        brush: *mut Gdiplus_GpPathGradient,
        rect: *mut Gdiplus_GpRect,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientPointCount"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientPointCount(
        brush: *mut Gdiplus_GpPathGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientSurroundColorCount"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientSurroundColorCount(
        brush: *mut Gdiplus_GpPathGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientGammaCorrection"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientGammaCorrection(
        brush: *mut Gdiplus_GpPathGradient,
        useGammaCorrection: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientGammaCorrection"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientGammaCorrection(
        brush: *mut Gdiplus_GpPathGradient,
        useGammaCorrection: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientBlendCount"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientBlendCount(
        brush: *mut Gdiplus_GpPathGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientBlend"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientBlend(
        brush: *mut Gdiplus_GpPathGradient,
        blend: *mut Gdiplus_REAL,
        positions: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientBlend"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientBlend(
        brush: *mut Gdiplus_GpPathGradient,
        blend: *const Gdiplus_REAL,
        positions: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientPresetBlendCount"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientPresetBlendCount(
        brush: *mut Gdiplus_GpPathGradient,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientPresetBlend"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientPresetBlend(
        brush: *mut Gdiplus_GpPathGradient,
        blend: *mut Gdiplus_ARGB,
        positions: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientPresetBlend"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientPresetBlend(
        brush: *mut Gdiplus_GpPathGradient,
        blend: *const Gdiplus_ARGB,
        positions: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientSigmaBlend"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientSigmaBlend(
        brush: *mut Gdiplus_GpPathGradient,
        focus: Gdiplus_REAL,
        scale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientLinearBlend"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientLinearBlend(
        brush: *mut Gdiplus_GpPathGradient,
        focus: Gdiplus_REAL,
        scale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientWrapMode"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientWrapMode(
        brush: *mut Gdiplus_GpPathGradient,
        wrapmode: *mut Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientWrapMode"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientWrapMode(
        brush: *mut Gdiplus_GpPathGradient,
        wrapmode: Gdiplus_GpWrapMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetPathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipResetPathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyPathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipMultiplyPathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        matrix: *const Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslatePathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipTranslatePathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScalePathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipScalePathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        sx: Gdiplus_REAL,
        sy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotatePathGradientTransform"]
    pub fn Gdiplus_DllExports_GdipRotatePathGradientTransform(
        brush: *mut Gdiplus_GpPathGradient,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPathGradientFocusScales"]
    pub fn Gdiplus_DllExports_GdipGetPathGradientFocusScales(
        brush: *mut Gdiplus_GpPathGradient,
        xScale: *mut Gdiplus_REAL,
        yScale: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPathGradientFocusScales"]
    pub fn Gdiplus_DllExports_GdipSetPathGradientFocusScales(
        brush: *mut Gdiplus_GpPathGradient,
        xScale: Gdiplus_REAL,
        yScale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePen1"]
    pub fn Gdiplus_DllExports_GdipCreatePen1(
        color: Gdiplus_ARGB,
        width: Gdiplus_REAL,
        unit: Gdiplus_GpUnit,
        pen: *mut *mut Gdiplus_GpPen,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreatePen2"]
    pub fn Gdiplus_DllExports_GdipCreatePen2(
        brush: *mut Gdiplus_GpBrush,
        width: Gdiplus_REAL,
        unit: Gdiplus_GpUnit,
        pen: *mut *mut Gdiplus_GpPen,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipClonePen"]
    pub fn Gdiplus_DllExports_GdipClonePen(
        pen: *mut Gdiplus_GpPen,
        clonepen: *mut *mut Gdiplus_GpPen,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeletePen"]
    pub fn Gdiplus_DllExports_GdipDeletePen(pen: *mut Gdiplus_GpPen) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenWidth"]
    pub fn Gdiplus_DllExports_GdipSetPenWidth(
        pen: *mut Gdiplus_GpPen,
        width: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenWidth"]
    pub fn Gdiplus_DllExports_GdipGetPenWidth(
        pen: *mut Gdiplus_GpPen,
        width: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenUnit"]
    pub fn Gdiplus_DllExports_GdipSetPenUnit(
        pen: *mut Gdiplus_GpPen,
        unit: Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenUnit"]
    pub fn Gdiplus_DllExports_GdipGetPenUnit(
        pen: *mut Gdiplus_GpPen,
        unit: *mut Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenLineCap197819"]
    pub fn Gdiplus_DllExports_GdipSetPenLineCap197819(
        pen: *mut Gdiplus_GpPen,
        startCap: Gdiplus_GpLineCap,
        endCap: Gdiplus_GpLineCap,
        dashCap: Gdiplus_GpDashCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenStartCap"]
    pub fn Gdiplus_DllExports_GdipSetPenStartCap(
        pen: *mut Gdiplus_GpPen,
        startCap: Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenEndCap"]
    pub fn Gdiplus_DllExports_GdipSetPenEndCap(
        pen: *mut Gdiplus_GpPen,
        endCap: Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenDashCap197819"]
    pub fn Gdiplus_DllExports_GdipSetPenDashCap197819(
        pen: *mut Gdiplus_GpPen,
        dashCap: Gdiplus_GpDashCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenStartCap"]
    pub fn Gdiplus_DllExports_GdipGetPenStartCap(
        pen: *mut Gdiplus_GpPen,
        startCap: *mut Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenEndCap"]
    pub fn Gdiplus_DllExports_GdipGetPenEndCap(
        pen: *mut Gdiplus_GpPen,
        endCap: *mut Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenDashCap197819"]
    pub fn Gdiplus_DllExports_GdipGetPenDashCap197819(
        pen: *mut Gdiplus_GpPen,
        dashCap: *mut Gdiplus_GpDashCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenLineJoin"]
    pub fn Gdiplus_DllExports_GdipSetPenLineJoin(
        pen: *mut Gdiplus_GpPen,
        lineJoin: Gdiplus_GpLineJoin,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenLineJoin"]
    pub fn Gdiplus_DllExports_GdipGetPenLineJoin(
        pen: *mut Gdiplus_GpPen,
        lineJoin: *mut Gdiplus_GpLineJoin,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenCustomStartCap"]
    pub fn Gdiplus_DllExports_GdipSetPenCustomStartCap(
        pen: *mut Gdiplus_GpPen,
        customCap: *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenCustomStartCap"]
    pub fn Gdiplus_DllExports_GdipGetPenCustomStartCap(
        pen: *mut Gdiplus_GpPen,
        customCap: *mut *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenCustomEndCap"]
    pub fn Gdiplus_DllExports_GdipSetPenCustomEndCap(
        pen: *mut Gdiplus_GpPen,
        customCap: *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenCustomEndCap"]
    pub fn Gdiplus_DllExports_GdipGetPenCustomEndCap(
        pen: *mut Gdiplus_GpPen,
        customCap: *mut *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenMiterLimit"]
    pub fn Gdiplus_DllExports_GdipSetPenMiterLimit(
        pen: *mut Gdiplus_GpPen,
        miterLimit: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenMiterLimit"]
    pub fn Gdiplus_DllExports_GdipGetPenMiterLimit(
        pen: *mut Gdiplus_GpPen,
        miterLimit: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenMode"]
    pub fn Gdiplus_DllExports_GdipSetPenMode(
        pen: *mut Gdiplus_GpPen,
        penMode: Gdiplus_GpPenAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenMode"]
    pub fn Gdiplus_DllExports_GdipGetPenMode(
        pen: *mut Gdiplus_GpPen,
        penMode: *mut Gdiplus_GpPenAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenTransform"]
    pub fn Gdiplus_DllExports_GdipSetPenTransform(
        pen: *mut Gdiplus_GpPen,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenTransform"]
    pub fn Gdiplus_DllExports_GdipGetPenTransform(
        pen: *mut Gdiplus_GpPen,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetPenTransform"]
    pub fn Gdiplus_DllExports_GdipResetPenTransform(pen: *mut Gdiplus_GpPen) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyPenTransform"]
    pub fn Gdiplus_DllExports_GdipMultiplyPenTransform(
        pen: *mut Gdiplus_GpPen,
        matrix: *const Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslatePenTransform"]
    pub fn Gdiplus_DllExports_GdipTranslatePenTransform(
        pen: *mut Gdiplus_GpPen,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScalePenTransform"]
    pub fn Gdiplus_DllExports_GdipScalePenTransform(
        pen: *mut Gdiplus_GpPen,
        sx: Gdiplus_REAL,
        sy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotatePenTransform"]
    pub fn Gdiplus_DllExports_GdipRotatePenTransform(
        pen: *mut Gdiplus_GpPen,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenColor"]
    pub fn Gdiplus_DllExports_GdipSetPenColor(
        pen: *mut Gdiplus_GpPen,
        argb: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenColor"]
    pub fn Gdiplus_DllExports_GdipGetPenColor(
        pen: *mut Gdiplus_GpPen,
        argb: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenBrushFill"]
    pub fn Gdiplus_DllExports_GdipSetPenBrushFill(
        pen: *mut Gdiplus_GpPen,
        brush: *mut Gdiplus_GpBrush,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenBrushFill"]
    pub fn Gdiplus_DllExports_GdipGetPenBrushFill(
        pen: *mut Gdiplus_GpPen,
        brush: *mut *mut Gdiplus_GpBrush,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenFillType"]
    pub fn Gdiplus_DllExports_GdipGetPenFillType(
        pen: *mut Gdiplus_GpPen,
        type_: *mut Gdiplus_GpPenType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenDashStyle"]
    pub fn Gdiplus_DllExports_GdipGetPenDashStyle(
        pen: *mut Gdiplus_GpPen,
        dashstyle: *mut Gdiplus_GpDashStyle,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenDashStyle"]
    pub fn Gdiplus_DllExports_GdipSetPenDashStyle(
        pen: *mut Gdiplus_GpPen,
        dashstyle: Gdiplus_GpDashStyle,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenDashOffset"]
    pub fn Gdiplus_DllExports_GdipGetPenDashOffset(
        pen: *mut Gdiplus_GpPen,
        offset: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenDashOffset"]
    pub fn Gdiplus_DllExports_GdipSetPenDashOffset(
        pen: *mut Gdiplus_GpPen,
        offset: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenDashCount"]
    pub fn Gdiplus_DllExports_GdipGetPenDashCount(
        pen: *mut Gdiplus_GpPen,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenDashArray"]
    pub fn Gdiplus_DllExports_GdipSetPenDashArray(
        pen: *mut Gdiplus_GpPen,
        dash: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenDashArray"]
    pub fn Gdiplus_DllExports_GdipGetPenDashArray(
        pen: *mut Gdiplus_GpPen,
        dash: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenCompoundCount"]
    pub fn Gdiplus_DllExports_GdipGetPenCompoundCount(
        pen: *mut Gdiplus_GpPen,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPenCompoundArray"]
    pub fn Gdiplus_DllExports_GdipSetPenCompoundArray(
        pen: *mut Gdiplus_GpPen,
        dash: *const Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPenCompoundArray"]
    pub fn Gdiplus_DllExports_GdipGetPenCompoundArray(
        pen: *mut Gdiplus_GpPen,
        dash: *mut Gdiplus_REAL,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateCustomLineCap"]
    pub fn Gdiplus_DllExports_GdipCreateCustomLineCap(
        fillPath: *mut Gdiplus_GpPath,
        strokePath: *mut Gdiplus_GpPath,
        baseCap: Gdiplus_GpLineCap,
        baseInset: Gdiplus_REAL,
        customCap: *mut *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteCustomLineCap"]
    pub fn Gdiplus_DllExports_GdipDeleteCustomLineCap(
        customCap: *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneCustomLineCap"]
    pub fn Gdiplus_DllExports_GdipCloneCustomLineCap(
        customCap: *mut Gdiplus_GpCustomLineCap,
        clonedCap: *mut *mut Gdiplus_GpCustomLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapType"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapType(
        customCap: *mut Gdiplus_GpCustomLineCap,
        capType: *mut Gdiplus_CustomLineCapType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCustomLineCapStrokeCaps"]
    pub fn Gdiplus_DllExports_GdipSetCustomLineCapStrokeCaps(
        customCap: *mut Gdiplus_GpCustomLineCap,
        startCap: Gdiplus_GpLineCap,
        endCap: Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapStrokeCaps"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapStrokeCaps(
        customCap: *mut Gdiplus_GpCustomLineCap,
        startCap: *mut Gdiplus_GpLineCap,
        endCap: *mut Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCustomLineCapStrokeJoin"]
    pub fn Gdiplus_DllExports_GdipSetCustomLineCapStrokeJoin(
        customCap: *mut Gdiplus_GpCustomLineCap,
        lineJoin: Gdiplus_GpLineJoin,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapStrokeJoin"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapStrokeJoin(
        customCap: *mut Gdiplus_GpCustomLineCap,
        lineJoin: *mut Gdiplus_GpLineJoin,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCustomLineCapBaseCap"]
    pub fn Gdiplus_DllExports_GdipSetCustomLineCapBaseCap(
        customCap: *mut Gdiplus_GpCustomLineCap,
        baseCap: Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapBaseCap"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapBaseCap(
        customCap: *mut Gdiplus_GpCustomLineCap,
        baseCap: *mut Gdiplus_GpLineCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCustomLineCapBaseInset"]
    pub fn Gdiplus_DllExports_GdipSetCustomLineCapBaseInset(
        customCap: *mut Gdiplus_GpCustomLineCap,
        inset: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapBaseInset"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapBaseInset(
        customCap: *mut Gdiplus_GpCustomLineCap,
        inset: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCustomLineCapWidthScale"]
    pub fn Gdiplus_DllExports_GdipSetCustomLineCapWidthScale(
        customCap: *mut Gdiplus_GpCustomLineCap,
        widthScale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCustomLineCapWidthScale"]
    pub fn Gdiplus_DllExports_GdipGetCustomLineCapWidthScale(
        customCap: *mut Gdiplus_GpCustomLineCap,
        widthScale: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateAdjustableArrowCap"]
    pub fn Gdiplus_DllExports_GdipCreateAdjustableArrowCap(
        height: Gdiplus_REAL,
        width: Gdiplus_REAL,
        isFilled: BOOL,
        cap: *mut *mut Gdiplus_GpAdjustableArrowCap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetAdjustableArrowCapHeight"]
    pub fn Gdiplus_DllExports_GdipSetAdjustableArrowCapHeight(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetAdjustableArrowCapHeight"]
    pub fn Gdiplus_DllExports_GdipGetAdjustableArrowCapHeight(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        height: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetAdjustableArrowCapWidth"]
    pub fn Gdiplus_DllExports_GdipSetAdjustableArrowCapWidth(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        width: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetAdjustableArrowCapWidth"]
    pub fn Gdiplus_DllExports_GdipGetAdjustableArrowCapWidth(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        width: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetAdjustableArrowCapMiddleInset"]
    pub fn Gdiplus_DllExports_GdipSetAdjustableArrowCapMiddleInset(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        middleInset: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetAdjustableArrowCapMiddleInset"]
    pub fn Gdiplus_DllExports_GdipGetAdjustableArrowCapMiddleInset(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        middleInset: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetAdjustableArrowCapFillState"]
    pub fn Gdiplus_DllExports_GdipSetAdjustableArrowCapFillState(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        fillState: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetAdjustableArrowCapFillState"]
    pub fn Gdiplus_DllExports_GdipGetAdjustableArrowCapFillState(
        cap: *mut Gdiplus_GpAdjustableArrowCap,
        fillState: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipLoadImageFromStream"]
    pub fn Gdiplus_DllExports_GdipLoadImageFromStream(
        stream: *mut IStream,
        image: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipLoadImageFromFile"]
    pub fn Gdiplus_DllExports_GdipLoadImageFromFile(
        filename: *const WCHAR,
        image: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipLoadImageFromStreamICM"]
    pub fn Gdiplus_DllExports_GdipLoadImageFromStreamICM(
        stream: *mut IStream,
        image: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipLoadImageFromFileICM"]
    pub fn Gdiplus_DllExports_GdipLoadImageFromFileICM(
        filename: *const WCHAR,
        image: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneImage"]
    pub fn Gdiplus_DllExports_GdipCloneImage(
        image: *mut Gdiplus_GpImage,
        cloneImage: *mut *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDisposeImage"]
    pub fn Gdiplus_DllExports_GdipDisposeImage(image: *mut Gdiplus_GpImage) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSaveImageToFile"]
    pub fn Gdiplus_DllExports_GdipSaveImageToFile(
        image: *mut Gdiplus_GpImage,
        filename: *const WCHAR,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSaveImageToStream"]
    pub fn Gdiplus_DllExports_GdipSaveImageToStream(
        image: *mut Gdiplus_GpImage,
        stream: *mut IStream,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSaveAdd"]
    pub fn Gdiplus_DllExports_GdipSaveAdd(
        image: *mut Gdiplus_GpImage,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSaveAddImage"]
    pub fn Gdiplus_DllExports_GdipSaveAddImage(
        image: *mut Gdiplus_GpImage,
        newImage: *mut Gdiplus_GpImage,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageGraphicsContext"]
    pub fn Gdiplus_DllExports_GdipGetImageGraphicsContext(
        image: *mut Gdiplus_GpImage,
        graphics: *mut *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageBounds"]
    pub fn Gdiplus_DllExports_GdipGetImageBounds(
        image: *mut Gdiplus_GpImage,
        srcRect: *mut Gdiplus_GpRectF,
        srcUnit: *mut Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageDimension"]
    pub fn Gdiplus_DllExports_GdipGetImageDimension(
        image: *mut Gdiplus_GpImage,
        width: *mut Gdiplus_REAL,
        height: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageType"]
    pub fn Gdiplus_DllExports_GdipGetImageType(
        image: *mut Gdiplus_GpImage,
        type_: *mut Gdiplus_ImageType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageWidth"]
    pub fn Gdiplus_DllExports_GdipGetImageWidth(
        image: *mut Gdiplus_GpImage,
        width: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageHeight"]
    pub fn Gdiplus_DllExports_GdipGetImageHeight(
        image: *mut Gdiplus_GpImage,
        height: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageHorizontalResolution"]
    pub fn Gdiplus_DllExports_GdipGetImageHorizontalResolution(
        image: *mut Gdiplus_GpImage,
        resolution: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageVerticalResolution"]
    pub fn Gdiplus_DllExports_GdipGetImageVerticalResolution(
        image: *mut Gdiplus_GpImage,
        resolution: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageFlags"]
    pub fn Gdiplus_DllExports_GdipGetImageFlags(
        image: *mut Gdiplus_GpImage,
        flags: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageRawFormat"]
    pub fn Gdiplus_DllExports_GdipGetImageRawFormat(
        image: *mut Gdiplus_GpImage,
        format: *mut GUID,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImagePixelFormat"]
    pub fn Gdiplus_DllExports_GdipGetImagePixelFormat(
        image: *mut Gdiplus_GpImage,
        format: *mut Gdiplus_PixelFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageThumbnail"]
    pub fn Gdiplus_DllExports_GdipGetImageThumbnail(
        image: *mut Gdiplus_GpImage,
        thumbWidth: UINT,
        thumbHeight: UINT,
        thumbImage: *mut *mut Gdiplus_GpImage,
        callback: Gdiplus_GetThumbnailImageAbort,
        callbackData: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetEncoderParameterListSize"]
    pub fn Gdiplus_DllExports_GdipGetEncoderParameterListSize(
        image: *mut Gdiplus_GpImage,
        clsidEncoder: *const CLSID,
        size: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetEncoderParameterList"]
    pub fn Gdiplus_DllExports_GdipGetEncoderParameterList(
        image: *mut Gdiplus_GpImage,
        clsidEncoder: *const CLSID,
        size: UINT,
        buffer: *mut Gdiplus_EncoderParameters,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageGetFrameDimensionsCount"]
    pub fn Gdiplus_DllExports_GdipImageGetFrameDimensionsCount(
        image: *mut Gdiplus_GpImage,
        count: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageGetFrameDimensionsList"]
    pub fn Gdiplus_DllExports_GdipImageGetFrameDimensionsList(
        image: *mut Gdiplus_GpImage,
        dimensionIDs: *mut GUID,
        count: UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageGetFrameCount"]
    pub fn Gdiplus_DllExports_GdipImageGetFrameCount(
        image: *mut Gdiplus_GpImage,
        dimensionID: *const GUID,
        count: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageSelectActiveFrame"]
    pub fn Gdiplus_DllExports_GdipImageSelectActiveFrame(
        image: *mut Gdiplus_GpImage,
        dimensionID: *const GUID,
        frameIndex: UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageRotateFlip"]
    pub fn Gdiplus_DllExports_GdipImageRotateFlip(
        image: *mut Gdiplus_GpImage,
        rfType: Gdiplus_RotateFlipType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImagePalette"]
    pub fn Gdiplus_DllExports_GdipGetImagePalette(
        image: *mut Gdiplus_GpImage,
        palette: *mut Gdiplus_ColorPalette,
        size: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImagePalette"]
    pub fn Gdiplus_DllExports_GdipSetImagePalette(
        image: *mut Gdiplus_GpImage,
        palette: *const Gdiplus_ColorPalette,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImagePaletteSize"]
    pub fn Gdiplus_DllExports_GdipGetImagePaletteSize(
        image: *mut Gdiplus_GpImage,
        size: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPropertyCount"]
    pub fn Gdiplus_DllExports_GdipGetPropertyCount(
        image: *mut Gdiplus_GpImage,
        numOfProperty: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPropertyIdList"]
    pub fn Gdiplus_DllExports_GdipGetPropertyIdList(
        image: *mut Gdiplus_GpImage,
        numOfProperty: UINT,
        list: *mut PROPID,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPropertyItemSize"]
    pub fn Gdiplus_DllExports_GdipGetPropertyItemSize(
        image: *mut Gdiplus_GpImage,
        propId: PROPID,
        size: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPropertyItem"]
    pub fn Gdiplus_DllExports_GdipGetPropertyItem(
        image: *mut Gdiplus_GpImage,
        propId: PROPID,
        propSize: UINT,
        buffer: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPropertySize"]
    pub fn Gdiplus_DllExports_GdipGetPropertySize(
        image: *mut Gdiplus_GpImage,
        totalBufferSize: *mut UINT,
        numProperties: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetAllPropertyItems"]
    pub fn Gdiplus_DllExports_GdipGetAllPropertyItems(
        image: *mut Gdiplus_GpImage,
        totalBufferSize: UINT,
        numProperties: UINT,
        allItems: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRemovePropertyItem"]
    pub fn Gdiplus_DllExports_GdipRemovePropertyItem(
        image: *mut Gdiplus_GpImage,
        propId: PROPID,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPropertyItem"]
    pub fn Gdiplus_DllExports_GdipSetPropertyItem(
        image: *mut Gdiplus_GpImage,
        item: *const Gdiplus_PropertyItem,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipImageForceValidation"]
    pub fn Gdiplus_DllExports_GdipImageForceValidation(
        image: *mut Gdiplus_GpImage,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromStream"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromStream(
        stream: *mut IStream,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromFile"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromFile(
        filename: *const WCHAR,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromStreamICM"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromStreamICM(
        stream: *mut IStream,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromFileICM"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromFileICM(
        filename: *const WCHAR,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromScan0"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromScan0(
        width: INT,
        height: INT,
        stride: INT,
        format: Gdiplus_PixelFormat,
        scan0: *mut BYTE,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromGraphics"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromGraphics(
        width: INT,
        height: INT,
        target: *mut Gdiplus_GpGraphics,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromDirectDrawSurface"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromDirectDrawSurface(
        surface: *mut IDirectDrawSurface7,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromGdiDib"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromGdiDib(
        gdiBitmapInfo: *const BITMAPINFO,
        gdiBitmapData: *mut c_void,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromHBITMAP"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromHBITMAP(
        hbm: HBITMAP,
        hpal: HPALETTE,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateHBITMAPFromBitmap"]
    pub fn Gdiplus_DllExports_GdipCreateHBITMAPFromBitmap(
        bitmap: *mut Gdiplus_GpBitmap,
        hbmReturn: *mut HBITMAP,
        background: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromHICON"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromHICON(
        hicon: HICON,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateHICONFromBitmap"]
    pub fn Gdiplus_DllExports_GdipCreateHICONFromBitmap(
        bitmap: *mut Gdiplus_GpBitmap,
        hbmReturn: *mut HICON,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateBitmapFromResource"]
    pub fn Gdiplus_DllExports_GdipCreateBitmapFromResource(
        hInstance: HINSTANCE,
        lpBitmapName: *const WCHAR,
        bitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneBitmapArea"]
    pub fn Gdiplus_DllExports_GdipCloneBitmapArea(
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        format: Gdiplus_PixelFormat,
        srcBitmap: *mut Gdiplus_GpBitmap,
        dstBitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneBitmapAreaI"]
    pub fn Gdiplus_DllExports_GdipCloneBitmapAreaI(
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        format: Gdiplus_PixelFormat,
        srcBitmap: *mut Gdiplus_GpBitmap,
        dstBitmap: *mut *mut Gdiplus_GpBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBitmapLockBits"]
    pub fn Gdiplus_DllExports_GdipBitmapLockBits(
        bitmap: *mut Gdiplus_GpBitmap,
        rect: *const Gdiplus_GpRect,
        flags: UINT,
        format: Gdiplus_PixelFormat,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBitmapUnlockBits"]
    pub fn Gdiplus_DllExports_GdipBitmapUnlockBits(
        bitmap: *mut Gdiplus_GpBitmap,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBitmapGetPixel"]
    pub fn Gdiplus_DllExports_GdipBitmapGetPixel(
        bitmap: *mut Gdiplus_GpBitmap,
        x: INT,
        y: INT,
        color: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBitmapSetPixel"]
    pub fn Gdiplus_DllExports_GdipBitmapSetPixel(
        bitmap: *mut Gdiplus_GpBitmap,
        x: INT,
        y: INT,
        color: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBitmapSetResolution"]
    pub fn Gdiplus_DllExports_GdipBitmapSetResolution(
        bitmap: *mut Gdiplus_GpBitmap,
        xdpi: Gdiplus_REAL,
        ydpi: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateImageAttributes"]
    pub fn Gdiplus_DllExports_GdipCreateImageAttributes(
        imageattr: *mut *mut Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneImageAttributes"]
    pub fn Gdiplus_DllExports_GdipCloneImageAttributes(
        imageattr: *const Gdiplus_GpImageAttributes,
        cloneImageattr: *mut *mut Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDisposeImageAttributes"]
    pub fn Gdiplus_DllExports_GdipDisposeImageAttributes(
        imageattr: *mut Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesToIdentity"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesToIdentity(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetImageAttributes"]
    pub fn Gdiplus_DllExports_GdipResetImageAttributes(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesColorMatrix"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesColorMatrix(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        colorMatrix: *const Gdiplus_ColorMatrix,
        grayMatrix: *const Gdiplus_ColorMatrix,
        flags: Gdiplus_ColorMatrixFlags,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesThreshold"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesThreshold(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        threshold: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesGamma"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesGamma(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        gamma: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesNoOp"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesNoOp(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesColorKeys"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesColorKeys(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        colorLow: Gdiplus_ARGB,
        colorHigh: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesOutputChannel"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesOutputChannel(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        channelFlags: Gdiplus_ColorChannelFlags,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesOutputChannelColorProfile"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesOutputChannelColorProfile(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        colorProfileFilename: *const WCHAR,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesRemapTable"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesRemapTable(
        imageattr: *mut Gdiplus_GpImageAttributes,
        type_: Gdiplus_ColorAdjustType,
        enableFlag: BOOL,
        mapSize: UINT,
        map: *const Gdiplus_ColorMap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesWrapMode"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesWrapMode(
        imageAttr: *mut Gdiplus_GpImageAttributes,
        wrap: Gdiplus_WrapMode,
        argb: Gdiplus_ARGB,
        clamp: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesICMMode"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesICMMode(
        imageAttr: *mut Gdiplus_GpImageAttributes,
        on: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageAttributesAdjustedPalette"]
    pub fn Gdiplus_DllExports_GdipGetImageAttributesAdjustedPalette(
        imageAttr: *mut Gdiplus_GpImageAttributes,
        colorPalette: *mut Gdiplus_ColorPalette,
        colorAdjustType: Gdiplus_ColorAdjustType,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFlush"]
    pub fn Gdiplus_DllExports_GdipFlush(
        graphics: *mut Gdiplus_GpGraphics,
        intention: Gdiplus_GpFlushIntention,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFromHDC"]
    pub fn Gdiplus_DllExports_GdipCreateFromHDC(
        hdc: HDC,
        graphics: *mut *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFromHDC2"]
    pub fn Gdiplus_DllExports_GdipCreateFromHDC2(
        hdc: HDC,
        hDevice: HANDLE,
        graphics: *mut *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFromHWND"]
    pub fn Gdiplus_DllExports_GdipCreateFromHWND(
        hwnd: HWND,
        graphics: *mut *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFromHWNDICM"]
    pub fn Gdiplus_DllExports_GdipCreateFromHWNDICM(
        hwnd: HWND,
        graphics: *mut *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteGraphics"]
    pub fn Gdiplus_DllExports_GdipDeleteGraphics(
        graphics: *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetDC"]
    pub fn Gdiplus_DllExports_GdipGetDC(
        graphics: *mut Gdiplus_GpGraphics,
        hdc: *mut HDC,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipReleaseDC"]
    pub fn Gdiplus_DllExports_GdipReleaseDC(
        graphics: *mut Gdiplus_GpGraphics,
        hdc: HDC,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCompositingMode"]
    pub fn Gdiplus_DllExports_GdipSetCompositingMode(
        graphics: *mut Gdiplus_GpGraphics,
        compositingMode: Gdiplus_CompositingMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCompositingMode"]
    pub fn Gdiplus_DllExports_GdipGetCompositingMode(
        graphics: *mut Gdiplus_GpGraphics,
        compositingMode: *mut Gdiplus_CompositingMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetRenderingOrigin"]
    pub fn Gdiplus_DllExports_GdipSetRenderingOrigin(
        graphics: *mut Gdiplus_GpGraphics,
        x: INT,
        y: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetRenderingOrigin"]
    pub fn Gdiplus_DllExports_GdipGetRenderingOrigin(
        graphics: *mut Gdiplus_GpGraphics,
        x: *mut INT,
        y: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetCompositingQuality"]
    pub fn Gdiplus_DllExports_GdipSetCompositingQuality(
        graphics: *mut Gdiplus_GpGraphics,
        compositingQuality: Gdiplus_CompositingQuality,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCompositingQuality"]
    pub fn Gdiplus_DllExports_GdipGetCompositingQuality(
        graphics: *mut Gdiplus_GpGraphics,
        compositingQuality: *mut Gdiplus_CompositingQuality,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetSmoothingMode"]
    pub fn Gdiplus_DllExports_GdipSetSmoothingMode(
        graphics: *mut Gdiplus_GpGraphics,
        smoothingMode: Gdiplus_SmoothingMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetSmoothingMode"]
    pub fn Gdiplus_DllExports_GdipGetSmoothingMode(
        graphics: *mut Gdiplus_GpGraphics,
        smoothingMode: *mut Gdiplus_SmoothingMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPixelOffsetMode"]
    pub fn Gdiplus_DllExports_GdipSetPixelOffsetMode(
        graphics: *mut Gdiplus_GpGraphics,
        pixelOffsetMode: Gdiplus_PixelOffsetMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPixelOffsetMode"]
    pub fn Gdiplus_DllExports_GdipGetPixelOffsetMode(
        graphics: *mut Gdiplus_GpGraphics,
        pixelOffsetMode: *mut Gdiplus_PixelOffsetMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetTextRenderingHint"]
    pub fn Gdiplus_DllExports_GdipSetTextRenderingHint(
        graphics: *mut Gdiplus_GpGraphics,
        mode: Gdiplus_TextRenderingHint,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetTextRenderingHint"]
    pub fn Gdiplus_DllExports_GdipGetTextRenderingHint(
        graphics: *mut Gdiplus_GpGraphics,
        mode: *mut Gdiplus_TextRenderingHint,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetTextContrast"]
    pub fn Gdiplus_DllExports_GdipSetTextContrast(
        graphics: *mut Gdiplus_GpGraphics,
        contrast: UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetTextContrast"]
    pub fn Gdiplus_DllExports_GdipGetTextContrast(
        graphics: *mut Gdiplus_GpGraphics,
        contrast: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetInterpolationMode"]
    pub fn Gdiplus_DllExports_GdipSetInterpolationMode(
        graphics: *mut Gdiplus_GpGraphics,
        interpolationMode: Gdiplus_InterpolationMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetInterpolationMode"]
    pub fn Gdiplus_DllExports_GdipGetInterpolationMode(
        graphics: *mut Gdiplus_GpGraphics,
        interpolationMode: *mut Gdiplus_InterpolationMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetWorldTransform"]
    pub fn Gdiplus_DllExports_GdipSetWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetWorldTransform"]
    pub fn Gdiplus_DllExports_GdipResetWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMultiplyWorldTransform"]
    pub fn Gdiplus_DllExports_GdipMultiplyWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        matrix: *const Gdiplus_GpMatrix,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateWorldTransform"]
    pub fn Gdiplus_DllExports_GdipTranslateWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipScaleWorldTransform"]
    pub fn Gdiplus_DllExports_GdipScaleWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        sx: Gdiplus_REAL,
        sy: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRotateWorldTransform"]
    pub fn Gdiplus_DllExports_GdipRotateWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        angle: Gdiplus_REAL,
        order: Gdiplus_GpMatrixOrder,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetWorldTransform"]
    pub fn Gdiplus_DllExports_GdipGetWorldTransform(
        graphics: *mut Gdiplus_GpGraphics,
        matrix: *mut Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetPageTransform"]
    pub fn Gdiplus_DllExports_GdipResetPageTransform(
        graphics: *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPageUnit"]
    pub fn Gdiplus_DllExports_GdipGetPageUnit(
        graphics: *mut Gdiplus_GpGraphics,
        unit: *mut Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetPageScale"]
    pub fn Gdiplus_DllExports_GdipGetPageScale(
        graphics: *mut Gdiplus_GpGraphics,
        scale: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPageUnit"]
    pub fn Gdiplus_DllExports_GdipSetPageUnit(
        graphics: *mut Gdiplus_GpGraphics,
        unit: Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetPageScale"]
    pub fn Gdiplus_DllExports_GdipSetPageScale(
        graphics: *mut Gdiplus_GpGraphics,
        scale: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetDpiX"]
    pub fn Gdiplus_DllExports_GdipGetDpiX(
        graphics: *mut Gdiplus_GpGraphics,
        dpi: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetDpiY"]
    pub fn Gdiplus_DllExports_GdipGetDpiY(
        graphics: *mut Gdiplus_GpGraphics,
        dpi: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformPoints"]
    pub fn Gdiplus_DllExports_GdipTransformPoints(
        graphics: *mut Gdiplus_GpGraphics,
        destSpace: Gdiplus_GpCoordinateSpace,
        srcSpace: Gdiplus_GpCoordinateSpace,
        points: *mut Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTransformPointsI"]
    pub fn Gdiplus_DllExports_GdipTransformPointsI(
        graphics: *mut Gdiplus_GpGraphics,
        destSpace: Gdiplus_GpCoordinateSpace,
        srcSpace: Gdiplus_GpCoordinateSpace,
        points: *mut Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetNearestColor"]
    pub fn Gdiplus_DllExports_GdipGetNearestColor(
        graphics: *mut Gdiplus_GpGraphics,
        argb: *mut Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateHalftonePalette"]
    pub fn Gdiplus_DllExports_GdipCreateHalftonePalette() -> HPALETTE;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawLine"]
    pub fn Gdiplus_DllExports_GdipDrawLine(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x1: Gdiplus_REAL,
        y1: Gdiplus_REAL,
        x2: Gdiplus_REAL,
        y2: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawLineI"]
    pub fn Gdiplus_DllExports_GdipDrawLineI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawLines"]
    pub fn Gdiplus_DllExports_GdipDrawLines(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawLinesI"]
    pub fn Gdiplus_DllExports_GdipDrawLinesI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawArc"]
    pub fn Gdiplus_DllExports_GdipDrawArc(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawArcI"]
    pub fn Gdiplus_DllExports_GdipDrawArcI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawBezier"]
    pub fn Gdiplus_DllExports_GdipDrawBezier(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x1: Gdiplus_REAL,
        y1: Gdiplus_REAL,
        x2: Gdiplus_REAL,
        y2: Gdiplus_REAL,
        x3: Gdiplus_REAL,
        y3: Gdiplus_REAL,
        x4: Gdiplus_REAL,
        y4: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawBezierI"]
    pub fn Gdiplus_DllExports_GdipDrawBezierI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x1: INT,
        y1: INT,
        x2: INT,
        y2: INT,
        x3: INT,
        y3: INT,
        x4: INT,
        y4: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawBeziers"]
    pub fn Gdiplus_DllExports_GdipDrawBeziers(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawBeziersI"]
    pub fn Gdiplus_DllExports_GdipDrawBeziersI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawRectangle"]
    pub fn Gdiplus_DllExports_GdipDrawRectangle(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawRectangleI"]
    pub fn Gdiplus_DllExports_GdipDrawRectangleI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawRectangles"]
    pub fn Gdiplus_DllExports_GdipDrawRectangles(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        rects: *const Gdiplus_GpRectF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawRectanglesI"]
    pub fn Gdiplus_DllExports_GdipDrawRectanglesI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        rects: *const Gdiplus_GpRect,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawEllipse"]
    pub fn Gdiplus_DllExports_GdipDrawEllipse(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawEllipseI"]
    pub fn Gdiplus_DllExports_GdipDrawEllipseI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawPie"]
    pub fn Gdiplus_DllExports_GdipDrawPie(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawPieI"]
    pub fn Gdiplus_DllExports_GdipDrawPieI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawPolygon"]
    pub fn Gdiplus_DllExports_GdipDrawPolygon(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawPolygonI"]
    pub fn Gdiplus_DllExports_GdipDrawPolygonI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawPath"]
    pub fn Gdiplus_DllExports_GdipDrawPath(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        path: *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurve"]
    pub fn Gdiplus_DllExports_GdipDrawCurve(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurveI"]
    pub fn Gdiplus_DllExports_GdipDrawCurveI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurve2"]
    pub fn Gdiplus_DllExports_GdipDrawCurve2(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurve2I"]
    pub fn Gdiplus_DllExports_GdipDrawCurve2I(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurve3"]
    pub fn Gdiplus_DllExports_GdipDrawCurve3(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCurve3I"]
    pub fn Gdiplus_DllExports_GdipDrawCurve3I(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
        offset: INT,
        numberOfSegments: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawClosedCurve"]
    pub fn Gdiplus_DllExports_GdipDrawClosedCurve(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawClosedCurveI"]
    pub fn Gdiplus_DllExports_GdipDrawClosedCurveI(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawClosedCurve2"]
    pub fn Gdiplus_DllExports_GdipDrawClosedCurve2(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPointF,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawClosedCurve2I"]
    pub fn Gdiplus_DllExports_GdipDrawClosedCurve2I(
        graphics: *mut Gdiplus_GpGraphics,
        pen: *mut Gdiplus_GpPen,
        points: *const Gdiplus_GpPoint,
        count: INT,
        tension: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGraphicsClear"]
    pub fn Gdiplus_DllExports_GdipGraphicsClear(
        graphics: *mut Gdiplus_GpGraphics,
        color: Gdiplus_ARGB,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillRectangle"]
    pub fn Gdiplus_DllExports_GdipFillRectangle(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillRectangleI"]
    pub fn Gdiplus_DllExports_GdipFillRectangleI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillRectangles"]
    pub fn Gdiplus_DllExports_GdipFillRectangles(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        rects: *const Gdiplus_GpRectF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillRectanglesI"]
    pub fn Gdiplus_DllExports_GdipFillRectanglesI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        rects: *const Gdiplus_GpRect,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPolygon"]
    pub fn Gdiplus_DllExports_GdipFillPolygon(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPointF,
        count: INT,
        fillMode: Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPolygonI"]
    pub fn Gdiplus_DllExports_GdipFillPolygonI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPoint,
        count: INT,
        fillMode: Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPolygon2"]
    pub fn Gdiplus_DllExports_GdipFillPolygon2(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPolygon2I"]
    pub fn Gdiplus_DllExports_GdipFillPolygon2I(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillEllipse"]
    pub fn Gdiplus_DllExports_GdipFillEllipse(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillEllipseI"]
    pub fn Gdiplus_DllExports_GdipFillEllipseI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPie"]
    pub fn Gdiplus_DllExports_GdipFillPie(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPieI"]
    pub fn Gdiplus_DllExports_GdipFillPieI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        startAngle: Gdiplus_REAL,
        sweepAngle: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillPath"]
    pub fn Gdiplus_DllExports_GdipFillPath(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        path: *mut Gdiplus_GpPath,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillClosedCurve"]
    pub fn Gdiplus_DllExports_GdipFillClosedCurve(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillClosedCurveI"]
    pub fn Gdiplus_DllExports_GdipFillClosedCurveI(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillClosedCurve2"]
    pub fn Gdiplus_DllExports_GdipFillClosedCurve2(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPointF,
        count: INT,
        tension: Gdiplus_REAL,
        fillMode: Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillClosedCurve2I"]
    pub fn Gdiplus_DllExports_GdipFillClosedCurve2I(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        points: *const Gdiplus_GpPoint,
        count: INT,
        tension: Gdiplus_REAL,
        fillMode: Gdiplus_GpFillMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFillRegion"]
    pub fn Gdiplus_DllExports_GdipFillRegion(
        graphics: *mut Gdiplus_GpGraphics,
        brush: *mut Gdiplus_GpBrush,
        region: *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImage"]
    pub fn Gdiplus_DllExports_GdipDrawImage(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImageI"]
    pub fn Gdiplus_DllExports_GdipDrawImageI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: INT,
        y: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImageRect"]
    pub fn Gdiplus_DllExports_GdipDrawImageRect(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImageRectI"]
    pub fn Gdiplus_DllExports_GdipDrawImageRectI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePoints"]
    pub fn Gdiplus_DllExports_GdipDrawImagePoints(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        dstpoints: *const Gdiplus_GpPointF,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePointsI"]
    pub fn Gdiplus_DllExports_GdipDrawImagePointsI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        dstpoints: *const Gdiplus_GpPoint,
        count: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePointRect"]
    pub fn Gdiplus_DllExports_GdipDrawImagePointRect(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        srcx: Gdiplus_REAL,
        srcy: Gdiplus_REAL,
        srcwidth: Gdiplus_REAL,
        srcheight: Gdiplus_REAL,
        srcUnit: Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePointRectI"]
    pub fn Gdiplus_DllExports_GdipDrawImagePointRectI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        x: INT,
        y: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: Gdiplus_GpUnit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImageRectRect"]
    pub fn Gdiplus_DllExports_GdipDrawImageRectRect(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        dstx: Gdiplus_REAL,
        dsty: Gdiplus_REAL,
        dstwidth: Gdiplus_REAL,
        dstheight: Gdiplus_REAL,
        srcx: Gdiplus_REAL,
        srcy: Gdiplus_REAL,
        srcwidth: Gdiplus_REAL,
        srcheight: Gdiplus_REAL,
        srcUnit: Gdiplus_GpUnit,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        callback: Gdiplus_DrawImageAbort,
        callbackData: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImageRectRectI"]
    pub fn Gdiplus_DllExports_GdipDrawImageRectRectI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        dstx: INT,
        dsty: INT,
        dstwidth: INT,
        dstheight: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: Gdiplus_GpUnit,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        callback: Gdiplus_DrawImageAbort,
        callbackData: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePointsRect"]
    pub fn Gdiplus_DllExports_GdipDrawImagePointsRect(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        points: *const Gdiplus_GpPointF,
        count: INT,
        srcx: Gdiplus_REAL,
        srcy: Gdiplus_REAL,
        srcwidth: Gdiplus_REAL,
        srcheight: Gdiplus_REAL,
        srcUnit: Gdiplus_GpUnit,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        callback: Gdiplus_DrawImageAbort,
        callbackData: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawImagePointsRectI"]
    pub fn Gdiplus_DllExports_GdipDrawImagePointsRectI(
        graphics: *mut Gdiplus_GpGraphics,
        image: *mut Gdiplus_GpImage,
        points: *const Gdiplus_GpPoint,
        count: INT,
        srcx: INT,
        srcy: INT,
        srcwidth: INT,
        srcheight: INT,
        srcUnit: Gdiplus_GpUnit,
        imageAttributes: *const Gdiplus_GpImageAttributes,
        callback: Gdiplus_DrawImageAbort,
        callbackData: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestPoint"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestPoint(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoint: *const Gdiplus_PointF,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestPointI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestPointI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoint: *const Gdiplus_Point,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestRect"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestRect(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destRect: *const Gdiplus_RectF,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestRectI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestRectI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destRect: *const Gdiplus_Rect,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestPoints"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestPoints(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoints: *const Gdiplus_PointF,
        count: INT,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileDestPointsI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileDestPointsI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoints: *const Gdiplus_Point,
        count: INT,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestPoint"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestPoint(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoint: *const Gdiplus_PointF,
        srcRect: *const Gdiplus_RectF,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestPointI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestPointI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoint: *const Gdiplus_Point,
        srcRect: *const Gdiplus_Rect,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestRect"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestRect(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destRect: *const Gdiplus_RectF,
        srcRect: *const Gdiplus_RectF,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestRectI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestRectI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destRect: *const Gdiplus_Rect,
        srcRect: *const Gdiplus_Rect,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestPoints"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestPoints(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoints: *const Gdiplus_PointF,
        count: INT,
        srcRect: *const Gdiplus_RectF,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEnumerateMetafileSrcRectDestPointsI"]
    pub fn Gdiplus_DllExports_GdipEnumerateMetafileSrcRectDestPointsI(
        graphics: *mut Gdiplus_GpGraphics,
        metafile: *const Gdiplus_GpMetafile,
        destPoints: *const Gdiplus_Point,
        count: INT,
        srcRect: *const Gdiplus_Rect,
        srcUnit: Gdiplus_Unit,
        callback: Gdiplus_EnumerateMetafileProc,
        callbackData: *mut c_void,
        imageAttributes: *const Gdiplus_GpImageAttributes,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPlayMetafileRecord"]
    pub fn Gdiplus_DllExports_GdipPlayMetafileRecord(
        metafile: *const Gdiplus_GpMetafile,
        recordType: Gdiplus_EmfPlusRecordType,
        flags: UINT,
        dataSize: UINT,
        data: *const BYTE,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipGraphics"]
    pub fn Gdiplus_DllExports_GdipSetClipGraphics(
        graphics: *mut Gdiplus_GpGraphics,
        srcgraphics: *mut Gdiplus_GpGraphics,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipRect"]
    pub fn Gdiplus_DllExports_GdipSetClipRect(
        graphics: *mut Gdiplus_GpGraphics,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipRectI"]
    pub fn Gdiplus_DllExports_GdipSetClipRectI(
        graphics: *mut Gdiplus_GpGraphics,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipPath"]
    pub fn Gdiplus_DllExports_GdipSetClipPath(
        graphics: *mut Gdiplus_GpGraphics,
        path: *mut Gdiplus_GpPath,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipRegion"]
    pub fn Gdiplus_DllExports_GdipSetClipRegion(
        graphics: *mut Gdiplus_GpGraphics,
        region: *mut Gdiplus_GpRegion,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetClipHrgn"]
    pub fn Gdiplus_DllExports_GdipSetClipHrgn(
        graphics: *mut Gdiplus_GpGraphics,
        hRgn: HRGN,
        combineMode: Gdiplus_CombineMode,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipResetClip"]
    pub fn Gdiplus_DllExports_GdipResetClip(graphics: *mut Gdiplus_GpGraphics) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateClip"]
    pub fn Gdiplus_DllExports_GdipTranslateClip(
        graphics: *mut Gdiplus_GpGraphics,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTranslateClipI"]
    pub fn Gdiplus_DllExports_GdipTranslateClipI(
        graphics: *mut Gdiplus_GpGraphics,
        dx: INT,
        dy: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetClip"]
    pub fn Gdiplus_DllExports_GdipGetClip(
        graphics: *mut Gdiplus_GpGraphics,
        region: *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetClipBounds"]
    pub fn Gdiplus_DllExports_GdipGetClipBounds(
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetClipBoundsI"]
    pub fn Gdiplus_DllExports_GdipGetClipBoundsI(
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRect,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsClipEmpty"]
    pub fn Gdiplus_DllExports_GdipIsClipEmpty(
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetVisibleClipBounds"]
    pub fn Gdiplus_DllExports_GdipGetVisibleClipBounds(
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetVisibleClipBoundsI"]
    pub fn Gdiplus_DllExports_GdipGetVisibleClipBoundsI(
        graphics: *mut Gdiplus_GpGraphics,
        rect: *mut Gdiplus_GpRect,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleClipEmpty"]
    pub fn Gdiplus_DllExports_GdipIsVisibleClipEmpty(
        graphics: *mut Gdiplus_GpGraphics,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisiblePoint"]
    pub fn Gdiplus_DllExports_GdipIsVisiblePoint(
        graphics: *mut Gdiplus_GpGraphics,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisiblePointI"]
    pub fn Gdiplus_DllExports_GdipIsVisiblePointI(
        graphics: *mut Gdiplus_GpGraphics,
        x: INT,
        y: INT,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRect"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRect(
        graphics: *mut Gdiplus_GpGraphics,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsVisibleRectI"]
    pub fn Gdiplus_DllExports_GdipIsVisibleRectI(
        graphics: *mut Gdiplus_GpGraphics,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        result: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSaveGraphics"]
    pub fn Gdiplus_DllExports_GdipSaveGraphics(
        graphics: *mut Gdiplus_GpGraphics,
        state: *mut Gdiplus_GraphicsState,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRestoreGraphics"]
    pub fn Gdiplus_DllExports_GdipRestoreGraphics(
        graphics: *mut Gdiplus_GpGraphics,
        state: Gdiplus_GraphicsState,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBeginContainer"]
    pub fn Gdiplus_DllExports_GdipBeginContainer(
        graphics: *mut Gdiplus_GpGraphics,
        dstrect: *const Gdiplus_GpRectF,
        srcrect: *const Gdiplus_GpRectF,
        unit: Gdiplus_GpUnit,
        state: *mut Gdiplus_GraphicsContainer,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBeginContainerI"]
    pub fn Gdiplus_DllExports_GdipBeginContainerI(
        graphics: *mut Gdiplus_GpGraphics,
        dstrect: *const Gdiplus_GpRect,
        srcrect: *const Gdiplus_GpRect,
        unit: Gdiplus_GpUnit,
        state: *mut Gdiplus_GraphicsContainer,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipBeginContainer2"]
    pub fn Gdiplus_DllExports_GdipBeginContainer2(
        graphics: *mut Gdiplus_GpGraphics,
        state: *mut Gdiplus_GraphicsContainer,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEndContainer"]
    pub fn Gdiplus_DllExports_GdipEndContainer(
        graphics: *mut Gdiplus_GpGraphics,
        state: Gdiplus_GraphicsContainer,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileHeaderFromWmf"]
    pub fn Gdiplus_DllExports_GdipGetMetafileHeaderFromWmf(
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileHeaderFromEmf"]
    pub fn Gdiplus_DllExports_GdipGetMetafileHeaderFromEmf(
        hEmf: HENHMETAFILE,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileHeaderFromFile"]
    pub fn Gdiplus_DllExports_GdipGetMetafileHeaderFromFile(
        filename: *const WCHAR,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileHeaderFromStream"]
    pub fn Gdiplus_DllExports_GdipGetMetafileHeaderFromStream(
        stream: *mut IStream,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileHeaderFromMetafile"]
    pub fn Gdiplus_DllExports_GdipGetMetafileHeaderFromMetafile(
        metafile: *mut Gdiplus_GpMetafile,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetHemfFromMetafile"]
    pub fn Gdiplus_DllExports_GdipGetHemfFromMetafile(
        metafile: *mut Gdiplus_GpMetafile,
        hEmf: *mut HENHMETAFILE,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateStreamOnFile"]
    pub fn Gdiplus_DllExports_GdipCreateStreamOnFile(
        filename: *const WCHAR,
        access: UINT,
        stream: *mut *mut IStream,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMetafileFromWmf"]
    pub fn Gdiplus_DllExports_GdipCreateMetafileFromWmf(
        hWmf: HMETAFILE,
        deleteWmf: BOOL,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMetafileFromEmf"]
    pub fn Gdiplus_DllExports_GdipCreateMetafileFromEmf(
        hEmf: HENHMETAFILE,
        deleteEmf: BOOL,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMetafileFromFile"]
    pub fn Gdiplus_DllExports_GdipCreateMetafileFromFile(
        file: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMetafileFromWmfFile"]
    pub fn Gdiplus_DllExports_GdipCreateMetafileFromWmfFile(
        file: *const WCHAR,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateMetafileFromStream"]
    pub fn Gdiplus_DllExports_GdipCreateMetafileFromStream(
        stream: *mut IStream,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafile"]
    pub fn Gdiplus_DllExports_GdipRecordMetafile(
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafileI"]
    pub fn Gdiplus_DllExports_GdipRecordMetafileI(
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafileFileName"]
    pub fn Gdiplus_DllExports_GdipRecordMetafileFileName(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafileFileNameI"]
    pub fn Gdiplus_DllExports_GdipRecordMetafileFileNameI(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafileStream"]
    pub fn Gdiplus_DllExports_GdipRecordMetafileStream(
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipRecordMetafileStreamI"]
    pub fn Gdiplus_DllExports_GdipRecordMetafileStreamI(
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        frameRect: *const Gdiplus_GpRect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        description: *const WCHAR,
        metafile: *mut *mut Gdiplus_GpMetafile,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetMetafileDownLevelRasterizationLimit"]
    pub fn Gdiplus_DllExports_GdipSetMetafileDownLevelRasterizationLimit(
        metafile: *mut Gdiplus_GpMetafile,
        metafileRasterizationLimitDpi: UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetMetafileDownLevelRasterizationLimit"]
    pub fn Gdiplus_DllExports_GdipGetMetafileDownLevelRasterizationLimit(
        metafile: *const Gdiplus_GpMetafile,
        metafileRasterizationLimitDpi: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageDecodersSize"]
    pub fn Gdiplus_DllExports_GdipGetImageDecodersSize(
        numDecoders: *mut UINT,
        size: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageDecoders"]
    pub fn Gdiplus_DllExports_GdipGetImageDecoders(
        numDecoders: UINT,
        size: UINT,
        decoders: *mut Gdiplus_ImageCodecInfo,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageEncodersSize"]
    pub fn Gdiplus_DllExports_GdipGetImageEncodersSize(
        numEncoders: *mut UINT,
        size: *mut UINT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetImageEncoders"]
    pub fn Gdiplus_DllExports_GdipGetImageEncoders(
        numEncoders: UINT,
        size: UINT,
        encoders: *mut Gdiplus_ImageCodecInfo,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipComment"]
    pub fn Gdiplus_DllExports_GdipComment(
        graphics: *mut Gdiplus_GpGraphics,
        sizeData: UINT,
        data: *const BYTE,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFontFamilyFromName"]
    pub fn Gdiplus_DllExports_GdipCreateFontFamilyFromName(
        name: *const WCHAR,
        fontCollection: *mut Gdiplus_GpFontCollection,
        fontFamily: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteFontFamily"]
    pub fn Gdiplus_DllExports_GdipDeleteFontFamily(
        fontFamily: *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneFontFamily"]
    pub fn Gdiplus_DllExports_GdipCloneFontFamily(
        fontFamily: *mut Gdiplus_GpFontFamily,
        clonedFontFamily: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetGenericFontFamilySansSerif"]
    pub fn Gdiplus_DllExports_GdipGetGenericFontFamilySansSerif(
        nativeFamily: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetGenericFontFamilySerif"]
    pub fn Gdiplus_DllExports_GdipGetGenericFontFamilySerif(
        nativeFamily: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetGenericFontFamilyMonospace"]
    pub fn Gdiplus_DllExports_GdipGetGenericFontFamilyMonospace(
        nativeFamily: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFamilyName"]
    pub fn Gdiplus_DllExports_GdipGetFamilyName(
        family: *const Gdiplus_GpFontFamily,
        name: LPWSTR,
        language: LANGID,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipIsStyleAvailable"]
    pub fn Gdiplus_DllExports_GdipIsStyleAvailable(
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        IsStyleAvailable: *mut BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFontCollectionEnumerable"]
    pub fn Gdiplus_DllExports_GdipFontCollectionEnumerable(
        fontCollection: *mut Gdiplus_GpFontCollection,
        graphics: *mut Gdiplus_GpGraphics,
        numFound: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipFontCollectionEnumerate"]
    pub fn Gdiplus_DllExports_GdipFontCollectionEnumerate(
        fontCollection: *mut Gdiplus_GpFontCollection,
        numSought: INT,
        gpfamilies: *mut *mut Gdiplus_GpFontFamily,
        numFound: *mut INT,
        graphics: *mut Gdiplus_GpGraphics,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetEmHeight"]
    pub fn Gdiplus_DllExports_GdipGetEmHeight(
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        EmHeight: *mut UINT16,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCellAscent"]
    pub fn Gdiplus_DllExports_GdipGetCellAscent(
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        CellAscent: *mut UINT16,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetCellDescent"]
    pub fn Gdiplus_DllExports_GdipGetCellDescent(
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        CellDescent: *mut UINT16,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLineSpacing"]
    pub fn Gdiplus_DllExports_GdipGetLineSpacing(
        family: *const Gdiplus_GpFontFamily,
        style: INT,
        LineSpacing: *mut UINT16,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFontFromDC"]
    pub fn Gdiplus_DllExports_GdipCreateFontFromDC(
        hdc: HDC,
        font: *mut *mut Gdiplus_GpFont,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFontFromLogfontA"]
    pub fn Gdiplus_DllExports_GdipCreateFontFromLogfontA(
        hdc: HDC,
        logfont: *const LOGFONTA,
        font: *mut *mut Gdiplus_GpFont,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFontFromLogfontW"]
    pub fn Gdiplus_DllExports_GdipCreateFontFromLogfontW(
        hdc: HDC,
        logfont: *const LOGFONTW,
        font: *mut *mut Gdiplus_GpFont,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateFont"]
    pub fn Gdiplus_DllExports_GdipCreateFont(
        fontFamily: *const Gdiplus_GpFontFamily,
        emSize: Gdiplus_REAL,
        style: INT,
        unit: Gdiplus_Unit,
        font: *mut *mut Gdiplus_GpFont,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneFont"]
    pub fn Gdiplus_DllExports_GdipCloneFont(
        font: *mut Gdiplus_GpFont,
        cloneFont: *mut *mut Gdiplus_GpFont,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteFont"]
    pub fn Gdiplus_DllExports_GdipDeleteFont(font: *mut Gdiplus_GpFont) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFamily"]
    pub fn Gdiplus_DllExports_GdipGetFamily(
        font: *mut Gdiplus_GpFont,
        family: *mut *mut Gdiplus_GpFontFamily,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontStyle"]
    pub fn Gdiplus_DllExports_GdipGetFontStyle(
        font: *mut Gdiplus_GpFont,
        style: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontSize"]
    pub fn Gdiplus_DllExports_GdipGetFontSize(
        font: *mut Gdiplus_GpFont,
        size: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontUnit"]
    pub fn Gdiplus_DllExports_GdipGetFontUnit(
        font: *mut Gdiplus_GpFont,
        unit: *mut Gdiplus_Unit,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontHeight"]
    pub fn Gdiplus_DllExports_GdipGetFontHeight(
        font: *const Gdiplus_GpFont,
        graphics: *const Gdiplus_GpGraphics,
        height: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontHeightGivenDPI"]
    pub fn Gdiplus_DllExports_GdipGetFontHeightGivenDPI(
        font: *const Gdiplus_GpFont,
        dpi: Gdiplus_REAL,
        height: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLogFontA"]
    pub fn Gdiplus_DllExports_GdipGetLogFontA(
        font: *mut Gdiplus_GpFont,
        graphics: *mut Gdiplus_GpGraphics,
        logfontA: *mut LOGFONTA,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetLogFontW"]
    pub fn Gdiplus_DllExports_GdipGetLogFontW(
        font: *mut Gdiplus_GpFont,
        graphics: *mut Gdiplus_GpGraphics,
        logfontW: *mut LOGFONTW,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipNewInstalledFontCollection"]
    pub fn Gdiplus_DllExports_GdipNewInstalledFontCollection(
        fontCollection: *mut *mut Gdiplus_GpFontCollection,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipNewPrivateFontCollection"]
    pub fn Gdiplus_DllExports_GdipNewPrivateFontCollection(
        fontCollection: *mut *mut Gdiplus_GpFontCollection,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeletePrivateFontCollection"]
    pub fn Gdiplus_DllExports_GdipDeletePrivateFontCollection(
        fontCollection: *mut *mut Gdiplus_GpFontCollection,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontCollectionFamilyCount"]
    pub fn Gdiplus_DllExports_GdipGetFontCollectionFamilyCount(
        fontCollection: *mut Gdiplus_GpFontCollection,
        numFound: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetFontCollectionFamilyList"]
    pub fn Gdiplus_DllExports_GdipGetFontCollectionFamilyList(
        fontCollection: *mut Gdiplus_GpFontCollection,
        numSought: INT,
        gpfamilies: *mut *mut Gdiplus_GpFontFamily,
        numFound: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPrivateAddFontFile"]
    pub fn Gdiplus_DllExports_GdipPrivateAddFontFile(
        fontCollection: *mut Gdiplus_GpFontCollection,
        filename: *const WCHAR,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipPrivateAddMemoryFont"]
    pub fn Gdiplus_DllExports_GdipPrivateAddMemoryFont(
        fontCollection: *mut Gdiplus_GpFontCollection,
        memory: *const c_void,
        length: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawString"]
    pub fn Gdiplus_DllExports_GdipDrawString(
        graphics: *mut Gdiplus_GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const Gdiplus_GpFont,
        layoutRect: *const Gdiplus_RectF,
        stringFormat: *const Gdiplus_GpStringFormat,
        brush: *const Gdiplus_GpBrush,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMeasureString"]
    pub fn Gdiplus_DllExports_GdipMeasureString(
        graphics: *mut Gdiplus_GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const Gdiplus_GpFont,
        layoutRect: *const Gdiplus_RectF,
        stringFormat: *const Gdiplus_GpStringFormat,
        boundingBox: *mut Gdiplus_RectF,
        codepointsFitted: *mut INT,
        linesFilled: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMeasureCharacterRanges"]
    pub fn Gdiplus_DllExports_GdipMeasureCharacterRanges(
        graphics: *mut Gdiplus_GpGraphics,
        string: *const WCHAR,
        length: INT,
        font: *const Gdiplus_GpFont,
        layoutRect: *const Gdiplus_RectF,
        stringFormat: *const Gdiplus_GpStringFormat,
        regionCount: INT,
        regions: *mut *mut Gdiplus_GpRegion,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawDriverString"]
    pub fn Gdiplus_DllExports_GdipDrawDriverString(
        graphics: *mut Gdiplus_GpGraphics,
        text: *const UINT16,
        length: INT,
        font: *const Gdiplus_GpFont,
        brush: *const Gdiplus_GpBrush,
        positions: *const Gdiplus_PointF,
        flags: INT,
        matrix: *const Gdiplus_GpMatrix,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipMeasureDriverString"]
    pub fn Gdiplus_DllExports_GdipMeasureDriverString(
        graphics: *mut Gdiplus_GpGraphics,
        text: *const UINT16,
        length: INT,
        font: *const Gdiplus_GpFont,
        positions: *const Gdiplus_PointF,
        flags: INT,
        matrix: *const Gdiplus_GpMatrix,
        boundingBox: *mut Gdiplus_RectF,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateStringFormat"]
    pub fn Gdiplus_DllExports_GdipCreateStringFormat(
        formatAttributes: INT,
        language: LANGID,
        format: *mut *mut Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipStringFormatGetGenericDefault"]
    pub fn Gdiplus_DllExports_GdipStringFormatGetGenericDefault(
        format: *mut *mut Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipStringFormatGetGenericTypographic"]
    pub fn Gdiplus_DllExports_GdipStringFormatGetGenericTypographic(
        format: *mut *mut Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteStringFormat"]
    pub fn Gdiplus_DllExports_GdipDeleteStringFormat(
        format: *mut Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCloneStringFormat"]
    pub fn Gdiplus_DllExports_GdipCloneStringFormat(
        format: *const Gdiplus_GpStringFormat,
        newFormat: *mut *mut Gdiplus_GpStringFormat,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatFlags"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatFlags(
        format: *mut Gdiplus_GpStringFormat,
        flags: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatFlags"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatFlags(
        format: *const Gdiplus_GpStringFormat,
        flags: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatAlign"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatAlign(
        format: *mut Gdiplus_GpStringFormat,
        align: Gdiplus_StringAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatAlign"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatAlign(
        format: *const Gdiplus_GpStringFormat,
        align: *mut Gdiplus_StringAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatLineAlign"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatLineAlign(
        format: *mut Gdiplus_GpStringFormat,
        align: Gdiplus_StringAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatLineAlign"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatLineAlign(
        format: *const Gdiplus_GpStringFormat,
        align: *mut Gdiplus_StringAlignment,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatTrimming"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatTrimming(
        format: *mut Gdiplus_GpStringFormat,
        trimming: Gdiplus_StringTrimming,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatTrimming"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatTrimming(
        format: *const Gdiplus_GpStringFormat,
        trimming: *mut Gdiplus_StringTrimming,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatHotkeyPrefix"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatHotkeyPrefix(
        format: *mut Gdiplus_GpStringFormat,
        hotkeyPrefix: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatHotkeyPrefix"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatHotkeyPrefix(
        format: *const Gdiplus_GpStringFormat,
        hotkeyPrefix: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatTabStops"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatTabStops(
        format: *mut Gdiplus_GpStringFormat,
        firstTabOffset: Gdiplus_REAL,
        count: INT,
        tabStops: *const Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatTabStops"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatTabStops(
        format: *const Gdiplus_GpStringFormat,
        count: INT,
        firstTabOffset: *mut Gdiplus_REAL,
        tabStops: *mut Gdiplus_REAL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatTabStopCount"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatTabStopCount(
        format: *const Gdiplus_GpStringFormat,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatDigitSubstitution"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatDigitSubstitution(
        format: *mut Gdiplus_GpStringFormat,
        language: LANGID,
        substitute: Gdiplus_StringDigitSubstitute,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatDigitSubstitution"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatDigitSubstitution(
        format: *const Gdiplus_GpStringFormat,
        language: *mut LANGID,
        substitute: *mut Gdiplus_StringDigitSubstitute,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipGetStringFormatMeasurableCharacterRangeCount"]
    pub fn Gdiplus_DllExports_GdipGetStringFormatMeasurableCharacterRangeCount(
        format: *const Gdiplus_GpStringFormat,
        count: *mut INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipSetStringFormatMeasurableCharacterRanges"]
    pub fn Gdiplus_DllExports_GdipSetStringFormatMeasurableCharacterRanges(
        format: *mut Gdiplus_GpStringFormat,
        rangeCount: INT,
        ranges: *const Gdiplus_CharacterRange,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipCreateCachedBitmap"]
    pub fn Gdiplus_DllExports_GdipCreateCachedBitmap(
        bitmap: *mut Gdiplus_GpBitmap,
        graphics: *mut Gdiplus_GpGraphics,
        cachedBitmap: *mut *mut Gdiplus_GpCachedBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDeleteCachedBitmap"]
    pub fn Gdiplus_DllExports_GdipDeleteCachedBitmap(
        cachedBitmap: *mut Gdiplus_GpCachedBitmap,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipDrawCachedBitmap"]
    pub fn Gdiplus_DllExports_GdipDrawCachedBitmap(
        graphics: *mut Gdiplus_GpGraphics,
        cachedBitmap: *mut Gdiplus_GpCachedBitmap,
        x: INT,
        y: INT,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipEmfToWmfBits"]
    pub fn Gdiplus_DllExports_GdipEmfToWmfBits(
        hemf: HENHMETAFILE,
        cbData16: UINT,
        pData16: LPBYTE,
        iMapMode: INT,
        eFlags: INT,
    ) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}GdipSetImageAttributesCachedBackground"]
    pub fn Gdiplus_DllExports_GdipSetImageAttributesCachedBackground(
        imageattr: *mut Gdiplus_GpImageAttributes,
        enableFlag: BOOL,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdipTestControl"]
    pub fn Gdiplus_DllExports_GdipTestControl(
        control: Gdiplus_GpTestControlEnum,
        param: *mut c_void,
    ) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdiplusNotificationHook"]
    pub fn Gdiplus_DllExports_GdiplusNotificationHook(token: *mut ULONG_PTR) -> Gdiplus_GpStatus;
}
extern "C" {
    #[link_name = "\u{1}GdiplusNotificationUnhook"]
    pub fn Gdiplus_DllExports_GdiplusNotificationUnhook(token: ULONG_PTR);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GdiplusBase {
    pub _address: u8,
}
pub type Gdiplus_GraphicsState = UINT;
pub type Gdiplus_GraphicsContainer = UINT;
pub const Gdiplus_FillMode_FillModeAlternate: Gdiplus_FillMode = 0;
pub const Gdiplus_FillMode_FillModeWinding: Gdiplus_FillMode = 1;
pub type Gdiplus_FillMode = c_int;
pub const Gdiplus_CompositingMode_CompositingModeSourceOver: Gdiplus_CompositingMode = 0;
pub const Gdiplus_CompositingMode_CompositingModeSourceCopy: Gdiplus_CompositingMode = 1;
pub type Gdiplus_CompositingMode = c_int;
pub const Gdiplus_CompositingQuality_CompositingQualityInvalid: Gdiplus_CompositingQuality = -1;
pub const Gdiplus_CompositingQuality_CompositingQualityDefault: Gdiplus_CompositingQuality = 0;
pub const Gdiplus_CompositingQuality_CompositingQualityHighSpeed: Gdiplus_CompositingQuality = 1;
pub const Gdiplus_CompositingQuality_CompositingQualityHighQuality: Gdiplus_CompositingQuality = 2;
pub const Gdiplus_CompositingQuality_CompositingQualityGammaCorrected: Gdiplus_CompositingQuality =
    3;
pub const Gdiplus_CompositingQuality_CompositingQualityAssumeLinear: Gdiplus_CompositingQuality = 4;
pub type Gdiplus_CompositingQuality = c_int;
pub const Gdiplus_Unit_UnitWorld: Gdiplus_Unit = 0;
pub const Gdiplus_Unit_UnitDisplay: Gdiplus_Unit = 1;
pub const Gdiplus_Unit_UnitPixel: Gdiplus_Unit = 2;
pub const Gdiplus_Unit_UnitPoint: Gdiplus_Unit = 3;
pub const Gdiplus_Unit_UnitInch: Gdiplus_Unit = 4;
pub const Gdiplus_Unit_UnitDocument: Gdiplus_Unit = 5;
pub const Gdiplus_Unit_UnitMillimeter: Gdiplus_Unit = 6;
pub type Gdiplus_Unit = c_int;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitPixel: Gdiplus_MetafileFrameUnit = 2;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitPoint: Gdiplus_MetafileFrameUnit = 3;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitInch: Gdiplus_MetafileFrameUnit = 4;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitDocument: Gdiplus_MetafileFrameUnit = 5;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitMillimeter: Gdiplus_MetafileFrameUnit = 6;
pub const Gdiplus_MetafileFrameUnit_MetafileFrameUnitGdi: Gdiplus_MetafileFrameUnit = 7;
pub type Gdiplus_MetafileFrameUnit = c_int;
pub const Gdiplus_CoordinateSpace_CoordinateSpaceWorld: Gdiplus_CoordinateSpace = 0;
pub const Gdiplus_CoordinateSpace_CoordinateSpacePage: Gdiplus_CoordinateSpace = 1;
pub const Gdiplus_CoordinateSpace_CoordinateSpaceDevice: Gdiplus_CoordinateSpace = 2;
pub type Gdiplus_CoordinateSpace = c_int;
pub const Gdiplus_WrapMode_WrapModeTile: Gdiplus_WrapMode = 0;
pub const Gdiplus_WrapMode_WrapModeTileFlipX: Gdiplus_WrapMode = 1;
pub const Gdiplus_WrapMode_WrapModeTileFlipY: Gdiplus_WrapMode = 2;
pub const Gdiplus_WrapMode_WrapModeTileFlipXY: Gdiplus_WrapMode = 3;
pub const Gdiplus_WrapMode_WrapModeClamp: Gdiplus_WrapMode = 4;
pub type Gdiplus_WrapMode = c_int;
pub const Gdiplus_HatchStyle_HatchStyleHorizontal: Gdiplus_HatchStyle = 0;
pub const Gdiplus_HatchStyle_HatchStyleVertical: Gdiplus_HatchStyle = 1;
pub const Gdiplus_HatchStyle_HatchStyleForwardDiagonal: Gdiplus_HatchStyle = 2;
pub const Gdiplus_HatchStyle_HatchStyleBackwardDiagonal: Gdiplus_HatchStyle = 3;
pub const Gdiplus_HatchStyle_HatchStyleCross: Gdiplus_HatchStyle = 4;
pub const Gdiplus_HatchStyle_HatchStyleDiagonalCross: Gdiplus_HatchStyle = 5;
pub const Gdiplus_HatchStyle_HatchStyle05Percent: Gdiplus_HatchStyle = 6;
pub const Gdiplus_HatchStyle_HatchStyle10Percent: Gdiplus_HatchStyle = 7;
pub const Gdiplus_HatchStyle_HatchStyle20Percent: Gdiplus_HatchStyle = 8;
pub const Gdiplus_HatchStyle_HatchStyle25Percent: Gdiplus_HatchStyle = 9;
pub const Gdiplus_HatchStyle_HatchStyle30Percent: Gdiplus_HatchStyle = 10;
pub const Gdiplus_HatchStyle_HatchStyle40Percent: Gdiplus_HatchStyle = 11;
pub const Gdiplus_HatchStyle_HatchStyle50Percent: Gdiplus_HatchStyle = 12;
pub const Gdiplus_HatchStyle_HatchStyle60Percent: Gdiplus_HatchStyle = 13;
pub const Gdiplus_HatchStyle_HatchStyle70Percent: Gdiplus_HatchStyle = 14;
pub const Gdiplus_HatchStyle_HatchStyle75Percent: Gdiplus_HatchStyle = 15;
pub const Gdiplus_HatchStyle_HatchStyle80Percent: Gdiplus_HatchStyle = 16;
pub const Gdiplus_HatchStyle_HatchStyle90Percent: Gdiplus_HatchStyle = 17;
pub const Gdiplus_HatchStyle_HatchStyleLightDownwardDiagonal: Gdiplus_HatchStyle = 18;
pub const Gdiplus_HatchStyle_HatchStyleLightUpwardDiagonal: Gdiplus_HatchStyle = 19;
pub const Gdiplus_HatchStyle_HatchStyleDarkDownwardDiagonal: Gdiplus_HatchStyle = 20;
pub const Gdiplus_HatchStyle_HatchStyleDarkUpwardDiagonal: Gdiplus_HatchStyle = 21;
pub const Gdiplus_HatchStyle_HatchStyleWideDownwardDiagonal: Gdiplus_HatchStyle = 22;
pub const Gdiplus_HatchStyle_HatchStyleWideUpwardDiagonal: Gdiplus_HatchStyle = 23;
pub const Gdiplus_HatchStyle_HatchStyleLightVertical: Gdiplus_HatchStyle = 24;
pub const Gdiplus_HatchStyle_HatchStyleLightHorizontal: Gdiplus_HatchStyle = 25;
pub const Gdiplus_HatchStyle_HatchStyleNarrowVertical: Gdiplus_HatchStyle = 26;
pub const Gdiplus_HatchStyle_HatchStyleNarrowHorizontal: Gdiplus_HatchStyle = 27;
pub const Gdiplus_HatchStyle_HatchStyleDarkVertical: Gdiplus_HatchStyle = 28;
pub const Gdiplus_HatchStyle_HatchStyleDarkHorizontal: Gdiplus_HatchStyle = 29;
pub const Gdiplus_HatchStyle_HatchStyleDashedDownwardDiagonal: Gdiplus_HatchStyle = 30;
pub const Gdiplus_HatchStyle_HatchStyleDashedUpwardDiagonal: Gdiplus_HatchStyle = 31;
pub const Gdiplus_HatchStyle_HatchStyleDashedHorizontal: Gdiplus_HatchStyle = 32;
pub const Gdiplus_HatchStyle_HatchStyleDashedVertical: Gdiplus_HatchStyle = 33;
pub const Gdiplus_HatchStyle_HatchStyleSmallConfetti: Gdiplus_HatchStyle = 34;
pub const Gdiplus_HatchStyle_HatchStyleLargeConfetti: Gdiplus_HatchStyle = 35;
pub const Gdiplus_HatchStyle_HatchStyleZigZag: Gdiplus_HatchStyle = 36;
pub const Gdiplus_HatchStyle_HatchStyleWave: Gdiplus_HatchStyle = 37;
pub const Gdiplus_HatchStyle_HatchStyleDiagonalBrick: Gdiplus_HatchStyle = 38;
pub const Gdiplus_HatchStyle_HatchStyleHorizontalBrick: Gdiplus_HatchStyle = 39;
pub const Gdiplus_HatchStyle_HatchStyleWeave: Gdiplus_HatchStyle = 40;
pub const Gdiplus_HatchStyle_HatchStylePlaid: Gdiplus_HatchStyle = 41;
pub const Gdiplus_HatchStyle_HatchStyleDivot: Gdiplus_HatchStyle = 42;
pub const Gdiplus_HatchStyle_HatchStyleDottedGrid: Gdiplus_HatchStyle = 43;
pub const Gdiplus_HatchStyle_HatchStyleDottedDiamond: Gdiplus_HatchStyle = 44;
pub const Gdiplus_HatchStyle_HatchStyleShingle: Gdiplus_HatchStyle = 45;
pub const Gdiplus_HatchStyle_HatchStyleTrellis: Gdiplus_HatchStyle = 46;
pub const Gdiplus_HatchStyle_HatchStyleSphere: Gdiplus_HatchStyle = 47;
pub const Gdiplus_HatchStyle_HatchStyleSmallGrid: Gdiplus_HatchStyle = 48;
pub const Gdiplus_HatchStyle_HatchStyleSmallCheckerBoard: Gdiplus_HatchStyle = 49;
pub const Gdiplus_HatchStyle_HatchStyleLargeCheckerBoard: Gdiplus_HatchStyle = 50;
pub const Gdiplus_HatchStyle_HatchStyleOutlinedDiamond: Gdiplus_HatchStyle = 51;
pub const Gdiplus_HatchStyle_HatchStyleSolidDiamond: Gdiplus_HatchStyle = 52;
pub const Gdiplus_HatchStyle_HatchStyleTotal: Gdiplus_HatchStyle = 53;
pub const Gdiplus_HatchStyle_HatchStyleLargeGrid: Gdiplus_HatchStyle = 4;
pub const Gdiplus_HatchStyle_HatchStyleMin: Gdiplus_HatchStyle = 0;
pub const Gdiplus_HatchStyle_HatchStyleMax: Gdiplus_HatchStyle = 52;
pub type Gdiplus_HatchStyle = c_int;
pub const Gdiplus_DashStyle_DashStyleSolid: Gdiplus_DashStyle = 0;
pub const Gdiplus_DashStyle_DashStyleDash: Gdiplus_DashStyle = 1;
pub const Gdiplus_DashStyle_DashStyleDot: Gdiplus_DashStyle = 2;
pub const Gdiplus_DashStyle_DashStyleDashDot: Gdiplus_DashStyle = 3;
pub const Gdiplus_DashStyle_DashStyleDashDotDot: Gdiplus_DashStyle = 4;
pub const Gdiplus_DashStyle_DashStyleCustom: Gdiplus_DashStyle = 5;
pub type Gdiplus_DashStyle = c_int;
pub const Gdiplus_DashCap_DashCapFlat: Gdiplus_DashCap = 0;
pub const Gdiplus_DashCap_DashCapRound: Gdiplus_DashCap = 2;
pub const Gdiplus_DashCap_DashCapTriangle: Gdiplus_DashCap = 3;
pub type Gdiplus_DashCap = c_int;
pub const Gdiplus_LineCap_LineCapFlat: Gdiplus_LineCap = 0;
pub const Gdiplus_LineCap_LineCapSquare: Gdiplus_LineCap = 1;
pub const Gdiplus_LineCap_LineCapRound: Gdiplus_LineCap = 2;
pub const Gdiplus_LineCap_LineCapTriangle: Gdiplus_LineCap = 3;
pub const Gdiplus_LineCap_LineCapNoAnchor: Gdiplus_LineCap = 16;
pub const Gdiplus_LineCap_LineCapSquareAnchor: Gdiplus_LineCap = 17;
pub const Gdiplus_LineCap_LineCapRoundAnchor: Gdiplus_LineCap = 18;
pub const Gdiplus_LineCap_LineCapDiamondAnchor: Gdiplus_LineCap = 19;
pub const Gdiplus_LineCap_LineCapArrowAnchor: Gdiplus_LineCap = 20;
pub const Gdiplus_LineCap_LineCapCustom: Gdiplus_LineCap = 255;
pub const Gdiplus_LineCap_LineCapAnchorMask: Gdiplus_LineCap = 240;
pub type Gdiplus_LineCap = c_int;
pub const Gdiplus_CustomLineCapType_CustomLineCapTypeDefault: Gdiplus_CustomLineCapType = 0;
pub const Gdiplus_CustomLineCapType_CustomLineCapTypeAdjustableArrow: Gdiplus_CustomLineCapType = 1;
pub type Gdiplus_CustomLineCapType = c_int;
pub const Gdiplus_LineJoin_LineJoinMiter: Gdiplus_LineJoin = 0;
pub const Gdiplus_LineJoin_LineJoinBevel: Gdiplus_LineJoin = 1;
pub const Gdiplus_LineJoin_LineJoinRound: Gdiplus_LineJoin = 2;
pub const Gdiplus_LineJoin_LineJoinMiterClipped: Gdiplus_LineJoin = 3;
pub type Gdiplus_LineJoin = c_int;
pub const Gdiplus_WarpMode_WarpModePerspective: Gdiplus_WarpMode = 0;
pub const Gdiplus_WarpMode_WarpModeBilinear: Gdiplus_WarpMode = 1;
pub type Gdiplus_WarpMode = c_int;
pub const Gdiplus_LinearGradientMode_LinearGradientModeHorizontal: Gdiplus_LinearGradientMode = 0;
pub const Gdiplus_LinearGradientMode_LinearGradientModeVertical: Gdiplus_LinearGradientMode = 1;
pub const Gdiplus_LinearGradientMode_LinearGradientModeForwardDiagonal: Gdiplus_LinearGradientMode =
    2;
pub const Gdiplus_LinearGradientMode_LinearGradientModeBackwardDiagonal:
    Gdiplus_LinearGradientMode = 3;
pub type Gdiplus_LinearGradientMode = c_int;
pub const Gdiplus_CombineMode_CombineModeReplace: Gdiplus_CombineMode = 0;
pub const Gdiplus_CombineMode_CombineModeIntersect: Gdiplus_CombineMode = 1;
pub const Gdiplus_CombineMode_CombineModeUnion: Gdiplus_CombineMode = 2;
pub const Gdiplus_CombineMode_CombineModeXor: Gdiplus_CombineMode = 3;
pub const Gdiplus_CombineMode_CombineModeExclude: Gdiplus_CombineMode = 4;
pub const Gdiplus_CombineMode_CombineModeComplement: Gdiplus_CombineMode = 5;
pub type Gdiplus_CombineMode = c_int;
pub const Gdiplus_ImageType_ImageTypeUnknown: Gdiplus_ImageType = 0;
pub const Gdiplus_ImageType_ImageTypeBitmap: Gdiplus_ImageType = 1;
pub const Gdiplus_ImageType_ImageTypeMetafile: Gdiplus_ImageType = 2;
pub type Gdiplus_ImageType = c_int;
pub const Gdiplus_InterpolationMode_InterpolationModeInvalid: Gdiplus_InterpolationMode = -1;
pub const Gdiplus_InterpolationMode_InterpolationModeDefault: Gdiplus_InterpolationMode = 0;
pub const Gdiplus_InterpolationMode_InterpolationModeLowQuality: Gdiplus_InterpolationMode = 1;
pub const Gdiplus_InterpolationMode_InterpolationModeHighQuality: Gdiplus_InterpolationMode = 2;
pub const Gdiplus_InterpolationMode_InterpolationModeBilinear: Gdiplus_InterpolationMode = 3;
pub const Gdiplus_InterpolationMode_InterpolationModeBicubic: Gdiplus_InterpolationMode = 4;
pub const Gdiplus_InterpolationMode_InterpolationModeNearestNeighbor: Gdiplus_InterpolationMode = 5;
pub const Gdiplus_InterpolationMode_InterpolationModeHighQualityBilinear:
    Gdiplus_InterpolationMode = 6;
pub const Gdiplus_InterpolationMode_InterpolationModeHighQualityBicubic: Gdiplus_InterpolationMode =
    7;
pub type Gdiplus_InterpolationMode = c_int;
pub const Gdiplus_PenAlignment_PenAlignmentCenter: Gdiplus_PenAlignment = 0;
pub const Gdiplus_PenAlignment_PenAlignmentInset: Gdiplus_PenAlignment = 1;
pub type Gdiplus_PenAlignment = c_int;
pub const Gdiplus_BrushType_BrushTypeSolidColor: Gdiplus_BrushType = 0;
pub const Gdiplus_BrushType_BrushTypeHatchFill: Gdiplus_BrushType = 1;
pub const Gdiplus_BrushType_BrushTypeTextureFill: Gdiplus_BrushType = 2;
pub const Gdiplus_BrushType_BrushTypePathGradient: Gdiplus_BrushType = 3;
pub const Gdiplus_BrushType_BrushTypeLinearGradient: Gdiplus_BrushType = 4;
pub type Gdiplus_BrushType = c_int;
pub const Gdiplus_PenType_PenTypeSolidColor: Gdiplus_PenType = 0;
pub const Gdiplus_PenType_PenTypeHatchFill: Gdiplus_PenType = 1;
pub const Gdiplus_PenType_PenTypeTextureFill: Gdiplus_PenType = 2;
pub const Gdiplus_PenType_PenTypePathGradient: Gdiplus_PenType = 3;
pub const Gdiplus_PenType_PenTypeLinearGradient: Gdiplus_PenType = 4;
pub const Gdiplus_PenType_PenTypeUnknown: Gdiplus_PenType = -1;
pub type Gdiplus_PenType = c_int;
pub const Gdiplus_MatrixOrder_MatrixOrderPrepend: Gdiplus_MatrixOrder = 0;
pub const Gdiplus_MatrixOrder_MatrixOrderAppend: Gdiplus_MatrixOrder = 1;
pub type Gdiplus_MatrixOrder = c_int;
pub const Gdiplus_SmoothingMode_SmoothingModeInvalid: Gdiplus_SmoothingMode = -1;
pub const Gdiplus_SmoothingMode_SmoothingModeDefault: Gdiplus_SmoothingMode = 0;
pub const Gdiplus_SmoothingMode_SmoothingModeHighSpeed: Gdiplus_SmoothingMode = 1;
pub const Gdiplus_SmoothingMode_SmoothingModeHighQuality: Gdiplus_SmoothingMode = 2;
pub const Gdiplus_SmoothingMode_SmoothingModeNone: Gdiplus_SmoothingMode = 3;
pub const Gdiplus_SmoothingMode_SmoothingModeAntiAlias: Gdiplus_SmoothingMode = 4;
pub type Gdiplus_SmoothingMode = c_int;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeInvalid: Gdiplus_PixelOffsetMode = -1;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeDefault: Gdiplus_PixelOffsetMode = 0;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeHighSpeed: Gdiplus_PixelOffsetMode = 1;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeHighQuality: Gdiplus_PixelOffsetMode = 2;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeNone: Gdiplus_PixelOffsetMode = 3;
pub const Gdiplus_PixelOffsetMode_PixelOffsetModeHalf: Gdiplus_PixelOffsetMode = 4;
pub type Gdiplus_PixelOffsetMode = c_int;
pub const Gdiplus_TextRenderingHint_TextRenderingHintSystemDefault: Gdiplus_TextRenderingHint = 0;
pub const Gdiplus_TextRenderingHint_TextRenderingHintSingleBitPerPixelGridFit:
    Gdiplus_TextRenderingHint = 1;
pub const Gdiplus_TextRenderingHint_TextRenderingHintSingleBitPerPixel: Gdiplus_TextRenderingHint =
    2;
pub const Gdiplus_TextRenderingHint_TextRenderingHintAntiAliasGridFit: Gdiplus_TextRenderingHint =
    3;
pub const Gdiplus_TextRenderingHint_TextRenderingHintAntiAlias: Gdiplus_TextRenderingHint = 4;
pub const Gdiplus_TextRenderingHint_TextRenderingHintClearTypeGridFit: Gdiplus_TextRenderingHint =
    5;
pub type Gdiplus_TextRenderingHint = c_int;
pub const Gdiplus_MetafileType_MetafileTypeInvalid: Gdiplus_MetafileType = 0;
pub const Gdiplus_MetafileType_MetafileTypeWmf: Gdiplus_MetafileType = 1;
pub const Gdiplus_MetafileType_MetafileTypeWmfPlaceable: Gdiplus_MetafileType = 2;
pub const Gdiplus_MetafileType_MetafileTypeEmf: Gdiplus_MetafileType = 3;
pub const Gdiplus_MetafileType_MetafileTypeEmfPlusOnly: Gdiplus_MetafileType = 4;
pub const Gdiplus_MetafileType_MetafileTypeEmfPlusDual: Gdiplus_MetafileType = 5;
pub type Gdiplus_MetafileType = c_int;
pub const Gdiplus_EmfType_EmfTypeEmfOnly: Gdiplus_EmfType = 3;
pub const Gdiplus_EmfType_EmfTypeEmfPlusOnly: Gdiplus_EmfType = 4;
pub const Gdiplus_EmfType_EmfTypeEmfPlusDual: Gdiplus_EmfType = 5;
pub type Gdiplus_EmfType = c_int;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetBkColor: Gdiplus_EmfPlusRecordType = 66049;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetBkMode: Gdiplus_EmfPlusRecordType = 65794;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetMapMode: Gdiplus_EmfPlusRecordType = 65795;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetROP2: Gdiplus_EmfPlusRecordType = 65796;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetRelAbs: Gdiplus_EmfPlusRecordType = 65797;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetPolyFillMode: Gdiplus_EmfPlusRecordType = 65798;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetStretchBltMode: Gdiplus_EmfPlusRecordType =
    65799;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetTextCharExtra: Gdiplus_EmfPlusRecordType =
    65800;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetTextColor: Gdiplus_EmfPlusRecordType = 66057;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetTextJustification: Gdiplus_EmfPlusRecordType =
    66058;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetWindowOrg: Gdiplus_EmfPlusRecordType = 66059;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetWindowExt: Gdiplus_EmfPlusRecordType = 66060;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetViewportOrg: Gdiplus_EmfPlusRecordType = 66061;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetViewportExt: Gdiplus_EmfPlusRecordType = 66062;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeOffsetWindowOrg: Gdiplus_EmfPlusRecordType = 66063;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeScaleWindowExt: Gdiplus_EmfPlusRecordType = 66576;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeOffsetViewportOrg: Gdiplus_EmfPlusRecordType =
    66065;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeScaleViewportExt: Gdiplus_EmfPlusRecordType =
    66578;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeLineTo: Gdiplus_EmfPlusRecordType = 66067;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeMoveTo: Gdiplus_EmfPlusRecordType = 66068;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeExcludeClipRect: Gdiplus_EmfPlusRecordType = 66581;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeIntersectClipRect: Gdiplus_EmfPlusRecordType =
    66582;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeArc: Gdiplus_EmfPlusRecordType = 67607;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeEllipse: Gdiplus_EmfPlusRecordType = 66584;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeFloodFill: Gdiplus_EmfPlusRecordType = 66585;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePie: Gdiplus_EmfPlusRecordType = 67610;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeRectangle: Gdiplus_EmfPlusRecordType = 66587;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeRoundRect: Gdiplus_EmfPlusRecordType = 67100;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePatBlt: Gdiplus_EmfPlusRecordType = 67101;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSaveDC: Gdiplus_EmfPlusRecordType = 65566;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetPixel: Gdiplus_EmfPlusRecordType = 66591;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeOffsetClipRgn: Gdiplus_EmfPlusRecordType = 66080;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeTextOut: Gdiplus_EmfPlusRecordType = 66849;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeBitBlt: Gdiplus_EmfPlusRecordType = 67874;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeStretchBlt: Gdiplus_EmfPlusRecordType = 68387;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePolygon: Gdiplus_EmfPlusRecordType = 66340;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePolyline: Gdiplus_EmfPlusRecordType = 66341;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeEscape: Gdiplus_EmfPlusRecordType = 67110;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeRestoreDC: Gdiplus_EmfPlusRecordType = 65831;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeFillRegion: Gdiplus_EmfPlusRecordType = 66088;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeFrameRegion: Gdiplus_EmfPlusRecordType = 66601;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeInvertRegion: Gdiplus_EmfPlusRecordType = 65834;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePaintRegion: Gdiplus_EmfPlusRecordType = 65835;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSelectClipRegion: Gdiplus_EmfPlusRecordType =
    65836;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSelectObject: Gdiplus_EmfPlusRecordType = 65837;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetTextAlign: Gdiplus_EmfPlusRecordType = 65838;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeDrawText: Gdiplus_EmfPlusRecordType = 67119;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeChord: Gdiplus_EmfPlusRecordType = 67632;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetMapperFlags: Gdiplus_EmfPlusRecordType = 66097;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeExtTextOut: Gdiplus_EmfPlusRecordType = 68146;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetDIBToDev: Gdiplus_EmfPlusRecordType = 68915;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSelectPalette: Gdiplus_EmfPlusRecordType = 66100;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeRealizePalette: Gdiplus_EmfPlusRecordType = 65589;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeAnimatePalette: Gdiplus_EmfPlusRecordType = 66614;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetPalEntries: Gdiplus_EmfPlusRecordType = 65591;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypePolyPolygon: Gdiplus_EmfPlusRecordType = 66872;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeResizePalette: Gdiplus_EmfPlusRecordType = 65849;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeDIBBitBlt: Gdiplus_EmfPlusRecordType = 67904;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeDIBStretchBlt: Gdiplus_EmfPlusRecordType = 68417;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeDIBCreatePatternBrush: Gdiplus_EmfPlusRecordType =
    65858;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeStretchDIB: Gdiplus_EmfPlusRecordType = 69443;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeExtFloodFill: Gdiplus_EmfPlusRecordType = 66888;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeSetLayout: Gdiplus_EmfPlusRecordType = 65865;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeResetDC: Gdiplus_EmfPlusRecordType = 65868;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeStartDoc: Gdiplus_EmfPlusRecordType = 65869;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeStartPage: Gdiplus_EmfPlusRecordType = 65615;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeEndPage: Gdiplus_EmfPlusRecordType = 65616;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeAbortDoc: Gdiplus_EmfPlusRecordType = 65618;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeEndDoc: Gdiplus_EmfPlusRecordType = 65630;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeDeleteObject: Gdiplus_EmfPlusRecordType = 66032;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreatePalette: Gdiplus_EmfPlusRecordType = 65783;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateBrush: Gdiplus_EmfPlusRecordType = 65784;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreatePatternBrush: Gdiplus_EmfPlusRecordType =
    66041;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreatePenIndirect: Gdiplus_EmfPlusRecordType =
    66298;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateFontIndirect: Gdiplus_EmfPlusRecordType =
    66299;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateBrushIndirect: Gdiplus_EmfPlusRecordType =
    66300;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateBitmapIndirect: Gdiplus_EmfPlusRecordType =
    66301;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateBitmap: Gdiplus_EmfPlusRecordType = 67326;
pub const Gdiplus_EmfPlusRecordType_WmfRecordTypeCreateRegion: Gdiplus_EmfPlusRecordType = 67327;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeHeader: Gdiplus_EmfPlusRecordType = 1;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyBezier: Gdiplus_EmfPlusRecordType = 2;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolygon: Gdiplus_EmfPlusRecordType = 3;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyline: Gdiplus_EmfPlusRecordType = 4;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyBezierTo: Gdiplus_EmfPlusRecordType = 5;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyLineTo: Gdiplus_EmfPlusRecordType = 6;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyPolyline: Gdiplus_EmfPlusRecordType = 7;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyPolygon: Gdiplus_EmfPlusRecordType = 8;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetWindowExtEx: Gdiplus_EmfPlusRecordType = 9;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetWindowOrgEx: Gdiplus_EmfPlusRecordType = 10;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetViewportExtEx: Gdiplus_EmfPlusRecordType = 11;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetViewportOrgEx: Gdiplus_EmfPlusRecordType = 12;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetBrushOrgEx: Gdiplus_EmfPlusRecordType = 13;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeEOF: Gdiplus_EmfPlusRecordType = 14;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetPixelV: Gdiplus_EmfPlusRecordType = 15;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetMapperFlags: Gdiplus_EmfPlusRecordType = 16;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetMapMode: Gdiplus_EmfPlusRecordType = 17;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetBkMode: Gdiplus_EmfPlusRecordType = 18;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetPolyFillMode: Gdiplus_EmfPlusRecordType = 19;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetROP2: Gdiplus_EmfPlusRecordType = 20;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetStretchBltMode: Gdiplus_EmfPlusRecordType = 21;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetTextAlign: Gdiplus_EmfPlusRecordType = 22;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetColorAdjustment: Gdiplus_EmfPlusRecordType = 23;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetTextColor: Gdiplus_EmfPlusRecordType = 24;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetBkColor: Gdiplus_EmfPlusRecordType = 25;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeOffsetClipRgn: Gdiplus_EmfPlusRecordType = 26;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeMoveToEx: Gdiplus_EmfPlusRecordType = 27;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetMetaRgn: Gdiplus_EmfPlusRecordType = 28;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExcludeClipRect: Gdiplus_EmfPlusRecordType = 29;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeIntersectClipRect: Gdiplus_EmfPlusRecordType = 30;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeScaleViewportExtEx: Gdiplus_EmfPlusRecordType = 31;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeScaleWindowExtEx: Gdiplus_EmfPlusRecordType = 32;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSaveDC: Gdiplus_EmfPlusRecordType = 33;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeRestoreDC: Gdiplus_EmfPlusRecordType = 34;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetWorldTransform: Gdiplus_EmfPlusRecordType = 35;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeModifyWorldTransform: Gdiplus_EmfPlusRecordType =
    36;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSelectObject: Gdiplus_EmfPlusRecordType = 37;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreatePen: Gdiplus_EmfPlusRecordType = 38;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreateBrushIndirect: Gdiplus_EmfPlusRecordType =
    39;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeDeleteObject: Gdiplus_EmfPlusRecordType = 40;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeAngleArc: Gdiplus_EmfPlusRecordType = 41;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeEllipse: Gdiplus_EmfPlusRecordType = 42;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeRectangle: Gdiplus_EmfPlusRecordType = 43;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeRoundRect: Gdiplus_EmfPlusRecordType = 44;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeArc: Gdiplus_EmfPlusRecordType = 45;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeChord: Gdiplus_EmfPlusRecordType = 46;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePie: Gdiplus_EmfPlusRecordType = 47;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSelectPalette: Gdiplus_EmfPlusRecordType = 48;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreatePalette: Gdiplus_EmfPlusRecordType = 49;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetPaletteEntries: Gdiplus_EmfPlusRecordType = 50;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeResizePalette: Gdiplus_EmfPlusRecordType = 51;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeRealizePalette: Gdiplus_EmfPlusRecordType = 52;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtFloodFill: Gdiplus_EmfPlusRecordType = 53;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeLineTo: Gdiplus_EmfPlusRecordType = 54;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeArcTo: Gdiplus_EmfPlusRecordType = 55;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyDraw: Gdiplus_EmfPlusRecordType = 56;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetArcDirection: Gdiplus_EmfPlusRecordType = 57;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetMiterLimit: Gdiplus_EmfPlusRecordType = 58;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeBeginPath: Gdiplus_EmfPlusRecordType = 59;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeEndPath: Gdiplus_EmfPlusRecordType = 60;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCloseFigure: Gdiplus_EmfPlusRecordType = 61;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeFillPath: Gdiplus_EmfPlusRecordType = 62;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeStrokeAndFillPath: Gdiplus_EmfPlusRecordType = 63;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeStrokePath: Gdiplus_EmfPlusRecordType = 64;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeFlattenPath: Gdiplus_EmfPlusRecordType = 65;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeWidenPath: Gdiplus_EmfPlusRecordType = 66;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSelectClipPath: Gdiplus_EmfPlusRecordType = 67;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeAbortPath: Gdiplus_EmfPlusRecordType = 68;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeReserved_069: Gdiplus_EmfPlusRecordType = 69;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeGdiComment: Gdiplus_EmfPlusRecordType = 70;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeFillRgn: Gdiplus_EmfPlusRecordType = 71;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeFrameRgn: Gdiplus_EmfPlusRecordType = 72;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeInvertRgn: Gdiplus_EmfPlusRecordType = 73;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePaintRgn: Gdiplus_EmfPlusRecordType = 74;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtSelectClipRgn: Gdiplus_EmfPlusRecordType = 75;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeBitBlt: Gdiplus_EmfPlusRecordType = 76;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeStretchBlt: Gdiplus_EmfPlusRecordType = 77;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeMaskBlt: Gdiplus_EmfPlusRecordType = 78;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePlgBlt: Gdiplus_EmfPlusRecordType = 79;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetDIBitsToDevice: Gdiplus_EmfPlusRecordType = 80;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeStretchDIBits: Gdiplus_EmfPlusRecordType = 81;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtCreateFontIndirect: Gdiplus_EmfPlusRecordType =
    82;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtTextOutA: Gdiplus_EmfPlusRecordType = 83;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtTextOutW: Gdiplus_EmfPlusRecordType = 84;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyBezier16: Gdiplus_EmfPlusRecordType = 85;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolygon16: Gdiplus_EmfPlusRecordType = 86;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyline16: Gdiplus_EmfPlusRecordType = 87;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyBezierTo16: Gdiplus_EmfPlusRecordType = 88;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolylineTo16: Gdiplus_EmfPlusRecordType = 89;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyPolyline16: Gdiplus_EmfPlusRecordType = 90;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyPolygon16: Gdiplus_EmfPlusRecordType = 91;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyDraw16: Gdiplus_EmfPlusRecordType = 92;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreateMonoBrush: Gdiplus_EmfPlusRecordType = 93;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreateDIBPatternBrushPt:
    Gdiplus_EmfPlusRecordType = 94;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtCreatePen: Gdiplus_EmfPlusRecordType = 95;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyTextOutA: Gdiplus_EmfPlusRecordType = 96;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePolyTextOutW: Gdiplus_EmfPlusRecordType = 97;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetICMMode: Gdiplus_EmfPlusRecordType = 98;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreateColorSpace: Gdiplus_EmfPlusRecordType = 99;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetColorSpace: Gdiplus_EmfPlusRecordType = 100;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeDeleteColorSpace: Gdiplus_EmfPlusRecordType = 101;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeGLSRecord: Gdiplus_EmfPlusRecordType = 102;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeGLSBoundedRecord: Gdiplus_EmfPlusRecordType = 103;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypePixelFormat: Gdiplus_EmfPlusRecordType = 104;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeDrawEscape: Gdiplus_EmfPlusRecordType = 105;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeExtEscape: Gdiplus_EmfPlusRecordType = 106;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeStartDoc: Gdiplus_EmfPlusRecordType = 107;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSmallTextOut: Gdiplus_EmfPlusRecordType = 108;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeForceUFIMapping: Gdiplus_EmfPlusRecordType = 109;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeNamedEscape: Gdiplus_EmfPlusRecordType = 110;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeColorCorrectPalette: Gdiplus_EmfPlusRecordType =
    111;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetICMProfileA: Gdiplus_EmfPlusRecordType = 112;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetICMProfileW: Gdiplus_EmfPlusRecordType = 113;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeAlphaBlend: Gdiplus_EmfPlusRecordType = 114;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetLayout: Gdiplus_EmfPlusRecordType = 115;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeTransparentBlt: Gdiplus_EmfPlusRecordType = 116;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeReserved_117: Gdiplus_EmfPlusRecordType = 117;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeGradientFill: Gdiplus_EmfPlusRecordType = 118;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetLinkedUFIs: Gdiplus_EmfPlusRecordType = 119;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeSetTextJustification: Gdiplus_EmfPlusRecordType =
    120;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeColorMatchToTargetW: Gdiplus_EmfPlusRecordType =
    121;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeCreateColorSpaceW: Gdiplus_EmfPlusRecordType = 122;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeMax: Gdiplus_EmfPlusRecordType = 122;
pub const Gdiplus_EmfPlusRecordType_EmfRecordTypeMin: Gdiplus_EmfPlusRecordType = 1;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeInvalid: Gdiplus_EmfPlusRecordType = 16384;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeHeader: Gdiplus_EmfPlusRecordType = 16385;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeEndOfFile: Gdiplus_EmfPlusRecordType = 16386;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeComment: Gdiplus_EmfPlusRecordType = 16387;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeGetDC: Gdiplus_EmfPlusRecordType = 16388;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMultiFormatStart: Gdiplus_EmfPlusRecordType =
    16389;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMultiFormatSection: Gdiplus_EmfPlusRecordType =
    16390;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMultiFormatEnd: Gdiplus_EmfPlusRecordType =
    16391;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeObject: Gdiplus_EmfPlusRecordType = 16392;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeClear: Gdiplus_EmfPlusRecordType = 16393;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillRects: Gdiplus_EmfPlusRecordType = 16394;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawRects: Gdiplus_EmfPlusRecordType = 16395;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillPolygon: Gdiplus_EmfPlusRecordType = 16396;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawLines: Gdiplus_EmfPlusRecordType = 16397;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillEllipse: Gdiplus_EmfPlusRecordType = 16398;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawEllipse: Gdiplus_EmfPlusRecordType = 16399;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillPie: Gdiplus_EmfPlusRecordType = 16400;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawPie: Gdiplus_EmfPlusRecordType = 16401;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawArc: Gdiplus_EmfPlusRecordType = 16402;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillRegion: Gdiplus_EmfPlusRecordType = 16403;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillPath: Gdiplus_EmfPlusRecordType = 16404;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawPath: Gdiplus_EmfPlusRecordType = 16405;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeFillClosedCurve: Gdiplus_EmfPlusRecordType =
    16406;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawClosedCurve: Gdiplus_EmfPlusRecordType =
    16407;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawCurve: Gdiplus_EmfPlusRecordType = 16408;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawBeziers: Gdiplus_EmfPlusRecordType = 16409;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawImage: Gdiplus_EmfPlusRecordType = 16410;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawImagePoints: Gdiplus_EmfPlusRecordType =
    16411;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawString: Gdiplus_EmfPlusRecordType = 16412;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetRenderingOrigin: Gdiplus_EmfPlusRecordType =
    16413;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetAntiAliasMode: Gdiplus_EmfPlusRecordType =
    16414;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetTextRenderingHint:
    Gdiplus_EmfPlusRecordType = 16415;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetTextContrast: Gdiplus_EmfPlusRecordType =
    16416;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetInterpolationMode:
    Gdiplus_EmfPlusRecordType = 16417;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetPixelOffsetMode: Gdiplus_EmfPlusRecordType =
    16418;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetCompositingMode: Gdiplus_EmfPlusRecordType =
    16419;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetCompositingQuality:
    Gdiplus_EmfPlusRecordType = 16420;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSave: Gdiplus_EmfPlusRecordType = 16421;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeRestore: Gdiplus_EmfPlusRecordType = 16422;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeBeginContainer: Gdiplus_EmfPlusRecordType =
    16423;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeBeginContainerNoParams:
    Gdiplus_EmfPlusRecordType = 16424;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeEndContainer: Gdiplus_EmfPlusRecordType =
    16425;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetWorldTransform: Gdiplus_EmfPlusRecordType =
    16426;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeResetWorldTransform:
    Gdiplus_EmfPlusRecordType = 16427;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMultiplyWorldTransform:
    Gdiplus_EmfPlusRecordType = 16428;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeTranslateWorldTransform:
    Gdiplus_EmfPlusRecordType = 16429;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeScaleWorldTransform:
    Gdiplus_EmfPlusRecordType = 16430;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeRotateWorldTransform:
    Gdiplus_EmfPlusRecordType = 16431;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetPageTransform: Gdiplus_EmfPlusRecordType =
    16432;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeResetClip: Gdiplus_EmfPlusRecordType = 16433;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetClipRect: Gdiplus_EmfPlusRecordType = 16434;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetClipPath: Gdiplus_EmfPlusRecordType = 16435;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeSetClipRegion: Gdiplus_EmfPlusRecordType =
    16436;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeOffsetClip: Gdiplus_EmfPlusRecordType = 16437;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeDrawDriverString: Gdiplus_EmfPlusRecordType =
    16438;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTotal: Gdiplus_EmfPlusRecordType = 16439;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMax: Gdiplus_EmfPlusRecordType = 16438;
pub const Gdiplus_EmfPlusRecordType_EmfPlusRecordTypeMin: Gdiplus_EmfPlusRecordType = 16385;
pub type Gdiplus_EmfPlusRecordType = c_int;
pub const Gdiplus_StringTrimming_StringTrimmingNone: Gdiplus_StringTrimming = 0;
pub const Gdiplus_StringTrimming_StringTrimmingCharacter: Gdiplus_StringTrimming = 1;
pub const Gdiplus_StringTrimming_StringTrimmingWord: Gdiplus_StringTrimming = 2;
pub const Gdiplus_StringTrimming_StringTrimmingEllipsisCharacter: Gdiplus_StringTrimming = 3;
pub const Gdiplus_StringTrimming_StringTrimmingEllipsisWord: Gdiplus_StringTrimming = 4;
pub const Gdiplus_StringTrimming_StringTrimmingEllipsisPath: Gdiplus_StringTrimming = 5;
pub type Gdiplus_StringTrimming = c_int;
pub const Gdiplus_StringDigitSubstitute_StringDigitSubstituteUser: Gdiplus_StringDigitSubstitute =
    0;
pub const Gdiplus_StringDigitSubstitute_StringDigitSubstituteNone: Gdiplus_StringDigitSubstitute =
    1;
pub const Gdiplus_StringDigitSubstitute_StringDigitSubstituteNational:
    Gdiplus_StringDigitSubstitute = 2;
pub const Gdiplus_StringDigitSubstitute_StringDigitSubstituteTraditional:
    Gdiplus_StringDigitSubstitute = 3;
pub type Gdiplus_StringDigitSubstitute = c_int;
pub const Gdiplus_StringAlignment_StringAlignmentNear: Gdiplus_StringAlignment = 0;
pub const Gdiplus_StringAlignment_StringAlignmentCenter: Gdiplus_StringAlignment = 1;
pub const Gdiplus_StringAlignment_StringAlignmentFar: Gdiplus_StringAlignment = 2;
pub type Gdiplus_StringAlignment = c_int;
pub const Gdiplus_FlushIntention_FlushIntentionFlush: Gdiplus_FlushIntention = 0;
pub const Gdiplus_FlushIntention_FlushIntentionSync: Gdiplus_FlushIntention = 1;
pub type Gdiplus_FlushIntention = c_int;
pub const Gdiplus_GpTestControlEnum_TestControlForceBilinear: Gdiplus_GpTestControlEnum = 0;
pub const Gdiplus_GpTestControlEnum_TestControlNoICM: Gdiplus_GpTestControlEnum = 1;
pub const Gdiplus_GpTestControlEnum_TestControlGetBuildNumber: Gdiplus_GpTestControlEnum = 2;
pub type Gdiplus_GpTestControlEnum = c_int;
pub type Gdiplus_ImageAbort = Option<unsafe extern "C" fn(arg1: *mut c_void) -> BOOL>;
pub type Gdiplus_DrawImageAbort = Gdiplus_ImageAbort;
pub type Gdiplus_GetThumbnailImageAbort = Gdiplus_ImageAbort;
pub type Gdiplus_EnumerateMetafileProc = Option<
    unsafe extern "C" fn(
        arg1: Gdiplus_EmfPlusRecordType,
        arg2: UINT,
        arg3: UINT,
        arg4: *const BYTE,
        arg5: *mut c_void,
    ) -> BOOL,
>;
pub type Gdiplus_REAL = f32;
pub const Gdiplus_Status_Ok: Gdiplus_Status = 0;
pub const Gdiplus_Status_GenericError: Gdiplus_Status = 1;
pub const Gdiplus_Status_InvalidParameter: Gdiplus_Status = 2;
pub const Gdiplus_Status_OutOfMemory: Gdiplus_Status = 3;
pub const Gdiplus_Status_ObjectBusy: Gdiplus_Status = 4;
pub const Gdiplus_Status_InsufficientBuffer: Gdiplus_Status = 5;
pub const Gdiplus_Status_NotImplemented: Gdiplus_Status = 6;
pub const Gdiplus_Status_Win32Error: Gdiplus_Status = 7;
pub const Gdiplus_Status_WrongState: Gdiplus_Status = 8;
pub const Gdiplus_Status_Aborted: Gdiplus_Status = 9;
pub const Gdiplus_Status_FileNotFound: Gdiplus_Status = 10;
pub const Gdiplus_Status_ValueOverflow: Gdiplus_Status = 11;
pub const Gdiplus_Status_AccessDenied: Gdiplus_Status = 12;
pub const Gdiplus_Status_UnknownImageFormat: Gdiplus_Status = 13;
pub const Gdiplus_Status_FontFamilyNotFound: Gdiplus_Status = 14;
pub const Gdiplus_Status_FontStyleNotFound: Gdiplus_Status = 15;
pub const Gdiplus_Status_NotTrueTypeFont: Gdiplus_Status = 16;
pub const Gdiplus_Status_UnsupportedGdiplusVersion: Gdiplus_Status = 17;
pub const Gdiplus_Status_GdiplusNotInitialized: Gdiplus_Status = 18;
pub const Gdiplus_Status_PropertyNotFound: Gdiplus_Status = 19;
pub const Gdiplus_Status_PropertyNotSupported: Gdiplus_Status = 20;
pub type Gdiplus_Status = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_SizeF {
    pub Width: Gdiplus_REAL,
    pub Height: Gdiplus_REAL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_PointF {
    pub X: Gdiplus_REAL,
    pub Y: Gdiplus_REAL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_Point {
    pub X: INT,
    pub Y: INT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_RectF {
    pub X: Gdiplus_REAL,
    pub Y: Gdiplus_REAL,
    pub Width: Gdiplus_REAL,
    pub Height: Gdiplus_REAL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_Rect {
    pub X: INT,
    pub Y: INT,
    pub Width: INT,
    pub Height: INT,
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_PathData {
    pub Count: INT,
    pub Points: *mut Gdiplus_PointF,
    pub Types: *mut BYTE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_CharacterRange {
    pub First: INT,
    pub Length: INT,
}
pub const Gdiplus_DebugEventLevel_DebugEventLevelFatal: Gdiplus_DebugEventLevel = 0;
pub const Gdiplus_DebugEventLevel_DebugEventLevelWarning: Gdiplus_DebugEventLevel = 1;
pub type Gdiplus_DebugEventLevel = c_int;
pub type Gdiplus_DebugEventProc =
    Option<unsafe extern "C" fn(level: Gdiplus_DebugEventLevel, message: *mut CHAR)>;
pub type Gdiplus_NotificationHookProc =
    Option<unsafe extern "C" fn(token: *mut ULONG_PTR) -> Gdiplus_Status>;
pub type Gdiplus_NotificationUnhookProc = Option<unsafe extern "C" fn(token: ULONG_PTR)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GdiplusStartupInput {
    pub GdiplusVersion: UINT32,
    pub DebugEventCallback: Gdiplus_DebugEventProc,
    pub SuppressBackgroundThread: BOOL,
    pub SuppressExternalCodecs: BOOL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GdiplusStartupOutput {
    pub NotificationHook: Gdiplus_NotificationHookProc,
    pub NotificationUnhook: Gdiplus_NotificationUnhookProc,
}
extern "C" {
    #[link_name = "\u{1}GdiplusStartup"]
    pub fn Gdiplus_GdiplusStartup(
        token: *mut ULONG_PTR,
        input: *const Gdiplus_GdiplusStartupInput,
        output: *mut Gdiplus_GdiplusStartupOutput,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}GdiplusShutdown"]
    pub fn Gdiplus_GdiplusShutdown(token: ULONG_PTR);
}
pub type Gdiplus_ARGB = DWORD;
pub type Gdiplus_PixelFormat = INT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_ColorPalette {
    pub Flags: UINT,
    pub Count: UINT,
    pub Entries: [Gdiplus_ARGB; 1usize],
}
pub const Gdiplus_ColorChannelFlags_ColorChannelFlagsC: Gdiplus_ColorChannelFlags = 0;
pub const Gdiplus_ColorChannelFlags_ColorChannelFlagsM: Gdiplus_ColorChannelFlags = 1;
pub const Gdiplus_ColorChannelFlags_ColorChannelFlagsY: Gdiplus_ColorChannelFlags = 2;
pub const Gdiplus_ColorChannelFlags_ColorChannelFlagsK: Gdiplus_ColorChannelFlags = 3;
pub const Gdiplus_ColorChannelFlags_ColorChannelFlagsLast: Gdiplus_ColorChannelFlags = 4;
pub type Gdiplus_ColorChannelFlags = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_Color {
    pub Argb: Gdiplus_ARGB,
}
pub const Gdiplus_Color_AliceBlue: c_int = -984833;
pub const Gdiplus_Color_AntiqueWhite: c_int = -332841;
pub const Gdiplus_Color_Aqua: c_int = -16711681;
pub const Gdiplus_Color_Aquamarine: c_int = -8388652;
pub const Gdiplus_Color_Azure: c_int = -983041;
pub const Gdiplus_Color_Beige: c_int = -657956;
pub const Gdiplus_Color_Bisque: c_int = -6972;
pub const Gdiplus_Color_Black: c_int = -16777216;
pub const Gdiplus_Color_BlanchedAlmond: c_int = -5171;
pub const Gdiplus_Color_Blue: c_int = -16776961;
pub const Gdiplus_Color_BlueViolet: c_int = -7722014;
pub const Gdiplus_Color_Brown: c_int = -5952982;
pub const Gdiplus_Color_BurlyWood: c_int = -2180985;
pub const Gdiplus_Color_CadetBlue: c_int = -10510688;
pub const Gdiplus_Color_Chartreuse: c_int = -8388864;
pub const Gdiplus_Color_Chocolate: c_int = -2987746;
pub const Gdiplus_Color_Coral: c_int = -32944;
pub const Gdiplus_Color_CornflowerBlue: c_int = -10185235;
pub const Gdiplus_Color_Cornsilk: c_int = -1828;
pub const Gdiplus_Color_Crimson: c_int = -2354116;
pub const Gdiplus_Color_Cyan: c_int = -16711681;
pub const Gdiplus_Color_DarkBlue: c_int = -16777077;
pub const Gdiplus_Color_DarkCyan: c_int = -16741493;
pub const Gdiplus_Color_DarkGoldenrod: c_int = -4684277;
pub const Gdiplus_Color_DarkGray: c_int = -5658199;
pub const Gdiplus_Color_DarkGreen: c_int = -16751616;
pub const Gdiplus_Color_DarkKhaki: c_int = -4343957;
pub const Gdiplus_Color_DarkMagenta: c_int = -7667573;
pub const Gdiplus_Color_DarkOliveGreen: c_int = -11179217;
pub const Gdiplus_Color_DarkOrange: c_int = -29696;
pub const Gdiplus_Color_DarkOrchid: c_int = -6737204;
pub const Gdiplus_Color_DarkRed: c_int = -7667712;
pub const Gdiplus_Color_DarkSalmon: c_int = -1468806;
pub const Gdiplus_Color_DarkSeaGreen: c_int = -7357301;
pub const Gdiplus_Color_DarkSlateBlue: c_int = -12042869;
pub const Gdiplus_Color_DarkSlateGray: c_int = -13676721;
pub const Gdiplus_Color_DarkTurquoise: c_int = -16724271;
pub const Gdiplus_Color_DarkViolet: c_int = -7077677;
pub const Gdiplus_Color_DeepPink: c_int = -60269;
pub const Gdiplus_Color_DeepSkyBlue: c_int = -16728065;
pub const Gdiplus_Color_DimGray: c_int = -9868951;
pub const Gdiplus_Color_DodgerBlue: c_int = -14774017;
pub const Gdiplus_Color_Firebrick: c_int = -5103070;
pub const Gdiplus_Color_FloralWhite: c_int = -1296;
pub const Gdiplus_Color_ForestGreen: c_int = -14513374;
pub const Gdiplus_Color_Fuchsia: c_int = -65281;
pub const Gdiplus_Color_Gainsboro: c_int = -2302756;
pub const Gdiplus_Color_GhostWhite: c_int = -460545;
pub const Gdiplus_Color_Gold: c_int = -10496;
pub const Gdiplus_Color_Goldenrod: c_int = -2448096;
pub const Gdiplus_Color_Gray: c_int = -8355712;
pub const Gdiplus_Color_Green: c_int = -16744448;
pub const Gdiplus_Color_GreenYellow: c_int = -5374161;
pub const Gdiplus_Color_Honeydew: c_int = -983056;
pub const Gdiplus_Color_HotPink: c_int = -38476;
pub const Gdiplus_Color_IndianRed: c_int = -3318692;
pub const Gdiplus_Color_Indigo: c_int = -11861886;
pub const Gdiplus_Color_Ivory: c_int = -16;
pub const Gdiplus_Color_Khaki: c_int = -989556;
pub const Gdiplus_Color_Lavender: c_int = -1644806;
pub const Gdiplus_Color_LavenderBlush: c_int = -3851;
pub const Gdiplus_Color_LawnGreen: c_int = -8586240;
pub const Gdiplus_Color_LemonChiffon: c_int = -1331;
pub const Gdiplus_Color_LightBlue: c_int = -5383962;
pub const Gdiplus_Color_LightCoral: c_int = -1015680;
pub const Gdiplus_Color_LightCyan: c_int = -2031617;
pub const Gdiplus_Color_LightGoldenrodYellow: c_int = -329006;
pub const Gdiplus_Color_LightGray: c_int = -2894893;
pub const Gdiplus_Color_LightGreen: c_int = -7278960;
pub const Gdiplus_Color_LightPink: c_int = -18751;
pub const Gdiplus_Color_LightSalmon: c_int = -24454;
pub const Gdiplus_Color_LightSeaGreen: c_int = -14634326;
pub const Gdiplus_Color_LightSkyBlue: c_int = -7876870;
pub const Gdiplus_Color_LightSlateGray: c_int = -8943463;
pub const Gdiplus_Color_LightSteelBlue: c_int = -5192482;
pub const Gdiplus_Color_LightYellow: c_int = -32;
pub const Gdiplus_Color_Lime: c_int = -16711936;
pub const Gdiplus_Color_LimeGreen: c_int = -13447886;
pub const Gdiplus_Color_Linen: c_int = -331546;
pub const Gdiplus_Color_Magenta: c_int = -65281;
pub const Gdiplus_Color_Maroon: c_int = -8388608;
pub const Gdiplus_Color_MediumAquamarine: c_int = -10039894;
pub const Gdiplus_Color_MediumBlue: c_int = -16777011;
pub const Gdiplus_Color_MediumOrchid: c_int = -4565549;
pub const Gdiplus_Color_MediumPurple: c_int = -7114533;
pub const Gdiplus_Color_MediumSeaGreen: c_int = -12799119;
pub const Gdiplus_Color_MediumSlateBlue: c_int = -8689426;
pub const Gdiplus_Color_MediumSpringGreen: c_int = -16713062;
pub const Gdiplus_Color_MediumTurquoise: c_int = -12004916;
pub const Gdiplus_Color_MediumVioletRed: c_int = -3730043;
pub const Gdiplus_Color_MidnightBlue: c_int = -15132304;
pub const Gdiplus_Color_MintCream: c_int = -655366;
pub const Gdiplus_Color_MistyRose: c_int = -6943;
pub const Gdiplus_Color_Moccasin: c_int = -6987;
pub const Gdiplus_Color_NavajoWhite: c_int = -8531;
pub const Gdiplus_Color_Navy: c_int = -16777088;
pub const Gdiplus_Color_OldLace: c_int = -133658;
pub const Gdiplus_Color_Olive: c_int = -8355840;
pub const Gdiplus_Color_OliveDrab: c_int = -9728477;
pub const Gdiplus_Color_Orange: c_int = -23296;
pub const Gdiplus_Color_OrangeRed: c_int = -47872;
pub const Gdiplus_Color_Orchid: c_int = -2461482;
pub const Gdiplus_Color_PaleGoldenrod: c_int = -1120086;
pub const Gdiplus_Color_PaleGreen: c_int = -6751336;
pub const Gdiplus_Color_PaleTurquoise: c_int = -5247250;
pub const Gdiplus_Color_PaleVioletRed: c_int = -2396013;
pub const Gdiplus_Color_PapayaWhip: c_int = -4139;
pub const Gdiplus_Color_PeachPuff: c_int = -9543;
pub const Gdiplus_Color_Peru: c_int = -3308225;
pub const Gdiplus_Color_Pink: c_int = -16181;
pub const Gdiplus_Color_Plum: c_int = -2252579;
pub const Gdiplus_Color_PowderBlue: c_int = -5185306;
pub const Gdiplus_Color_Purple: c_int = -8388480;
pub const Gdiplus_Color_Red: c_int = -65536;
pub const Gdiplus_Color_RosyBrown: c_int = -4419697;
pub const Gdiplus_Color_RoyalBlue: c_int = -12490271;
pub const Gdiplus_Color_SaddleBrown: c_int = -7650029;
pub const Gdiplus_Color_Salmon: c_int = -360334;
pub const Gdiplus_Color_SandyBrown: c_int = -744352;
pub const Gdiplus_Color_SeaGreen: c_int = -13726889;
pub const Gdiplus_Color_SeaShell: c_int = -2578;
pub const Gdiplus_Color_Sienna: c_int = -6270419;
pub const Gdiplus_Color_Silver: c_int = -4144960;
pub const Gdiplus_Color_SkyBlue: c_int = -7876885;
pub const Gdiplus_Color_SlateBlue: c_int = -9807155;
pub const Gdiplus_Color_SlateGray: c_int = -9404272;
pub const Gdiplus_Color_Snow: c_int = -1286;
pub const Gdiplus_Color_SpringGreen: c_int = -16711809;
pub const Gdiplus_Color_SteelBlue: c_int = -12156236;
pub const Gdiplus_Color_Tan: c_int = -2968436;
pub const Gdiplus_Color_Teal: c_int = -16744320;
pub const Gdiplus_Color_Thistle: c_int = -2572328;
pub const Gdiplus_Color_Tomato: c_int = -40121;
pub const Gdiplus_Color_Transparent: c_int = 16777215;
pub const Gdiplus_Color_Turquoise: c_int = -12525360;
pub const Gdiplus_Color_Violet: c_int = -1146130;
pub const Gdiplus_Color_Wheat: c_int = -663885;
pub const Gdiplus_Color_White: c_int = -1;
pub const Gdiplus_Color_WhiteSmoke: c_int = -657931;
pub const Gdiplus_Color_Yellow: c_int = -256;
pub const Gdiplus_Color_YellowGreen: c_int = -6632142;
pub type Gdiplus_Color__bindgen_ty_1 = c_int;
pub const Gdiplus_Color_AlphaShift: c_int = 24;
pub const Gdiplus_Color_RedShift: c_int = 16;
pub const Gdiplus_Color_GreenShift: c_int = 8;
pub const Gdiplus_Color_BlueShift: c_int = 0;
pub type Gdiplus_Color__bindgen_ty_2 = c_int;
pub const Gdiplus_Color_AlphaMask: c_int = -16777216;
pub const Gdiplus_Color_RedMask: c_int = 16711680;
pub const Gdiplus_Color_GreenMask: c_int = 65280;
pub const Gdiplus_Color_BlueMask: c_int = 255;
pub type Gdiplus_Color__bindgen_ty_3 = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gdiplus_ENHMETAHEADER3 {
    pub iType: DWORD,
    pub nSize: DWORD,
    pub rclBounds: RECTL,
    pub rclFrame: RECTL,
    pub dSignature: DWORD,
    pub nVersion: DWORD,
    pub nBytes: DWORD,
    pub nRecords: DWORD,
    pub nHandles: WORD,
    pub sReserved: WORD,
    pub nDescription: DWORD,
    pub offDescription: DWORD,
    pub nPalEntries: DWORD,
    pub szlDevice: SIZEL,
    pub szlMillimeters: SIZEL,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_PWMFRect16 {
    pub Left: INT16,
    pub Top: INT16,
    pub Right: INT16,
    pub Bottom: INT16,
}
#[repr(C, packed(2))]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_WmfPlaceableFileHeader {
    pub Key: UINT32,
    pub Hmf: INT16,
    pub BoundingBox: Gdiplus_PWMFRect16,
    pub Inch: INT16,
    pub Reserved: UINT32,
    pub Checksum: INT16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gdiplus_MetafileHeader {
    pub Type: Gdiplus_MetafileType,
    pub Size: UINT,
    pub Version: UINT,
    pub EmfPlusFlags: UINT,
    pub DpiX: Gdiplus_REAL,
    pub DpiY: Gdiplus_REAL,
    pub X: INT,
    pub Y: INT,
    pub Width: INT,
    pub Height: INT,
    pub __bindgen_anon_1: Gdiplus_MetafileHeader__bindgen_ty_1,
    pub EmfPlusHeaderSize: INT,
    pub LogicalDpiX: INT,
    pub LogicalDpiY: INT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Gdiplus_MetafileHeader__bindgen_ty_1 {
    pub WmfHeader: METAHEADER,
    pub EmfHeader: Gdiplus_ENHMETAHEADER3,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gdiplus_ImageCodecInfo {
    pub Clsid: CLSID,
    pub FormatID: GUID,
    pub CodecName: *const WCHAR,
    pub DllName: *const WCHAR,
    pub FormatDescription: *const WCHAR,
    pub FilenameExtension: *const WCHAR,
    pub MimeType: *const WCHAR,
    pub Flags: DWORD,
    pub Version: DWORD,
    pub SigCount: DWORD,
    pub SigSize: DWORD,
    pub SigPattern: *const BYTE,
    pub SigMask: *const BYTE,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_BitmapData {
    pub Width: UINT,
    pub Height: UINT,
    pub Stride: INT,
    pub PixelFormat: Gdiplus_PixelFormat,
    pub Scan0: *mut c_void,
    pub Reserved: UINT_PTR,
}
pub const Gdiplus_RotateFlipType_RotateNoneFlipNone: Gdiplus_RotateFlipType = 0;
pub const Gdiplus_RotateFlipType_Rotate90FlipNone: Gdiplus_RotateFlipType = 1;
pub const Gdiplus_RotateFlipType_Rotate180FlipNone: Gdiplus_RotateFlipType = 2;
pub const Gdiplus_RotateFlipType_Rotate270FlipNone: Gdiplus_RotateFlipType = 3;
pub const Gdiplus_RotateFlipType_RotateNoneFlipX: Gdiplus_RotateFlipType = 4;
pub const Gdiplus_RotateFlipType_Rotate90FlipX: Gdiplus_RotateFlipType = 5;
pub const Gdiplus_RotateFlipType_Rotate180FlipX: Gdiplus_RotateFlipType = 6;
pub const Gdiplus_RotateFlipType_Rotate270FlipX: Gdiplus_RotateFlipType = 7;
pub const Gdiplus_RotateFlipType_RotateNoneFlipY: Gdiplus_RotateFlipType = 6;
pub const Gdiplus_RotateFlipType_Rotate90FlipY: Gdiplus_RotateFlipType = 7;
pub const Gdiplus_RotateFlipType_Rotate180FlipY: Gdiplus_RotateFlipType = 4;
pub const Gdiplus_RotateFlipType_Rotate270FlipY: Gdiplus_RotateFlipType = 5;
pub const Gdiplus_RotateFlipType_RotateNoneFlipXY: Gdiplus_RotateFlipType = 2;
pub const Gdiplus_RotateFlipType_Rotate90FlipXY: Gdiplus_RotateFlipType = 3;
pub const Gdiplus_RotateFlipType_Rotate180FlipXY: Gdiplus_RotateFlipType = 0;
pub const Gdiplus_RotateFlipType_Rotate270FlipXY: Gdiplus_RotateFlipType = 1;
pub type Gdiplus_RotateFlipType = c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gdiplus_EncoderParameter {
    pub Guid: GUID,
    pub NumberOfValues: ULONG,
    pub Type: ULONG,
    pub Value: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Gdiplus_EncoderParameters {
    pub Count: UINT,
    pub Parameter: [Gdiplus_EncoderParameter; 1usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_PropertyItem {
    pub id: PROPID,
    pub length: ULONG,
    pub type_: WORD,
    pub value: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_ColorMatrix {
    pub m: [[Gdiplus_REAL; 5usize]; 5usize],
}
pub const Gdiplus_ColorMatrixFlags_ColorMatrixFlagsDefault: Gdiplus_ColorMatrixFlags = 0;
pub const Gdiplus_ColorMatrixFlags_ColorMatrixFlagsSkipGrays: Gdiplus_ColorMatrixFlags = 1;
pub const Gdiplus_ColorMatrixFlags_ColorMatrixFlagsAltGray: Gdiplus_ColorMatrixFlags = 2;
pub type Gdiplus_ColorMatrixFlags = c_int;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeDefault: Gdiplus_ColorAdjustType = 0;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeBitmap: Gdiplus_ColorAdjustType = 1;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeBrush: Gdiplus_ColorAdjustType = 2;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypePen: Gdiplus_ColorAdjustType = 3;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeText: Gdiplus_ColorAdjustType = 4;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeCount: Gdiplus_ColorAdjustType = 5;
pub const Gdiplus_ColorAdjustType_ColorAdjustTypeAny: Gdiplus_ColorAdjustType = 6;
pub type Gdiplus_ColorAdjustType = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_ColorMap {
    pub oldColor: Gdiplus_Color,
    pub newColor: Gdiplus_Color,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpGraphics {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpBrush {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpTexture {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpSolidFill {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpLineGradient {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpPathGradient {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpHatch {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpPen {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpCustomLineCap {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpAdjustableArrowCap {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpImage {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpBitmap {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpMetafile {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpImageAttributes {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpPath {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpRegion {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpPathIterator {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpFontFamily {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpFont {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpStringFormat {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpFontCollection {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gdiplus_GpCachedBitmap {
    _unused: [u8; 0],
}
pub use self::{
    Gdiplus_CoordinateSpace as Gdiplus_GpCoordinateSpace, Gdiplus_FillMode as Gdiplus_GpFillMode,
    Gdiplus_Status as Gdiplus_GpStatus, Gdiplus_Unit as Gdiplus_GpUnit,
    Gdiplus_WrapMode as Gdiplus_GpWrapMode,
};
pub type Gdiplus_GpPointF = Gdiplus_PointF;
pub type Gdiplus_GpPoint = Gdiplus_Point;
pub type Gdiplus_GpRectF = Gdiplus_RectF;
pub type Gdiplus_GpRect = Gdiplus_Rect;
pub use self::{
    Gdiplus_DashCap as Gdiplus_GpDashCap, Gdiplus_DashStyle as Gdiplus_GpDashStyle,
    Gdiplus_HatchStyle as Gdiplus_GpHatchStyle, Gdiplus_LineCap as Gdiplus_GpLineCap,
    Gdiplus_LineJoin as Gdiplus_GpLineJoin, Gdiplus_PenAlignment as Gdiplus_GpPenAlignment,
    Gdiplus_PenType as Gdiplus_GpPenType,
};
pub type Gdiplus_GpMatrix = Gdiplus_Matrix;
pub use self::{
    Gdiplus_BrushType as Gdiplus_GpBrushType, Gdiplus_FlushIntention as Gdiplus_GpFlushIntention,
    Gdiplus_MatrixOrder as Gdiplus_GpMatrixOrder,
};
pub type Gdiplus_GpPathData = Gdiplus_PathData;
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Region {
    pub nativeRegion: *mut Gdiplus_GpRegion,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?FromHRGN@Region@Gdiplus@@SAPEAV12@PEAUHRGN__@@@Z"]
    pub fn Gdiplus_Region_FromHRGN(hRgn: HRGN) -> *mut Gdiplus_Region;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Region@Gdiplus@@QEBAPEAV12@XZ"]
    pub fn Gdiplus_Region_Clone(this: *const Gdiplus_Region) -> *mut Gdiplus_Region;
}
extern "C" {
    #[link_name = "\u{1}?MakeInfinite@Region@Gdiplus@@QEAA?AW4Status@2@XZ"]
    pub fn Gdiplus_Region_MakeInfinite(this: *mut Gdiplus_Region) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?MakeEmpty@Region@Gdiplus@@QEAA?AW4Status@2@XZ"]
    pub fn Gdiplus_Region_MakeEmpty(this: *mut Gdiplus_Region) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetDataSize@Region@Gdiplus@@QEBAIXZ"]
    pub fn Gdiplus_Region_GetDataSize(this: *const Gdiplus_Region) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetData@Region@Gdiplus@@QEBA?AW4Status@2@PEAEIPEAI@Z"]
    pub fn Gdiplus_Region_GetData(
        this: *const Gdiplus_Region,
        buffer: *mut BYTE,
        bufferSize: UINT,
        sizeFilled: *mut UINT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Intersect@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRect@2@@Z"]
    pub fn Gdiplus_Region_Intersect(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_Rect,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Intersect@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRectF@2@@Z"]
    pub fn Gdiplus_Region_Intersect1(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_RectF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Intersect@Region@Gdiplus@@QEAA?AW4Status@2@PEBVGraphicsPath@2@@Z"]
    pub fn Gdiplus_Region_Intersect2(
        this: *mut Gdiplus_Region,
        path: *const Gdiplus_GraphicsPath,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Intersect@Region@Gdiplus@@QEAA?AW4Status@2@PEBV12@@Z"]
    pub fn Gdiplus_Region_Intersect3(
        this: *mut Gdiplus_Region,
        region: *const Gdiplus_Region,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Union@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRect@2@@Z"]
    pub fn Gdiplus_Region_Union(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_Rect,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Union@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRectF@2@@Z"]
    pub fn Gdiplus_Region_Union1(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_RectF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Union@Region@Gdiplus@@QEAA?AW4Status@2@PEBVGraphicsPath@2@@Z"]
    pub fn Gdiplus_Region_Union2(
        this: *mut Gdiplus_Region,
        path: *const Gdiplus_GraphicsPath,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Union@Region@Gdiplus@@QEAA?AW4Status@2@PEBV12@@Z"]
    pub fn Gdiplus_Region_Union3(
        this: *mut Gdiplus_Region,
        region: *const Gdiplus_Region,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Xor@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRect@2@@Z"]
    pub fn Gdiplus_Region_Xor(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_Rect,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Xor@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRectF@2@@Z"]
    pub fn Gdiplus_Region_Xor1(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_RectF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Xor@Region@Gdiplus@@QEAA?AW4Status@2@PEBVGraphicsPath@2@@Z"]
    pub fn Gdiplus_Region_Xor2(
        this: *mut Gdiplus_Region,
        path: *const Gdiplus_GraphicsPath,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Xor@Region@Gdiplus@@QEAA?AW4Status@2@PEBV12@@Z"]
    pub fn Gdiplus_Region_Xor3(
        this: *mut Gdiplus_Region,
        region: *const Gdiplus_Region,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Exclude@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRect@2@@Z"]
    pub fn Gdiplus_Region_Exclude(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_Rect,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Exclude@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRectF@2@@Z"]
    pub fn Gdiplus_Region_Exclude1(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_RectF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Exclude@Region@Gdiplus@@QEAA?AW4Status@2@PEBVGraphicsPath@2@@Z"]
    pub fn Gdiplus_Region_Exclude2(
        this: *mut Gdiplus_Region,
        path: *const Gdiplus_GraphicsPath,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Exclude@Region@Gdiplus@@QEAA?AW4Status@2@PEBV12@@Z"]
    pub fn Gdiplus_Region_Exclude3(
        this: *mut Gdiplus_Region,
        region: *const Gdiplus_Region,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Complement@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRect@2@@Z"]
    pub fn Gdiplus_Region_Complement(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_Rect,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Complement@Region@Gdiplus@@QEAA?AW4Status@2@AEBVRectF@2@@Z"]
    pub fn Gdiplus_Region_Complement1(
        this: *mut Gdiplus_Region,
        rect: *const Gdiplus_RectF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Complement@Region@Gdiplus@@QEAA?AW4Status@2@PEBVGraphicsPath@2@@Z"]
    pub fn Gdiplus_Region_Complement2(
        this: *mut Gdiplus_Region,
        path: *const Gdiplus_GraphicsPath,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Complement@Region@Gdiplus@@QEAA?AW4Status@2@PEBV12@@Z"]
    pub fn Gdiplus_Region_Complement3(
        this: *mut Gdiplus_Region,
        region: *const Gdiplus_Region,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Translate@Region@Gdiplus@@QEAA?AW4Status@2@MM@Z"]
    pub fn Gdiplus_Region_Translate(
        this: *mut Gdiplus_Region,
        dx: Gdiplus_REAL,
        dy: Gdiplus_REAL,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Translate@Region@Gdiplus@@QEAA?AW4Status@2@HH@Z"]
    pub fn Gdiplus_Region_Translate1(this: *mut Gdiplus_Region, dx: INT, dy: INT)
        -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Transform@Region@Gdiplus@@QEAA?AW4Status@2@PEBVMatrix@2@@Z"]
    pub fn Gdiplus_Region_Transform(
        this: *mut Gdiplus_Region,
        matrix: *const Gdiplus_Matrix,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBounds@Region@Gdiplus@@QEBA?AW4Status@2@PEAVRect@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_GetBounds(
        this: *const Gdiplus_Region,
        rect: *mut Gdiplus_Rect,
        g: *const Gdiplus_Graphics,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBounds@Region@Gdiplus@@QEBA?AW4Status@2@PEAVRectF@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_GetBounds1(
        this: *const Gdiplus_Region,
        rect: *mut Gdiplus_RectF,
        g: *const Gdiplus_Graphics,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetHRGN@Region@Gdiplus@@QEBAPEAUHRGN__@@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_GetHRGN(this: *const Gdiplus_Region, g: *const Gdiplus_Graphics) -> HRGN;
}
extern "C" {
    #[link_name = "\u{1}?IsEmpty@Region@Gdiplus@@QEBAHPEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsEmpty(this: *const Gdiplus_Region, g: *const Gdiplus_Graphics) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsInfinite@Region@Gdiplus@@QEBAHPEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsInfinite(
        this: *const Gdiplus_Region,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@Region@Gdiplus@@QEBAHAEBVPoint@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsVisible(
        this: *const Gdiplus_Region,
        point: *const Gdiplus_Point,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@Region@Gdiplus@@QEBAHAEBVPointF@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsVisible1(
        this: *const Gdiplus_Region,
        point: *const Gdiplus_PointF,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@Region@Gdiplus@@QEBAHAEBVRect@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsVisible2(
        this: *const Gdiplus_Region,
        rect: *const Gdiplus_Rect,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@Region@Gdiplus@@QEBAHAEBVRectF@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_IsVisible3(
        this: *const Gdiplus_Region,
        rect: *const Gdiplus_RectF,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?Equals@Region@Gdiplus@@QEBAHPEBV12@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_Region_Equals(
        this: *const Gdiplus_Region,
        region: *const Gdiplus_Region,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?GetRegionScansCount@Region@Gdiplus@@QEBAIPEBVMatrix@2@@Z"]
    pub fn Gdiplus_Region_GetRegionScansCount(
        this: *const Gdiplus_Region,
        matrix: *const Gdiplus_Matrix,
    ) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetRegionScans@Region@Gdiplus@@QEBA?AW4Status@2@PEBVMatrix@2@PEAVRectF@2@PEAH@Z"]
    pub fn Gdiplus_Region_GetRegionScans(
        this: *const Gdiplus_Region,
        matrix: *const Gdiplus_Matrix,
        rects: *mut Gdiplus_RectF,
        count: *mut INT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetRegionScans@Region@Gdiplus@@QEBA?AW4Status@2@PEBVMatrix@2@PEAVRect@2@PEAH@Z"]
    pub fn Gdiplus_Region_GetRegionScans1(
        this: *const Gdiplus_Region,
        matrix: *const Gdiplus_Matrix,
        rects: *mut Gdiplus_Rect,
        count: *mut INT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@Region@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_Region_GetLastStatus(this: *const Gdiplus_Region) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetNativeRegion@Region@Gdiplus@@IEAAXPEAVGpRegion@2@@Z"]
    pub fn Gdiplus_Region_SetNativeRegion(
        this: *mut Gdiplus_Region,
        nativeRegion: *mut Gdiplus_GpRegion,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@XZ"]
    pub fn Gdiplus_Region_Region(this: *mut Gdiplus_Region);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@AEBVRectF@1@@Z"]
    pub fn Gdiplus_Region_Region1(this: *mut Gdiplus_Region, rect: *const Gdiplus_RectF);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@AEBVRect@1@@Z"]
    pub fn Gdiplus_Region_Region2(this: *mut Gdiplus_Region, rect: *const Gdiplus_Rect);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@PEBVGraphicsPath@1@@Z"]
    pub fn Gdiplus_Region_Region3(this: *mut Gdiplus_Region, path: *const Gdiplus_GraphicsPath);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@PEBEH@Z"]
    pub fn Gdiplus_Region_Region4(this: *mut Gdiplus_Region, regionData: *const BYTE, size: INT);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@QEAA@PEAUHRGN__@@@Z"]
    pub fn Gdiplus_Region_Region5(this: *mut Gdiplus_Region, hRgn: HRGN);
}
extern "C" {
    #[link_name = "\u{1}??0Region@Gdiplus@@IEAA@PEAVGpRegion@1@@Z"]
    pub fn Gdiplus_Region_Region6(this: *mut Gdiplus_Region, nativeRegion: *mut Gdiplus_GpRegion);
}
extern "C" {
    #[link_name = "\u{1}??_DRegion@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_Region_Region_destructor(this: *mut Gdiplus_Region);
}
impl Gdiplus_Region {
    #[inline]
    pub unsafe fn FromHRGN(hRgn: HRGN) -> *mut Gdiplus_Region {
        Gdiplus_Region_FromHRGN(hRgn)
    }
    #[inline]
    pub unsafe fn Clone(&self) -> *mut Gdiplus_Region {
        Gdiplus_Region_Clone(self)
    }
    #[inline]
    pub unsafe fn MakeInfinite(&mut self) -> Gdiplus_Status {
        Gdiplus_Region_MakeInfinite(self)
    }
    #[inline]
    pub unsafe fn MakeEmpty(&mut self) -> Gdiplus_Status {
        Gdiplus_Region_MakeEmpty(self)
    }
    #[inline]
    pub unsafe fn GetDataSize(&self) -> UINT {
        Gdiplus_Region_GetDataSize(self)
    }
    #[inline]
    pub unsafe fn GetData(
        &self,
        buffer: *mut BYTE,
        bufferSize: UINT,
        sizeFilled: *mut UINT,
    ) -> Gdiplus_Status {
        Gdiplus_Region_GetData(self, buffer, bufferSize, sizeFilled)
    }
    #[inline]
    pub unsafe fn Intersect(&mut self, rect: *const Gdiplus_Rect) -> Gdiplus_Status {
        Gdiplus_Region_Intersect(self, rect)
    }
    #[inline]
    pub unsafe fn Intersect1(&mut self, rect: *const Gdiplus_RectF) -> Gdiplus_Status {
        Gdiplus_Region_Intersect1(self, rect)
    }
    #[inline]
    pub unsafe fn Intersect2(&mut self, path: *const Gdiplus_GraphicsPath) -> Gdiplus_Status {
        Gdiplus_Region_Intersect2(self, path)
    }
    #[inline]
    pub unsafe fn Intersect3(&mut self, region: *const Gdiplus_Region) -> Gdiplus_Status {
        Gdiplus_Region_Intersect3(self, region)
    }
    #[inline]
    pub unsafe fn Union(&mut self, rect: *const Gdiplus_Rect) -> Gdiplus_Status {
        Gdiplus_Region_Union(self, rect)
    }
    #[inline]
    pub unsafe fn Union1(&mut self, rect: *const Gdiplus_RectF) -> Gdiplus_Status {
        Gdiplus_Region_Union1(self, rect)
    }
    #[inline]
    pub unsafe fn Union2(&mut self, path: *const Gdiplus_GraphicsPath) -> Gdiplus_Status {
        Gdiplus_Region_Union2(self, path)
    }
    #[inline]
    pub unsafe fn Union3(&mut self, region: *const Gdiplus_Region) -> Gdiplus_Status {
        Gdiplus_Region_Union3(self, region)
    }
    #[inline]
    pub unsafe fn Xor(&mut self, rect: *const Gdiplus_Rect) -> Gdiplus_Status {
        Gdiplus_Region_Xor(self, rect)
    }
    #[inline]
    pub unsafe fn Xor1(&mut self, rect: *const Gdiplus_RectF) -> Gdiplus_Status {
        Gdiplus_Region_Xor1(self, rect)
    }
    #[inline]
    pub unsafe fn Xor2(&mut self, path: *const Gdiplus_GraphicsPath) -> Gdiplus_Status {
        Gdiplus_Region_Xor2(self, path)
    }
    #[inline]
    pub unsafe fn Xor3(&mut self, region: *const Gdiplus_Region) -> Gdiplus_Status {
        Gdiplus_Region_Xor3(self, region)
    }
    #[inline]
    pub unsafe fn Exclude(&mut self, rect: *const Gdiplus_Rect) -> Gdiplus_Status {
        Gdiplus_Region_Exclude(self, rect)
    }
    #[inline]
    pub unsafe fn Exclude1(&mut self, rect: *const Gdiplus_RectF) -> Gdiplus_Status {
        Gdiplus_Region_Exclude1(self, rect)
    }
    #[inline]
    pub unsafe fn Exclude2(&mut self, path: *const Gdiplus_GraphicsPath) -> Gdiplus_Status {
        Gdiplus_Region_Exclude2(self, path)
    }
    #[inline]
    pub unsafe fn Exclude3(&mut self, region: *const Gdiplus_Region) -> Gdiplus_Status {
        Gdiplus_Region_Exclude3(self, region)
    }
    #[inline]
    pub unsafe fn Complement(&mut self, rect: *const Gdiplus_Rect) -> Gdiplus_Status {
        Gdiplus_Region_Complement(self, rect)
    }
    #[inline]
    pub unsafe fn Complement1(&mut self, rect: *const Gdiplus_RectF) -> Gdiplus_Status {
        Gdiplus_Region_Complement1(self, rect)
    }
    #[inline]
    pub unsafe fn Complement2(&mut self, path: *const Gdiplus_GraphicsPath) -> Gdiplus_Status {
        Gdiplus_Region_Complement2(self, path)
    }
    #[inline]
    pub unsafe fn Complement3(&mut self, region: *const Gdiplus_Region) -> Gdiplus_Status {
        Gdiplus_Region_Complement3(self, region)
    }
    #[inline]
    pub unsafe fn Translate(&mut self, dx: Gdiplus_REAL, dy: Gdiplus_REAL) -> Gdiplus_Status {
        Gdiplus_Region_Translate(self, dx, dy)
    }
    #[inline]
    pub unsafe fn Translate1(&mut self, dx: INT, dy: INT) -> Gdiplus_Status {
        Gdiplus_Region_Translate1(self, dx, dy)
    }
    #[inline]
    pub unsafe fn Transform(&mut self, matrix: *const Gdiplus_Matrix) -> Gdiplus_Status {
        Gdiplus_Region_Transform(self, matrix)
    }
    #[inline]
    pub unsafe fn GetBounds(
        &self,
        rect: *mut Gdiplus_Rect,
        g: *const Gdiplus_Graphics,
    ) -> Gdiplus_Status {
        Gdiplus_Region_GetBounds(self, rect, g)
    }
    #[inline]
    pub unsafe fn GetBounds1(
        &self,
        rect: *mut Gdiplus_RectF,
        g: *const Gdiplus_Graphics,
    ) -> Gdiplus_Status {
        Gdiplus_Region_GetBounds1(self, rect, g)
    }
    #[inline]
    pub unsafe fn GetHRGN(&self, g: *const Gdiplus_Graphics) -> HRGN {
        Gdiplus_Region_GetHRGN(self, g)
    }
    #[inline]
    pub unsafe fn IsEmpty(&self, g: *const Gdiplus_Graphics) -> BOOL {
        Gdiplus_Region_IsEmpty(self, g)
    }
    #[inline]
    pub unsafe fn IsInfinite(&self, g: *const Gdiplus_Graphics) -> BOOL {
        Gdiplus_Region_IsInfinite(self, g)
    }
    #[inline]
    pub unsafe fn IsVisible(
        &self,
        point: *const Gdiplus_Point,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_Region_IsVisible(self, point, g)
    }
    #[inline]
    pub unsafe fn IsVisible1(
        &self,
        point: *const Gdiplus_PointF,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_Region_IsVisible1(self, point, g)
    }
    #[inline]
    pub unsafe fn IsVisible2(&self, rect: *const Gdiplus_Rect, g: *const Gdiplus_Graphics) -> BOOL {
        Gdiplus_Region_IsVisible2(self, rect, g)
    }
    #[inline]
    pub unsafe fn IsVisible3(
        &self,
        rect: *const Gdiplus_RectF,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_Region_IsVisible3(self, rect, g)
    }
    #[inline]
    pub unsafe fn Equals(&self, region: *const Gdiplus_Region, g: *const Gdiplus_Graphics) -> BOOL {
        Gdiplus_Region_Equals(self, region, g)
    }
    #[inline]
    pub unsafe fn GetRegionScansCount(&self, matrix: *const Gdiplus_Matrix) -> UINT {
        Gdiplus_Region_GetRegionScansCount(self, matrix)
    }
    #[inline]
    pub unsafe fn GetRegionScans(
        &self,
        matrix: *const Gdiplus_Matrix,
        rects: *mut Gdiplus_RectF,
        count: *mut INT,
    ) -> Gdiplus_Status {
        Gdiplus_Region_GetRegionScans(self, matrix, rects, count)
    }
    #[inline]
    pub unsafe fn GetRegionScans1(
        &self,
        matrix: *const Gdiplus_Matrix,
        rects: *mut Gdiplus_Rect,
        count: *mut INT,
    ) -> Gdiplus_Status {
        Gdiplus_Region_GetRegionScans1(self, matrix, rects, count)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_Region_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn SetNativeRegion(&mut self, nativeRegion: *mut Gdiplus_GpRegion) {
        Gdiplus_Region_SetNativeRegion(self, nativeRegion)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(rect: *const Gdiplus_RectF) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region1(__bindgen_tmp.as_mut_ptr(), rect);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(rect: *const Gdiplus_Rect) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region2(__bindgen_tmp.as_mut_ptr(), rect);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(path: *const Gdiplus_GraphicsPath) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region3(__bindgen_tmp.as_mut_ptr(), path);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new4(regionData: *const BYTE, size: INT) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region4(__bindgen_tmp.as_mut_ptr(), regionData, size);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new5(hRgn: HRGN) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region5(__bindgen_tmp.as_mut_ptr(), hRgn);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new6(nativeRegion: *mut Gdiplus_GpRegion) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Region_Region6(__bindgen_tmp.as_mut_ptr(), nativeRegion);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Gdiplus_Region_Region_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_FontFamily {
    pub nativeFamily: *mut Gdiplus_GpFontFamily,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GenericSansSerif@FontFamily@Gdiplus@@SAPEBV12@XZ"]
    pub fn Gdiplus_FontFamily_GenericSansSerif() -> *const Gdiplus_FontFamily;
}
extern "C" {
    #[link_name = "\u{1}?GenericSerif@FontFamily@Gdiplus@@SAPEBV12@XZ"]
    pub fn Gdiplus_FontFamily_GenericSerif() -> *const Gdiplus_FontFamily;
}
extern "C" {
    #[link_name = "\u{1}?GenericMonospace@FontFamily@Gdiplus@@SAPEBV12@XZ"]
    pub fn Gdiplus_FontFamily_GenericMonospace() -> *const Gdiplus_FontFamily;
}
extern "C" {
    #[link_name = "\u{1}?GetFamilyName@FontFamily@Gdiplus@@QEBA?AW4Status@2@PEA_WG@Z"]
    pub fn Gdiplus_FontFamily_GetFamilyName(
        this: *const Gdiplus_FontFamily,
        name: LPWSTR,
        language: LANGID,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Clone@FontFamily@Gdiplus@@QEBAPEAV12@XZ"]
    pub fn Gdiplus_FontFamily_Clone(this: *const Gdiplus_FontFamily) -> *mut Gdiplus_FontFamily;
}
extern "C" {
    #[link_name = "\u{1}?IsStyleAvailable@FontFamily@Gdiplus@@QEBAHH@Z"]
    pub fn Gdiplus_FontFamily_IsStyleAvailable(this: *const Gdiplus_FontFamily, style: INT)
        -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?GetEmHeight@FontFamily@Gdiplus@@QEBAGH@Z"]
    pub fn Gdiplus_FontFamily_GetEmHeight(this: *const Gdiplus_FontFamily, style: INT) -> UINT16;
}
extern "C" {
    #[link_name = "\u{1}?GetCellAscent@FontFamily@Gdiplus@@QEBAGH@Z"]
    pub fn Gdiplus_FontFamily_GetCellAscent(this: *const Gdiplus_FontFamily, style: INT) -> UINT16;
}
extern "C" {
    #[link_name = "\u{1}?GetCellDescent@FontFamily@Gdiplus@@QEBAGH@Z"]
    pub fn Gdiplus_FontFamily_GetCellDescent(this: *const Gdiplus_FontFamily, style: INT)
        -> UINT16;
}
extern "C" {
    #[link_name = "\u{1}?GetLineSpacing@FontFamily@Gdiplus@@QEBAGH@Z"]
    pub fn Gdiplus_FontFamily_GetLineSpacing(this: *const Gdiplus_FontFamily, style: INT)
        -> UINT16;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@FontFamily@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_FontFamily_GetLastStatus(this: *const Gdiplus_FontFamily) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetStatus@FontFamily@Gdiplus@@IEBA?AW4Status@2@W432@@Z"]
    pub fn Gdiplus_FontFamily_SetStatus(
        this: *const Gdiplus_FontFamily,
        status: Gdiplus_Status,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0FontFamily@Gdiplus@@QEAA@XZ"]
    pub fn Gdiplus_FontFamily_FontFamily(this: *mut Gdiplus_FontFamily);
}
extern "C" {
    #[link_name = "\u{1}??0FontFamily@Gdiplus@@QEAA@PEB_WPEBVFontCollection@1@@Z"]
    pub fn Gdiplus_FontFamily_FontFamily1(
        this: *mut Gdiplus_FontFamily,
        name: *const WCHAR,
        fontCollection: *const Gdiplus_FontCollection,
    );
}
extern "C" {
    #[link_name = "\u{1}??0FontFamily@Gdiplus@@IEAA@PEAVGpFontFamily@1@W4Status@1@@Z"]
    pub fn Gdiplus_FontFamily_FontFamily2(
        this: *mut Gdiplus_FontFamily,
        nativeFamily: *mut Gdiplus_GpFontFamily,
        status: Gdiplus_Status,
    );
}
extern "C" {
    #[link_name = "\u{1}??_DFontFamily@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_FontFamily_FontFamily_destructor(this: *mut Gdiplus_FontFamily);
}
impl Gdiplus_FontFamily {
    #[inline]
    pub unsafe fn GenericSansSerif() -> *const Gdiplus_FontFamily {
        Gdiplus_FontFamily_GenericSansSerif()
    }
    #[inline]
    pub unsafe fn GenericSerif() -> *const Gdiplus_FontFamily {
        Gdiplus_FontFamily_GenericSerif()
    }
    #[inline]
    pub unsafe fn GenericMonospace() -> *const Gdiplus_FontFamily {
        Gdiplus_FontFamily_GenericMonospace()
    }
    #[inline]
    pub unsafe fn GetFamilyName(&self, name: LPWSTR, language: LANGID) -> Gdiplus_Status {
        Gdiplus_FontFamily_GetFamilyName(self, name, language)
    }
    #[inline]
    pub unsafe fn Clone(&self) -> *mut Gdiplus_FontFamily {
        Gdiplus_FontFamily_Clone(self)
    }
    #[inline]
    pub unsafe fn IsStyleAvailable(&self, style: INT) -> BOOL {
        Gdiplus_FontFamily_IsStyleAvailable(self, style)
    }
    #[inline]
    pub unsafe fn GetEmHeight(&self, style: INT) -> UINT16 {
        Gdiplus_FontFamily_GetEmHeight(self, style)
    }
    #[inline]
    pub unsafe fn GetCellAscent(&self, style: INT) -> UINT16 {
        Gdiplus_FontFamily_GetCellAscent(self, style)
    }
    #[inline]
    pub unsafe fn GetCellDescent(&self, style: INT) -> UINT16 {
        Gdiplus_FontFamily_GetCellDescent(self, style)
    }
    #[inline]
    pub unsafe fn GetLineSpacing(&self, style: INT) -> UINT16 {
        Gdiplus_FontFamily_GetLineSpacing(self, style)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_FontFamily_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn SetStatus(&self, status: Gdiplus_Status) -> Gdiplus_Status {
        Gdiplus_FontFamily_SetStatus(self, status)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_FontFamily_FontFamily(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(name: *const WCHAR, fontCollection: *const Gdiplus_FontCollection) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_FontFamily_FontFamily1(__bindgen_tmp.as_mut_ptr(), name, fontCollection);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(nativeFamily: *mut Gdiplus_GpFontFamily, status: Gdiplus_Status) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_FontFamily_FontFamily2(__bindgen_tmp.as_mut_ptr(), nativeFamily, status);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Gdiplus_FontFamily_FontFamily_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Font {
    pub nativeFont: *mut Gdiplus_GpFont,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GetLogFontA@Font@Gdiplus@@QEBA?AW4Status@2@PEBVGraphics@2@PEAUtagLOGFONTA@@@Z"]
    pub fn Gdiplus_Font_GetLogFontA(
        this: *const Gdiplus_Font,
        g: *const Gdiplus_Graphics,
        logfontA: *mut LOGFONTA,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetLogFontW@Font@Gdiplus@@QEBA?AW4Status@2@PEBVGraphics@2@PEAUtagLOGFONTW@@@Z"]
    pub fn Gdiplus_Font_GetLogFontW(
        this: *const Gdiplus_Font,
        g: *const Gdiplus_Graphics,
        logfontW: *mut LOGFONTW,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Font@Gdiplus@@QEBAPEAV12@XZ"]
    pub fn Gdiplus_Font_Clone(this: *const Gdiplus_Font) -> *mut Gdiplus_Font;
}
extern "C" {
    #[link_name = "\u{1}?IsAvailable@Font@Gdiplus@@QEBAHXZ"]
    pub fn Gdiplus_Font_IsAvailable(this: *const Gdiplus_Font) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?GetStyle@Font@Gdiplus@@QEBAHXZ"]
    pub fn Gdiplus_Font_GetStyle(this: *const Gdiplus_Font) -> INT;
}
extern "C" {
    #[link_name = "\u{1}?GetSize@Font@Gdiplus@@QEBAMXZ"]
    pub fn Gdiplus_Font_GetSize(this: *const Gdiplus_Font) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetUnit@Font@Gdiplus@@QEBA?AW4Unit@2@XZ"]
    pub fn Gdiplus_Font_GetUnit(this: *const Gdiplus_Font) -> Gdiplus_Unit;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@Font@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_Font_GetLastStatus(this: *const Gdiplus_Font) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetHeight@Font@Gdiplus@@QEBAMPEBVGraphics@2@@Z"]
    pub fn Gdiplus_Font_GetHeight(
        this: *const Gdiplus_Font,
        graphics: *const Gdiplus_Graphics,
    ) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetHeight@Font@Gdiplus@@QEBAMM@Z"]
    pub fn Gdiplus_Font_GetHeight1(this: *const Gdiplus_Font, dpi: Gdiplus_REAL) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetFamily@Font@Gdiplus@@QEBA?AW4Status@2@PEAVFontFamily@2@@Z"]
    pub fn Gdiplus_Font_GetFamily(
        this: *const Gdiplus_Font,
        family: *mut Gdiplus_FontFamily,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetNativeFont@Font@Gdiplus@@IEAAXPEAVGpFont@2@@Z"]
    pub fn Gdiplus_Font_SetNativeFont(this: *mut Gdiplus_Font, Font: *mut Gdiplus_GpFont);
}
extern "C" {
    #[link_name = "\u{1}?SetStatus@Font@Gdiplus@@IEBA?AW4Status@2@W432@@Z"]
    pub fn Gdiplus_Font_SetStatus(
        this: *const Gdiplus_Font,
        status: Gdiplus_Status,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEAUHDC__@@@Z"]
    pub fn Gdiplus_Font_Font(this: *mut Gdiplus_Font, hdc: HDC);
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEAUHDC__@@PEBUtagLOGFONTA@@@Z"]
    pub fn Gdiplus_Font_Font1(this: *mut Gdiplus_Font, hdc: HDC, logfont: *const LOGFONTA);
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEAUHDC__@@PEBUtagLOGFONTW@@@Z"]
    pub fn Gdiplus_Font_Font2(this: *mut Gdiplus_Font, hdc: HDC, logfont: *const LOGFONTW);
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEAUHDC__@@QEAUHFONT__@@@Z"]
    pub fn Gdiplus_Font_Font3(this: *mut Gdiplus_Font, hdc: HDC, hfont: HFONT);
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEBVFontFamily@1@MHW4Unit@1@@Z"]
    pub fn Gdiplus_Font_Font4(
        this: *mut Gdiplus_Font,
        family: *const Gdiplus_FontFamily,
        emSize: Gdiplus_REAL,
        style: INT,
        unit: Gdiplus_Unit,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@QEAA@PEB_WMHW4Unit@1@PEBVFontCollection@1@@Z"]
    pub fn Gdiplus_Font_Font5(
        this: *mut Gdiplus_Font,
        familyName: *const WCHAR,
        emSize: Gdiplus_REAL,
        style: INT,
        unit: Gdiplus_Unit,
        fontCollection: *const Gdiplus_FontCollection,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Font@Gdiplus@@IEAA@PEAVGpFont@1@W4Status@1@@Z"]
    pub fn Gdiplus_Font_Font6(
        this: *mut Gdiplus_Font,
        font: *mut Gdiplus_GpFont,
        status: Gdiplus_Status,
    );
}
extern "C" {
    #[link_name = "\u{1}??_DFont@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_Font_Font_destructor(this: *mut Gdiplus_Font);
}
impl Gdiplus_Font {
    #[inline]
    pub unsafe fn GetLogFontA(
        &self,
        g: *const Gdiplus_Graphics,
        logfontA: *mut LOGFONTA,
    ) -> Gdiplus_Status {
        Gdiplus_Font_GetLogFontA(self, g, logfontA)
    }
    #[inline]
    pub unsafe fn GetLogFontW(
        &self,
        g: *const Gdiplus_Graphics,
        logfontW: *mut LOGFONTW,
    ) -> Gdiplus_Status {
        Gdiplus_Font_GetLogFontW(self, g, logfontW)
    }
    #[inline]
    pub unsafe fn Clone(&self) -> *mut Gdiplus_Font {
        Gdiplus_Font_Clone(self)
    }
    #[inline]
    pub unsafe fn IsAvailable(&self) -> BOOL {
        Gdiplus_Font_IsAvailable(self)
    }
    #[inline]
    pub unsafe fn GetStyle(&self) -> INT {
        Gdiplus_Font_GetStyle(self)
    }
    #[inline]
    pub unsafe fn GetSize(&self) -> Gdiplus_REAL {
        Gdiplus_Font_GetSize(self)
    }
    #[inline]
    pub unsafe fn GetUnit(&self) -> Gdiplus_Unit {
        Gdiplus_Font_GetUnit(self)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_Font_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn GetHeight(&self, graphics: *const Gdiplus_Graphics) -> Gdiplus_REAL {
        Gdiplus_Font_GetHeight(self, graphics)
    }
    #[inline]
    pub unsafe fn GetHeight1(&self, dpi: Gdiplus_REAL) -> Gdiplus_REAL {
        Gdiplus_Font_GetHeight1(self, dpi)
    }
    #[inline]
    pub unsafe fn GetFamily(&self, family: *mut Gdiplus_FontFamily) -> Gdiplus_Status {
        Gdiplus_Font_GetFamily(self, family)
    }
    #[inline]
    pub unsafe fn SetNativeFont(&mut self, Font: *mut Gdiplus_GpFont) {
        Gdiplus_Font_SetNativeFont(self, Font)
    }
    #[inline]
    pub unsafe fn SetStatus(&self, status: Gdiplus_Status) -> Gdiplus_Status {
        Gdiplus_Font_SetStatus(self, status)
    }
    #[inline]
    pub unsafe fn new(hdc: HDC) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font(__bindgen_tmp.as_mut_ptr(), hdc);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(hdc: HDC, logfont: *const LOGFONTA) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font1(__bindgen_tmp.as_mut_ptr(), hdc, logfont);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(hdc: HDC, logfont: *const LOGFONTW) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font2(__bindgen_tmp.as_mut_ptr(), hdc, logfont);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(hdc: HDC, hfont: HFONT) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font3(__bindgen_tmp.as_mut_ptr(), hdc, hfont);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new4(
        family: *const Gdiplus_FontFamily,
        emSize: Gdiplus_REAL,
        style: INT,
        unit: Gdiplus_Unit,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font4(__bindgen_tmp.as_mut_ptr(), family, emSize, style, unit);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new5(
        familyName: *const WCHAR,
        emSize: Gdiplus_REAL,
        style: INT,
        unit: Gdiplus_Unit,
        fontCollection: *const Gdiplus_FontCollection,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font5(
            __bindgen_tmp.as_mut_ptr(),
            familyName,
            emSize,
            style,
            unit,
            fontCollection,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new6(font: *mut Gdiplus_GpFont, status: Gdiplus_Status) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Font_Font6(__bindgen_tmp.as_mut_ptr(), font, status);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn destruct(&mut self) {
        Gdiplus_Font_Font_destructor(self)
    }
}
#[repr(C)]
pub struct Gdiplus_FontCollection__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_FontCollection {
    pub vtable_: *const Gdiplus_FontCollection__bindgen_vtable,
    pub nativeFontCollection: *mut Gdiplus_GpFontCollection,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GetFamilyCount@FontCollection@Gdiplus@@QEBAHXZ"]
    pub fn Gdiplus_FontCollection_GetFamilyCount(this: *const Gdiplus_FontCollection) -> INT;
}
extern "C" {
    #[link_name = "\u{1}?GetFamilies@FontCollection@Gdiplus@@QEBA?AW4Status@2@HPEAVFontFamily@2@PEAH@Z"]
    pub fn Gdiplus_FontCollection_GetFamilies(
        this: *const Gdiplus_FontCollection,
        numSought: INT,
        gpfamilies: *mut Gdiplus_FontFamily,
        numFound: *mut INT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@FontCollection@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_FontCollection_GetLastStatus(
        this: *const Gdiplus_FontCollection,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetStatus@FontCollection@Gdiplus@@IEBA?AW4Status@2@W432@@Z"]
    pub fn Gdiplus_FontCollection_SetStatus(
        this: *const Gdiplus_FontCollection,
        status: Gdiplus_Status,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0FontCollection@Gdiplus@@QEAA@XZ"]
    pub fn Gdiplus_FontCollection_FontCollection(this: *mut Gdiplus_FontCollection);
}
impl Gdiplus_FontCollection {
    #[inline]
    pub unsafe fn GetFamilyCount(&self) -> INT {
        Gdiplus_FontCollection_GetFamilyCount(self)
    }
    #[inline]
    pub unsafe fn GetFamilies(
        &self,
        numSought: INT,
        gpfamilies: *mut Gdiplus_FontFamily,
        numFound: *mut INT,
    ) -> Gdiplus_Status {
        Gdiplus_FontCollection_GetFamilies(self, numSought, gpfamilies, numFound)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_FontCollection_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn SetStatus(&self, status: Gdiplus_Status) -> Gdiplus_Status {
        Gdiplus_FontCollection_SetStatus(self, status)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_FontCollection_FontCollection(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DFontCollection@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_FontCollection_FontCollection_destructor(this: *mut Gdiplus_FontCollection);
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_InstalledFontCollection {
    pub _base: Gdiplus_FontCollection,
}
extern "C" {
    #[link_name = "\u{1}?SetStatus@InstalledFontCollection@Gdiplus@@IEBA?AW4Status@2@W432@@Z"]
    pub fn Gdiplus_InstalledFontCollection_SetStatus(
        this: *const Gdiplus_InstalledFontCollection,
        status: Gdiplus_Status,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0InstalledFontCollection@Gdiplus@@QEAA@XZ"]
    pub fn Gdiplus_InstalledFontCollection_InstalledFontCollection(
        this: *mut Gdiplus_InstalledFontCollection,
    );
}
impl Gdiplus_InstalledFontCollection {
    #[inline]
    pub unsafe fn SetStatus(&self, status: Gdiplus_Status) -> Gdiplus_Status {
        Gdiplus_InstalledFontCollection_SetStatus(self, status)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_InstalledFontCollection_InstalledFontCollection(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DInstalledFontCollection@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_InstalledFontCollection_InstalledFontCollection_destructor(
        this: *mut Gdiplus_InstalledFontCollection,
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_PrivateFontCollection {
    pub _base: Gdiplus_FontCollection,
}
extern "C" {
    #[link_name = "\u{1}?AddFontFile@PrivateFontCollection@Gdiplus@@QEAA?AW4Status@2@PEB_W@Z"]
    pub fn Gdiplus_PrivateFontCollection_AddFontFile(
        this: *mut Gdiplus_PrivateFontCollection,
        filename: *const WCHAR,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?AddMemoryFont@PrivateFontCollection@Gdiplus@@QEAA?AW4Status@2@PEBXH@Z"]
    pub fn Gdiplus_PrivateFontCollection_AddMemoryFont(
        this: *mut Gdiplus_PrivateFontCollection,
        memory: *const c_void,
        length: INT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0PrivateFontCollection@Gdiplus@@QEAA@XZ"]
    pub fn Gdiplus_PrivateFontCollection_PrivateFontCollection(
        this: *mut Gdiplus_PrivateFontCollection,
    );
}
impl Gdiplus_PrivateFontCollection {
    #[inline]
    pub unsafe fn AddFontFile(&mut self, filename: *const WCHAR) -> Gdiplus_Status {
        Gdiplus_PrivateFontCollection_AddFontFile(self, filename)
    }
    #[inline]
    pub unsafe fn AddMemoryFont(&mut self, memory: *const c_void, length: INT) -> Gdiplus_Status {
        Gdiplus_PrivateFontCollection_AddMemoryFont(self, memory, length)
    }
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_PrivateFontCollection_PrivateFontCollection(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DPrivateFontCollection@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_PrivateFontCollection_PrivateFontCollection_destructor(
        this: *mut Gdiplus_PrivateFontCollection,
    );
}
#[repr(C)]
pub struct Gdiplus_Image__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Image {
    pub vtable_: *const Gdiplus_Image__bindgen_vtable,
    pub nativeImage: *mut Gdiplus_GpImage,
    pub lastResult: Gdiplus_Status,
    pub loadStatus: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?FromFile@Image@Gdiplus@@SAPEAV12@PEB_WH@Z"]
    pub fn Gdiplus_Image_FromFile(
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Image;
}
extern "C" {
    #[link_name = "\u{1}?FromStream@Image@Gdiplus@@SAPEAV12@PEAUIStream@@H@Z"]
    pub fn Gdiplus_Image_FromStream(
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Image;
}
extern "C" {
    #[link_name = "\u{1}?Save@Image@Gdiplus@@QEAA?AW4Status@2@PEB_WPEBU_GUID@@PEBVEncoderParameters@2@@Z"]
    pub fn Gdiplus_Image_Save(
        this: *mut Gdiplus_Image,
        filename: *const WCHAR,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?Save@Image@Gdiplus@@QEAA?AW4Status@2@PEAUIStream@@PEBU_GUID@@PEBVEncoderParameters@2@@Z"]
    pub fn Gdiplus_Image_Save1(
        this: *mut Gdiplus_Image,
        stream: *mut IStream,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SaveAdd@Image@Gdiplus@@QEAA?AW4Status@2@PEBVEncoderParameters@2@@Z"]
    pub fn Gdiplus_Image_SaveAdd(
        this: *mut Gdiplus_Image,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SaveAdd@Image@Gdiplus@@QEAA?AW4Status@2@PEAV12@PEBVEncoderParameters@2@@Z"]
    pub fn Gdiplus_Image_SaveAdd1(
        this: *mut Gdiplus_Image,
        newImage: *mut Gdiplus_Image,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetType@Image@Gdiplus@@QEBA?AW4ImageType@2@XZ"]
    pub fn Gdiplus_Image_GetType(this: *const Gdiplus_Image) -> Gdiplus_ImageType;
}
extern "C" {
    #[link_name = "\u{1}?GetPhysicalDimension@Image@Gdiplus@@QEAA?AW4Status@2@PEAVSizeF@2@@Z"]
    pub fn Gdiplus_Image_GetPhysicalDimension(
        this: *mut Gdiplus_Image,
        size: *mut Gdiplus_SizeF,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBounds@Image@Gdiplus@@QEAA?AW4Status@2@PEAVRectF@2@PEAW4Unit@2@@Z"]
    pub fn Gdiplus_Image_GetBounds(
        this: *mut Gdiplus_Image,
        srcRect: *mut Gdiplus_RectF,
        srcUnit: *mut Gdiplus_Unit,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetWidth@Image@Gdiplus@@QEAAIXZ"]
    pub fn Gdiplus_Image_GetWidth(this: *mut Gdiplus_Image) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetHeight@Image@Gdiplus@@QEAAIXZ"]
    pub fn Gdiplus_Image_GetHeight(this: *mut Gdiplus_Image) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetHorizontalResolution@Image@Gdiplus@@QEAAMXZ"]
    pub fn Gdiplus_Image_GetHorizontalResolution(this: *mut Gdiplus_Image) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetVerticalResolution@Image@Gdiplus@@QEAAMXZ"]
    pub fn Gdiplus_Image_GetVerticalResolution(this: *mut Gdiplus_Image) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetFlags@Image@Gdiplus@@QEAAIXZ"]
    pub fn Gdiplus_Image_GetFlags(this: *mut Gdiplus_Image) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetRawFormat@Image@Gdiplus@@QEAA?AW4Status@2@PEAU_GUID@@@Z"]
    pub fn Gdiplus_Image_GetRawFormat(
        this: *mut Gdiplus_Image,
        format: *mut GUID,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetPixelFormat@Image@Gdiplus@@QEAAHXZ"]
    pub fn Gdiplus_Image_GetPixelFormat(this: *mut Gdiplus_Image) -> Gdiplus_PixelFormat;
}
extern "C" {
    #[link_name = "\u{1}?GetPaletteSize@Image@Gdiplus@@QEAAHXZ"]
    pub fn Gdiplus_Image_GetPaletteSize(this: *mut Gdiplus_Image) -> INT;
}
extern "C" {
    #[link_name = "\u{1}?GetPalette@Image@Gdiplus@@QEAA?AW4Status@2@PEAUColorPalette@2@H@Z"]
    pub fn Gdiplus_Image_GetPalette(
        this: *mut Gdiplus_Image,
        palette: *mut Gdiplus_ColorPalette,
        size: INT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetPalette@Image@Gdiplus@@QEAA?AW4Status@2@PEBUColorPalette@2@@Z"]
    pub fn Gdiplus_Image_SetPalette(
        this: *mut Gdiplus_Image,
        palette: *const Gdiplus_ColorPalette,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetThumbnailImage@Image@Gdiplus@@QEAAPEAV12@IIP6AHPEAX@Z0@Z"]
    pub fn Gdiplus_Image_GetThumbnailImage(
        this: *mut Gdiplus_Image,
        thumbWidth: UINT,
        thumbHeight: UINT,
        callback: Gdiplus_GetThumbnailImageAbort,
        callbackData: *mut c_void,
    ) -> *mut Gdiplus_Image;
}
extern "C" {
    #[link_name = "\u{1}?GetFrameDimensionsCount@Image@Gdiplus@@QEAAIXZ"]
    pub fn Gdiplus_Image_GetFrameDimensionsCount(this: *mut Gdiplus_Image) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetFrameDimensionsList@Image@Gdiplus@@QEAA?AW4Status@2@PEAU_GUID@@I@Z"]
    pub fn Gdiplus_Image_GetFrameDimensionsList(
        this: *mut Gdiplus_Image,
        dimensionIDs: *mut GUID,
        count: UINT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetFrameCount@Image@Gdiplus@@QEAAIPEBU_GUID@@@Z"]
    pub fn Gdiplus_Image_GetFrameCount(this: *mut Gdiplus_Image, dimensionID: *const GUID) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?SelectActiveFrame@Image@Gdiplus@@QEAA?AW4Status@2@PEBU_GUID@@I@Z"]
    pub fn Gdiplus_Image_SelectActiveFrame(
        this: *mut Gdiplus_Image,
        dimensionID: *const GUID,
        frameIndex: UINT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?RotateFlip@Image@Gdiplus@@QEAA?AW4Status@2@W4RotateFlipType@2@@Z"]
    pub fn Gdiplus_Image_RotateFlip(
        this: *mut Gdiplus_Image,
        rotateFlipType: Gdiplus_RotateFlipType,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetPropertyCount@Image@Gdiplus@@QEAAIXZ"]
    pub fn Gdiplus_Image_GetPropertyCount(this: *mut Gdiplus_Image) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetPropertyIdList@Image@Gdiplus@@QEAA?AW4Status@2@IPEAK@Z"]
    pub fn Gdiplus_Image_GetPropertyIdList(
        this: *mut Gdiplus_Image,
        numOfProperty: UINT,
        list: *mut PROPID,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetPropertyItemSize@Image@Gdiplus@@QEAAIK@Z"]
    pub fn Gdiplus_Image_GetPropertyItemSize(this: *mut Gdiplus_Image, propId: PROPID) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetPropertyItem@Image@Gdiplus@@QEAA?AW4Status@2@KIPEAVPropertyItem@2@@Z"]
    pub fn Gdiplus_Image_GetPropertyItem(
        this: *mut Gdiplus_Image,
        propId: PROPID,
        propSize: UINT,
        buffer: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetPropertySize@Image@Gdiplus@@QEAA?AW4Status@2@PEAI0@Z"]
    pub fn Gdiplus_Image_GetPropertySize(
        this: *mut Gdiplus_Image,
        totalBufferSize: *mut UINT,
        numProperties: *mut UINT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetAllPropertyItems@Image@Gdiplus@@QEAA?AW4Status@2@IIPEAVPropertyItem@2@@Z"]
    pub fn Gdiplus_Image_GetAllPropertyItems(
        this: *mut Gdiplus_Image,
        totalBufferSize: UINT,
        numProperties: UINT,
        allItems: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?RemovePropertyItem@Image@Gdiplus@@QEAA?AW4Status@2@K@Z"]
    pub fn Gdiplus_Image_RemovePropertyItem(
        this: *mut Gdiplus_Image,
        propId: PROPID,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetPropertyItem@Image@Gdiplus@@QEAA?AW4Status@2@PEBVPropertyItem@2@@Z"]
    pub fn Gdiplus_Image_SetPropertyItem(
        this: *mut Gdiplus_Image,
        item: *const Gdiplus_PropertyItem,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetEncoderParameterListSize@Image@Gdiplus@@QEAAIPEBU_GUID@@@Z"]
    pub fn Gdiplus_Image_GetEncoderParameterListSize(
        this: *mut Gdiplus_Image,
        clsidEncoder: *const CLSID,
    ) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?GetEncoderParameterList@Image@Gdiplus@@QEAA?AW4Status@2@PEBU_GUID@@IPEAVEncoderParameters@2@@Z"]
    pub fn Gdiplus_Image_GetEncoderParameterList(
        this: *mut Gdiplus_Image,
        clsidEncoder: *const CLSID,
        size: UINT,
        buffer: *mut Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@Image@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_Image_GetLastStatus(this: *const Gdiplus_Image) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetNativeImage@Image@Gdiplus@@IEAAXPEAVGpImage@2@@Z"]
    pub fn Gdiplus_Image_SetNativeImage(
        this: *mut Gdiplus_Image,
        nativeImage: *mut Gdiplus_GpImage,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Image@Gdiplus@@QEAA@PEB_WH@Z"]
    pub fn Gdiplus_Image_Image(
        this: *mut Gdiplus_Image,
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Image@Gdiplus@@QEAA@PEAUIStream@@H@Z"]
    pub fn Gdiplus_Image_Image1(
        this: *mut Gdiplus_Image,
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Image@Gdiplus@@IEAA@PEAVGpImage@1@W4Status@1@@Z"]
    pub fn Gdiplus_Image_Image2(
        this: *mut Gdiplus_Image,
        nativeImage: *mut Gdiplus_GpImage,
        status: Gdiplus_Status,
    );
}
impl Gdiplus_Image {
    #[inline]
    pub unsafe fn FromFile(
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Image {
        Gdiplus_Image_FromFile(filename, useEmbeddedColorManagement)
    }
    #[inline]
    pub unsafe fn FromStream(
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Image {
        Gdiplus_Image_FromStream(stream, useEmbeddedColorManagement)
    }
    #[inline]
    pub unsafe fn Save(
        &mut self,
        filename: *const WCHAR,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status {
        Gdiplus_Image_Save(self, filename, clsidEncoder, encoderParams)
    }
    #[inline]
    pub unsafe fn Save1(
        &mut self,
        stream: *mut IStream,
        clsidEncoder: *const CLSID,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status {
        Gdiplus_Image_Save1(self, stream, clsidEncoder, encoderParams)
    }
    #[inline]
    pub unsafe fn SaveAdd(
        &mut self,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status {
        Gdiplus_Image_SaveAdd(self, encoderParams)
    }
    #[inline]
    pub unsafe fn SaveAdd1(
        &mut self,
        newImage: *mut Gdiplus_Image,
        encoderParams: *const Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status {
        Gdiplus_Image_SaveAdd1(self, newImage, encoderParams)
    }
    #[inline]
    pub unsafe fn GetType(&self) -> Gdiplus_ImageType {
        Gdiplus_Image_GetType(self)
    }
    #[inline]
    pub unsafe fn GetPhysicalDimension(&mut self, size: *mut Gdiplus_SizeF) -> Gdiplus_Status {
        Gdiplus_Image_GetPhysicalDimension(self, size)
    }
    #[inline]
    pub unsafe fn GetBounds(
        &mut self,
        srcRect: *mut Gdiplus_RectF,
        srcUnit: *mut Gdiplus_Unit,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetBounds(self, srcRect, srcUnit)
    }
    #[inline]
    pub unsafe fn GetWidth(&mut self) -> UINT {
        Gdiplus_Image_GetWidth(self)
    }
    #[inline]
    pub unsafe fn GetHeight(&mut self) -> UINT {
        Gdiplus_Image_GetHeight(self)
    }
    #[inline]
    pub unsafe fn GetHorizontalResolution(&mut self) -> Gdiplus_REAL {
        Gdiplus_Image_GetHorizontalResolution(self)
    }
    #[inline]
    pub unsafe fn GetVerticalResolution(&mut self) -> Gdiplus_REAL {
        Gdiplus_Image_GetVerticalResolution(self)
    }
    #[inline]
    pub unsafe fn GetFlags(&mut self) -> UINT {
        Gdiplus_Image_GetFlags(self)
    }
    #[inline]
    pub unsafe fn GetRawFormat(&mut self, format: *mut GUID) -> Gdiplus_Status {
        Gdiplus_Image_GetRawFormat(self, format)
    }
    #[inline]
    pub unsafe fn GetPixelFormat(&mut self) -> Gdiplus_PixelFormat {
        Gdiplus_Image_GetPixelFormat(self)
    }
    #[inline]
    pub unsafe fn GetPaletteSize(&mut self) -> INT {
        Gdiplus_Image_GetPaletteSize(self)
    }
    #[inline]
    pub unsafe fn GetPalette(
        &mut self,
        palette: *mut Gdiplus_ColorPalette,
        size: INT,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetPalette(self, palette, size)
    }
    #[inline]
    pub unsafe fn SetPalette(&mut self, palette: *const Gdiplus_ColorPalette) -> Gdiplus_Status {
        Gdiplus_Image_SetPalette(self, palette)
    }
    #[inline]
    pub unsafe fn GetThumbnailImage(
        &mut self,
        thumbWidth: UINT,
        thumbHeight: UINT,
        callback: Gdiplus_GetThumbnailImageAbort,
        callbackData: *mut c_void,
    ) -> *mut Gdiplus_Image {
        Gdiplus_Image_GetThumbnailImage(self, thumbWidth, thumbHeight, callback, callbackData)
    }
    #[inline]
    pub unsafe fn GetFrameDimensionsCount(&mut self) -> UINT {
        Gdiplus_Image_GetFrameDimensionsCount(self)
    }
    #[inline]
    pub unsafe fn GetFrameDimensionsList(
        &mut self,
        dimensionIDs: *mut GUID,
        count: UINT,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetFrameDimensionsList(self, dimensionIDs, count)
    }
    #[inline]
    pub unsafe fn GetFrameCount(&mut self, dimensionID: *const GUID) -> UINT {
        Gdiplus_Image_GetFrameCount(self, dimensionID)
    }
    #[inline]
    pub unsafe fn SelectActiveFrame(
        &mut self,
        dimensionID: *const GUID,
        frameIndex: UINT,
    ) -> Gdiplus_Status {
        Gdiplus_Image_SelectActiveFrame(self, dimensionID, frameIndex)
    }
    #[inline]
    pub unsafe fn RotateFlip(&mut self, rotateFlipType: Gdiplus_RotateFlipType) -> Gdiplus_Status {
        Gdiplus_Image_RotateFlip(self, rotateFlipType)
    }
    #[inline]
    pub unsafe fn GetPropertyCount(&mut self) -> UINT {
        Gdiplus_Image_GetPropertyCount(self)
    }
    #[inline]
    pub unsafe fn GetPropertyIdList(
        &mut self,
        numOfProperty: UINT,
        list: *mut PROPID,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetPropertyIdList(self, numOfProperty, list)
    }
    #[inline]
    pub unsafe fn GetPropertyItemSize(&mut self, propId: PROPID) -> UINT {
        Gdiplus_Image_GetPropertyItemSize(self, propId)
    }
    #[inline]
    pub unsafe fn GetPropertyItem(
        &mut self,
        propId: PROPID,
        propSize: UINT,
        buffer: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetPropertyItem(self, propId, propSize, buffer)
    }
    #[inline]
    pub unsafe fn GetPropertySize(
        &mut self,
        totalBufferSize: *mut UINT,
        numProperties: *mut UINT,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetPropertySize(self, totalBufferSize, numProperties)
    }
    #[inline]
    pub unsafe fn GetAllPropertyItems(
        &mut self,
        totalBufferSize: UINT,
        numProperties: UINT,
        allItems: *mut Gdiplus_PropertyItem,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetAllPropertyItems(self, totalBufferSize, numProperties, allItems)
    }
    #[inline]
    pub unsafe fn RemovePropertyItem(&mut self, propId: PROPID) -> Gdiplus_Status {
        Gdiplus_Image_RemovePropertyItem(self, propId)
    }
    #[inline]
    pub unsafe fn SetPropertyItem(&mut self, item: *const Gdiplus_PropertyItem) -> Gdiplus_Status {
        Gdiplus_Image_SetPropertyItem(self, item)
    }
    #[inline]
    pub unsafe fn GetEncoderParameterListSize(&mut self, clsidEncoder: *const CLSID) -> UINT {
        Gdiplus_Image_GetEncoderParameterListSize(self, clsidEncoder)
    }
    #[inline]
    pub unsafe fn GetEncoderParameterList(
        &mut self,
        clsidEncoder: *const CLSID,
        size: UINT,
        buffer: *mut Gdiplus_EncoderParameters,
    ) -> Gdiplus_Status {
        Gdiplus_Image_GetEncoderParameterList(self, clsidEncoder, size, buffer)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_Image_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn SetNativeImage(&mut self, nativeImage: *mut Gdiplus_GpImage) {
        Gdiplus_Image_SetNativeImage(self, nativeImage)
    }
    #[inline]
    pub unsafe fn new(filename: *const WCHAR, useEmbeddedColorManagement: BOOL) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Image_Image(
            __bindgen_tmp.as_mut_ptr(),
            filename,
            useEmbeddedColorManagement,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(stream: *mut IStream, useEmbeddedColorManagement: BOOL) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Image_Image1(
            __bindgen_tmp.as_mut_ptr(),
            stream,
            useEmbeddedColorManagement,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(nativeImage: *mut Gdiplus_GpImage, status: Gdiplus_Status) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Image_Image2(__bindgen_tmp.as_mut_ptr(), nativeImage, status);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DImage@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_Image_Image_destructor(this: *mut Gdiplus_Image);
}
extern "C" {
    #[link_name = "\u{1}?Clone@Image@Gdiplus@@UEAAPEAV12@XZ"]
    pub fn Gdiplus_Image_Clone(this: *mut c_void) -> *mut Gdiplus_Image;
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Bitmap {
    pub _base: Gdiplus_Image,
}
extern "C" {
    #[link_name = "\u{1}?FromFile@Bitmap@Gdiplus@@SAPEAV12@PEB_WH@Z"]
    pub fn Gdiplus_Bitmap_FromFile(
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?FromStream@Bitmap@Gdiplus@@SAPEAV12@PEAUIStream@@H@Z"]
    pub fn Gdiplus_Bitmap_FromStream(
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Bitmap@Gdiplus@@QEAAPEAV12@AEBVRect@2@H@Z"]
    pub fn Gdiplus_Bitmap_Clone(
        this: *mut Gdiplus_Bitmap,
        rect: *const Gdiplus_Rect,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Bitmap@Gdiplus@@QEAAPEAV12@HHHHH@Z"]
    pub fn Gdiplus_Bitmap_Clone1(
        this: *mut Gdiplus_Bitmap,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Bitmap@Gdiplus@@QEAAPEAV12@AEBVRectF@2@H@Z"]
    pub fn Gdiplus_Bitmap_Clone2(
        this: *mut Gdiplus_Bitmap,
        rect: *const Gdiplus_RectF,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?Clone@Bitmap@Gdiplus@@QEAAPEAV12@MMMMH@Z"]
    pub fn Gdiplus_Bitmap_Clone3(
        this: *mut Gdiplus_Bitmap,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?LockBits@Bitmap@Gdiplus@@QEAA?AW4Status@2@PEBVRect@2@IHPEAVBitmapData@2@@Z"]
    pub fn Gdiplus_Bitmap_LockBits(
        this: *mut Gdiplus_Bitmap,
        rect: *const Gdiplus_Rect,
        flags: UINT,
        format: Gdiplus_PixelFormat,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?UnlockBits@Bitmap@Gdiplus@@QEAA?AW4Status@2@PEAVBitmapData@2@@Z"]
    pub fn Gdiplus_Bitmap_UnlockBits(
        this: *mut Gdiplus_Bitmap,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetPixel@Bitmap@Gdiplus@@QEAA?AW4Status@2@HHPEAVColor@2@@Z"]
    pub fn Gdiplus_Bitmap_GetPixel(
        this: *mut Gdiplus_Bitmap,
        x: INT,
        y: INT,
        color: *mut Gdiplus_Color,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetPixel@Bitmap@Gdiplus@@QEAA?AW4Status@2@HHAEBVColor@2@@Z"]
    pub fn Gdiplus_Bitmap_SetPixel(
        this: *mut Gdiplus_Bitmap,
        x: INT,
        y: INT,
        color: *const Gdiplus_Color,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetResolution@Bitmap@Gdiplus@@QEAA?AW4Status@2@MM@Z"]
    pub fn Gdiplus_Bitmap_SetResolution(
        this: *mut Gdiplus_Bitmap,
        xdpi: Gdiplus_REAL,
        ydpi: Gdiplus_REAL,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?FromDirectDrawSurface7@Bitmap@Gdiplus@@SAPEAV12@PEAUIDirectDrawSurface7@@@Z"]
    pub fn Gdiplus_Bitmap_FromDirectDrawSurface7(
        surface: *mut IDirectDrawSurface7,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?FromBITMAPINFO@Bitmap@Gdiplus@@SAPEAV12@PEBUtagBITMAPINFO@@PEAX@Z"]
    pub fn Gdiplus_Bitmap_FromBITMAPINFO(
        gdiBitmapInfo: *const BITMAPINFO,
        gdiBitmapData: *mut c_void,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?FromHBITMAP@Bitmap@Gdiplus@@SAPEAV12@PEAUHBITMAP__@@PEAUHPALETTE__@@@Z"]
    pub fn Gdiplus_Bitmap_FromHBITMAP(hbm: HBITMAP, hpal: HPALETTE) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?FromHICON@Bitmap@Gdiplus@@SAPEAV12@PEAUHICON__@@@Z"]
    pub fn Gdiplus_Bitmap_FromHICON(hicon: HICON) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?FromResource@Bitmap@Gdiplus@@SAPEAV12@PEAUHINSTANCE__@@PEB_W@Z"]
    pub fn Gdiplus_Bitmap_FromResource(
        hInstance: HINSTANCE,
        bitmapName: *const WCHAR,
    ) -> *mut Gdiplus_Bitmap;
}
extern "C" {
    #[link_name = "\u{1}?GetHBITMAP@Bitmap@Gdiplus@@QEAA?AW4Status@2@AEBVColor@2@PEAPEAUHBITMAP__@@@Z"]
    pub fn Gdiplus_Bitmap_GetHBITMAP(
        this: *mut Gdiplus_Bitmap,
        colorBackground: *const Gdiplus_Color,
        hbmReturn: *mut HBITMAP,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetHICON@Bitmap@Gdiplus@@QEAA?AW4Status@2@PEAPEAUHICON__@@@Z"]
    pub fn Gdiplus_Bitmap_GetHICON(this: *mut Gdiplus_Bitmap, hicon: *mut HICON) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEB_WH@Z"]
    pub fn Gdiplus_Bitmap_Bitmap(
        this: *mut Gdiplus_Bitmap,
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEAUIStream@@H@Z"]
    pub fn Gdiplus_Bitmap_Bitmap1(
        this: *mut Gdiplus_Bitmap,
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@HHHHPEAE@Z"]
    pub fn Gdiplus_Bitmap_Bitmap2(
        this: *mut Gdiplus_Bitmap,
        width: INT,
        height: INT,
        stride: INT,
        format: Gdiplus_PixelFormat,
        scan0: *mut BYTE,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@HHH@Z"]
    pub fn Gdiplus_Bitmap_Bitmap3(
        this: *mut Gdiplus_Bitmap,
        width: INT,
        height: INT,
        format: Gdiplus_PixelFormat,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@HHPEAVGraphics@1@@Z"]
    pub fn Gdiplus_Bitmap_Bitmap4(
        this: *mut Gdiplus_Bitmap,
        width: INT,
        height: INT,
        target: *mut Gdiplus_Graphics,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEAUIDirectDrawSurface7@@@Z"]
    pub fn Gdiplus_Bitmap_Bitmap5(this: *mut Gdiplus_Bitmap, surface: *mut IDirectDrawSurface7);
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEBUtagBITMAPINFO@@PEAX@Z"]
    pub fn Gdiplus_Bitmap_Bitmap6(
        this: *mut Gdiplus_Bitmap,
        gdiBitmapInfo: *const BITMAPINFO,
        gdiBitmapData: *mut c_void,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEAUHBITMAP__@@PEAUHPALETTE__@@@Z"]
    pub fn Gdiplus_Bitmap_Bitmap7(this: *mut Gdiplus_Bitmap, hbm: HBITMAP, hpal: HPALETTE);
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEAUHICON__@@@Z"]
    pub fn Gdiplus_Bitmap_Bitmap8(this: *mut Gdiplus_Bitmap, hicon: HICON);
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@QEAA@PEAUHINSTANCE__@@PEB_W@Z"]
    pub fn Gdiplus_Bitmap_Bitmap9(
        this: *mut Gdiplus_Bitmap,
        hInstance: HINSTANCE,
        bitmapName: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Bitmap@Gdiplus@@IEAA@PEAVGpBitmap@1@@Z"]
    pub fn Gdiplus_Bitmap_Bitmap10(this: *mut Gdiplus_Bitmap, nativeBitmap: *mut Gdiplus_GpBitmap);
}
impl Gdiplus_Bitmap {
    #[inline]
    pub unsafe fn FromFile(
        filename: *const WCHAR,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromFile(filename, useEmbeddedColorManagement)
    }
    #[inline]
    pub unsafe fn FromStream(
        stream: *mut IStream,
        useEmbeddedColorManagement: BOOL,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromStream(stream, useEmbeddedColorManagement)
    }
    #[inline]
    pub unsafe fn Clone(
        &mut self,
        rect: *const Gdiplus_Rect,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_Clone(self, rect, format)
    }
    #[inline]
    pub unsafe fn Clone1(
        &mut self,
        x: INT,
        y: INT,
        width: INT,
        height: INT,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_Clone1(self, x, y, width, height, format)
    }
    #[inline]
    pub unsafe fn Clone2(
        &mut self,
        rect: *const Gdiplus_RectF,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_Clone2(self, rect, format)
    }
    #[inline]
    pub unsafe fn Clone3(
        &mut self,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        width: Gdiplus_REAL,
        height: Gdiplus_REAL,
        format: Gdiplus_PixelFormat,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_Clone3(self, x, y, width, height, format)
    }
    #[inline]
    pub unsafe fn LockBits(
        &mut self,
        rect: *const Gdiplus_Rect,
        flags: UINT,
        format: Gdiplus_PixelFormat,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_Status {
        Gdiplus_Bitmap_LockBits(self, rect, flags, format, lockedBitmapData)
    }
    #[inline]
    pub unsafe fn UnlockBits(
        &mut self,
        lockedBitmapData: *mut Gdiplus_BitmapData,
    ) -> Gdiplus_Status {
        Gdiplus_Bitmap_UnlockBits(self, lockedBitmapData)
    }
    #[inline]
    pub unsafe fn GetPixel(&mut self, x: INT, y: INT, color: *mut Gdiplus_Color) -> Gdiplus_Status {
        Gdiplus_Bitmap_GetPixel(self, x, y, color)
    }
    #[inline]
    pub unsafe fn SetPixel(
        &mut self,
        x: INT,
        y: INT,
        color: *const Gdiplus_Color,
    ) -> Gdiplus_Status {
        Gdiplus_Bitmap_SetPixel(self, x, y, color)
    }
    #[inline]
    pub unsafe fn SetResolution(
        &mut self,
        xdpi: Gdiplus_REAL,
        ydpi: Gdiplus_REAL,
    ) -> Gdiplus_Status {
        Gdiplus_Bitmap_SetResolution(self, xdpi, ydpi)
    }
    #[inline]
    pub unsafe fn FromDirectDrawSurface7(surface: *mut IDirectDrawSurface7) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromDirectDrawSurface7(surface)
    }
    #[inline]
    pub unsafe fn FromBITMAPINFO(
        gdiBitmapInfo: *const BITMAPINFO,
        gdiBitmapData: *mut c_void,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromBITMAPINFO(gdiBitmapInfo, gdiBitmapData)
    }
    #[inline]
    pub unsafe fn FromHBITMAP(hbm: HBITMAP, hpal: HPALETTE) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromHBITMAP(hbm, hpal)
    }
    #[inline]
    pub unsafe fn FromHICON(hicon: HICON) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromHICON(hicon)
    }
    #[inline]
    pub unsafe fn FromResource(
        hInstance: HINSTANCE,
        bitmapName: *const WCHAR,
    ) -> *mut Gdiplus_Bitmap {
        Gdiplus_Bitmap_FromResource(hInstance, bitmapName)
    }
    #[inline]
    pub unsafe fn GetHBITMAP(
        &mut self,
        colorBackground: *const Gdiplus_Color,
        hbmReturn: *mut HBITMAP,
    ) -> Gdiplus_Status {
        Gdiplus_Bitmap_GetHBITMAP(self, colorBackground, hbmReturn)
    }
    #[inline]
    pub unsafe fn GetHICON(&mut self, hicon: *mut HICON) -> Gdiplus_Status {
        Gdiplus_Bitmap_GetHICON(self, hicon)
    }
    #[inline]
    pub unsafe fn new(filename: *const WCHAR, useEmbeddedColorManagement: BOOL) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap(
            __bindgen_tmp.as_mut_ptr(),
            filename,
            useEmbeddedColorManagement,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(stream: *mut IStream, useEmbeddedColorManagement: BOOL) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap1(
            __bindgen_tmp.as_mut_ptr(),
            stream,
            useEmbeddedColorManagement,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(
        width: INT,
        height: INT,
        stride: INT,
        format: Gdiplus_PixelFormat,
        scan0: *mut BYTE,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap2(
            __bindgen_tmp.as_mut_ptr(),
            width,
            height,
            stride,
            format,
            scan0,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(width: INT, height: INT, format: Gdiplus_PixelFormat) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap3(__bindgen_tmp.as_mut_ptr(), width, height, format);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new4(width: INT, height: INT, target: *mut Gdiplus_Graphics) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap4(__bindgen_tmp.as_mut_ptr(), width, height, target);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new5(surface: *mut IDirectDrawSurface7) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap5(__bindgen_tmp.as_mut_ptr(), surface);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new6(gdiBitmapInfo: *const BITMAPINFO, gdiBitmapData: *mut c_void) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap6(__bindgen_tmp.as_mut_ptr(), gdiBitmapInfo, gdiBitmapData);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new7(hbm: HBITMAP, hpal: HPALETTE) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap7(__bindgen_tmp.as_mut_ptr(), hbm, hpal);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new8(hicon: HICON) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap8(__bindgen_tmp.as_mut_ptr(), hicon);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new9(hInstance: HINSTANCE, bitmapName: *const WCHAR) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap9(__bindgen_tmp.as_mut_ptr(), hInstance, bitmapName);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new10(nativeBitmap: *mut Gdiplus_GpBitmap) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Bitmap_Bitmap10(__bindgen_tmp.as_mut_ptr(), nativeBitmap);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
pub struct Gdiplus_CustomLineCap__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_CustomLineCap {
    pub vtable_: *const Gdiplus_CustomLineCap__bindgen_vtable,
    pub nativeCap: *mut Gdiplus_GpCustomLineCap,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?Clone@CustomLineCap@Gdiplus@@QEBAPEAV12@XZ"]
    pub fn Gdiplus_CustomLineCap_Clone(
        this: *const Gdiplus_CustomLineCap,
    ) -> *mut Gdiplus_CustomLineCap;
}
extern "C" {
    #[link_name = "\u{1}?SetStrokeCaps@CustomLineCap@Gdiplus@@QEAA?AW4Status@2@W4LineCap@2@0@Z"]
    pub fn Gdiplus_CustomLineCap_SetStrokeCaps(
        this: *mut Gdiplus_CustomLineCap,
        startCap: Gdiplus_LineCap,
        endCap: Gdiplus_LineCap,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetStrokeCaps@CustomLineCap@Gdiplus@@QEBA?AW4Status@2@PEAW4LineCap@2@0@Z"]
    pub fn Gdiplus_CustomLineCap_GetStrokeCaps(
        this: *const Gdiplus_CustomLineCap,
        startCap: *mut Gdiplus_LineCap,
        endCap: *mut Gdiplus_LineCap,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetStrokeJoin@CustomLineCap@Gdiplus@@QEAA?AW4Status@2@W4LineJoin@2@@Z"]
    pub fn Gdiplus_CustomLineCap_SetStrokeJoin(
        this: *mut Gdiplus_CustomLineCap,
        lineJoin: Gdiplus_LineJoin,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetStrokeJoin@CustomLineCap@Gdiplus@@QEBA?AW4LineJoin@2@XZ"]
    pub fn Gdiplus_CustomLineCap_GetStrokeJoin(
        this: *const Gdiplus_CustomLineCap,
    ) -> Gdiplus_LineJoin;
}
extern "C" {
    #[link_name = "\u{1}?SetBaseCap@CustomLineCap@Gdiplus@@QEAA?AW4Status@2@W4LineCap@2@@Z"]
    pub fn Gdiplus_CustomLineCap_SetBaseCap(
        this: *mut Gdiplus_CustomLineCap,
        baseCap: Gdiplus_LineCap,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBaseCap@CustomLineCap@Gdiplus@@QEBA?AW4LineCap@2@XZ"]
    pub fn Gdiplus_CustomLineCap_GetBaseCap(this: *const Gdiplus_CustomLineCap) -> Gdiplus_LineCap;
}
extern "C" {
    #[link_name = "\u{1}?SetBaseInset@CustomLineCap@Gdiplus@@QEAA?AW4Status@2@M@Z"]
    pub fn Gdiplus_CustomLineCap_SetBaseInset(
        this: *mut Gdiplus_CustomLineCap,
        inset: Gdiplus_REAL,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBaseInset@CustomLineCap@Gdiplus@@QEBAMXZ"]
    pub fn Gdiplus_CustomLineCap_GetBaseInset(this: *const Gdiplus_CustomLineCap) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?SetWidthScale@CustomLineCap@Gdiplus@@QEAA?AW4Status@2@M@Z"]
    pub fn Gdiplus_CustomLineCap_SetWidthScale(
        this: *mut Gdiplus_CustomLineCap,
        widthScale: Gdiplus_REAL,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetWidthScale@CustomLineCap@Gdiplus@@QEBAMXZ"]
    pub fn Gdiplus_CustomLineCap_GetWidthScale(this: *const Gdiplus_CustomLineCap) -> Gdiplus_REAL;
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@CustomLineCap@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_CustomLineCap_GetLastStatus(
        this: *const Gdiplus_CustomLineCap,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0CustomLineCap@Gdiplus@@QEAA@PEBVGraphicsPath@1@0W4LineCap@1@M@Z"]
    pub fn Gdiplus_CustomLineCap_CustomLineCap(
        this: *mut Gdiplus_CustomLineCap,
        fillPath: *const Gdiplus_GraphicsPath,
        strokePath: *const Gdiplus_GraphicsPath,
        baseCap: Gdiplus_LineCap,
        baseInset: Gdiplus_REAL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0CustomLineCap@Gdiplus@@IEAA@XZ"]
    pub fn Gdiplus_CustomLineCap_CustomLineCap1(this: *mut Gdiplus_CustomLineCap);
}
impl Gdiplus_CustomLineCap {
    #[inline]
    pub unsafe fn Clone(&self) -> *mut Gdiplus_CustomLineCap {
        Gdiplus_CustomLineCap_Clone(self)
    }
    #[inline]
    pub unsafe fn SetStrokeCaps(
        &mut self,
        startCap: Gdiplus_LineCap,
        endCap: Gdiplus_LineCap,
    ) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_SetStrokeCaps(self, startCap, endCap)
    }
    #[inline]
    pub unsafe fn GetStrokeCaps(
        &self,
        startCap: *mut Gdiplus_LineCap,
        endCap: *mut Gdiplus_LineCap,
    ) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_GetStrokeCaps(self, startCap, endCap)
    }
    #[inline]
    pub unsafe fn SetStrokeJoin(&mut self, lineJoin: Gdiplus_LineJoin) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_SetStrokeJoin(self, lineJoin)
    }
    #[inline]
    pub unsafe fn GetStrokeJoin(&self) -> Gdiplus_LineJoin {
        Gdiplus_CustomLineCap_GetStrokeJoin(self)
    }
    #[inline]
    pub unsafe fn SetBaseCap(&mut self, baseCap: Gdiplus_LineCap) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_SetBaseCap(self, baseCap)
    }
    #[inline]
    pub unsafe fn GetBaseCap(&self) -> Gdiplus_LineCap {
        Gdiplus_CustomLineCap_GetBaseCap(self)
    }
    #[inline]
    pub unsafe fn SetBaseInset(&mut self, inset: Gdiplus_REAL) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_SetBaseInset(self, inset)
    }
    #[inline]
    pub unsafe fn GetBaseInset(&self) -> Gdiplus_REAL {
        Gdiplus_CustomLineCap_GetBaseInset(self)
    }
    #[inline]
    pub unsafe fn SetWidthScale(&mut self, widthScale: Gdiplus_REAL) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_SetWidthScale(self, widthScale)
    }
    #[inline]
    pub unsafe fn GetWidthScale(&self) -> Gdiplus_REAL {
        Gdiplus_CustomLineCap_GetWidthScale(self)
    }
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_CustomLineCap_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn new(
        fillPath: *const Gdiplus_GraphicsPath,
        strokePath: *const Gdiplus_GraphicsPath,
        baseCap: Gdiplus_LineCap,
        baseInset: Gdiplus_REAL,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_CustomLineCap_CustomLineCap(
            __bindgen_tmp.as_mut_ptr(),
            fillPath,
            strokePath,
            baseCap,
            baseInset,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1() -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_CustomLineCap_CustomLineCap1(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DCustomLineCap@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_CustomLineCap_CustomLineCap_destructor(this: *mut Gdiplus_CustomLineCap);
}
#[repr(C)]
pub struct Gdiplus_CachedBitmap__bindgen_vtable(c_void);
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_CachedBitmap {
    pub vtable_: *const Gdiplus_CachedBitmap__bindgen_vtable,
    pub nativeCachedBitmap: *mut Gdiplus_GpCachedBitmap,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GetLastStatus@CachedBitmap@Gdiplus@@QEBA?AW4Status@2@XZ"]
    pub fn Gdiplus_CachedBitmap_GetLastStatus(this: *const Gdiplus_CachedBitmap) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}??0CachedBitmap@Gdiplus@@QEAA@PEAVBitmap@1@PEAVGraphics@1@@Z"]
    pub fn Gdiplus_CachedBitmap_CachedBitmap(
        this: *mut Gdiplus_CachedBitmap,
        bitmap: *mut Gdiplus_Bitmap,
        graphics: *mut Gdiplus_Graphics,
    );
}
impl Gdiplus_CachedBitmap {
    #[inline]
    pub unsafe fn GetLastStatus(&self) -> Gdiplus_Status {
        Gdiplus_CachedBitmap_GetLastStatus(self)
    }
    #[inline]
    pub unsafe fn new(bitmap: *mut Gdiplus_Bitmap, graphics: *mut Gdiplus_Graphics) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_CachedBitmap_CachedBitmap(__bindgen_tmp.as_mut_ptr(), bitmap, graphics);
        __bindgen_tmp.assume_init()
    }
}
extern "C" {
    #[link_name = "\u{1}??_DCachedBitmap@Gdiplus@@QEAAXXZ"]
    pub fn Gdiplus_CachedBitmap_CachedBitmap_destructor(this: *mut Gdiplus_CachedBitmap);
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Metafile {
    pub _base: Gdiplus_Image,
}
extern "C" {
    #[link_name = "\u{1}?GetMetafileHeader@Metafile@Gdiplus@@SA?AW4Status@2@PEAUHMETAFILE__@@PEBUWmfPlaceableFileHeader@2@PEAVMetafileHeader@2@@Z"]
    pub fn Gdiplus_Metafile_GetMetafileHeader(
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetMetafileHeader@Metafile@Gdiplus@@SA?AW4Status@2@PEAUHENHMETAFILE__@@PEAVMetafileHeader@2@@Z"]
    pub fn Gdiplus_Metafile_GetMetafileHeader1(
        hEmf: HENHMETAFILE,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetMetafileHeader@Metafile@Gdiplus@@SA?AW4Status@2@PEB_WPEAVMetafileHeader@2@@Z"]
    pub fn Gdiplus_Metafile_GetMetafileHeader2(
        filename: *const WCHAR,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetMetafileHeader@Metafile@Gdiplus@@SA?AW4Status@2@PEAUIStream@@PEAVMetafileHeader@2@@Z"]
    pub fn Gdiplus_Metafile_GetMetafileHeader3(
        stream: *mut IStream,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetMetafileHeader@Metafile@Gdiplus@@QEBA?AW4Status@2@PEAVMetafileHeader@2@@Z"]
    pub fn Gdiplus_Metafile_GetMetafileHeader4(
        this: *const Gdiplus_Metafile,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetHENHMETAFILE@Metafile@Gdiplus@@QEAAPEAUHENHMETAFILE__@@XZ"]
    pub fn Gdiplus_Metafile_GetHENHMETAFILE(this: *mut Gdiplus_Metafile) -> HENHMETAFILE;
}
extern "C" {
    #[link_name = "\u{1}?PlayRecord@Metafile@Gdiplus@@QEBA?AW4Status@2@W4EmfPlusRecordType@2@IIPEBE@Z"]
    pub fn Gdiplus_Metafile_PlayRecord(
        this: *const Gdiplus_Metafile,
        recordType: Gdiplus_EmfPlusRecordType,
        flags: UINT,
        dataSize: UINT,
        data: *const BYTE,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?SetDownLevelRasterizationLimit@Metafile@Gdiplus@@QEAA?AW4Status@2@I@Z"]
    pub fn Gdiplus_Metafile_SetDownLevelRasterizationLimit(
        this: *mut Gdiplus_Metafile,
        metafileRasterizationLimitDpi: UINT,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetDownLevelRasterizationLimit@Metafile@Gdiplus@@QEBAIXZ"]
    pub fn Gdiplus_Metafile_GetDownLevelRasterizationLimit(this: *const Gdiplus_Metafile) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}?EmfToWmfBits@Metafile@Gdiplus@@SAIPEAUHENHMETAFILE__@@IPEAEHH@Z"]
    pub fn Gdiplus_Metafile_EmfToWmfBits(
        hemf: HENHMETAFILE,
        cbData16: UINT,
        pData16: LPBYTE,
        iMapMode: INT,
        eFlags: INT,
    ) -> UINT;
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUHMETAFILE__@@PEBUWmfPlaceableFileHeader@1@H@Z"]
    pub fn Gdiplus_Metafile_Metafile(
        this: *mut Gdiplus_Metafile,
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        deleteWmf: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUHENHMETAFILE__@@H@Z"]
    pub fn Gdiplus_Metafile_Metafile1(
        this: *mut Gdiplus_Metafile,
        hEmf: HENHMETAFILE,
        deleteEmf: BOOL,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile2(this: *mut Gdiplus_Metafile, filename: *const WCHAR);
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEB_WPEBUWmfPlaceableFileHeader@1@@Z"]
    pub fn Gdiplus_Metafile_Metafile3(
        this: *mut Gdiplus_Metafile,
        filename: *const WCHAR,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUIStream@@@Z"]
    pub fn Gdiplus_Metafile_Metafile4(this: *mut Gdiplus_Metafile, stream: *mut IStream);
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUHDC__@@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile5(
        this: *mut Gdiplus_Metafile,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUHDC__@@AEBVRectF@1@W4MetafileFrameUnit@1@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile6(
        this: *mut Gdiplus_Metafile,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUHDC__@@AEBVRect@1@W4MetafileFrameUnit@1@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile7(
        this: *mut Gdiplus_Metafile,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEB_WPEAUHDC__@@W4EmfType@1@0@Z"]
    pub fn Gdiplus_Metafile_Metafile8(
        this: *mut Gdiplus_Metafile,
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEB_WPEAUHDC__@@AEBVRectF@1@W4MetafileFrameUnit@1@W4EmfType@1@0@Z"]
    pub fn Gdiplus_Metafile_Metafile9(
        this: *mut Gdiplus_Metafile,
        fileName: *const WCHAR,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEB_WPEAUHDC__@@AEBVRect@1@W4MetafileFrameUnit@1@W4EmfType@1@0@Z"]
    pub fn Gdiplus_Metafile_Metafile10(
        this: *mut Gdiplus_Metafile,
        fileName: *const WCHAR,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUIStream@@PEAUHDC__@@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile11(
        this: *mut Gdiplus_Metafile,
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUIStream@@PEAUHDC__@@AEBVRectF@1@W4MetafileFrameUnit@1@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile12(
        this: *mut Gdiplus_Metafile,
        stream: *mut IStream,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
extern "C" {
    #[link_name = "\u{1}??0Metafile@Gdiplus@@QEAA@PEAUIStream@@PEAUHDC__@@AEBVRect@1@W4MetafileFrameUnit@1@W4EmfType@1@PEB_W@Z"]
    pub fn Gdiplus_Metafile_Metafile13(
        this: *mut Gdiplus_Metafile,
        stream: *mut IStream,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    );
}
impl Gdiplus_Metafile {
    #[inline]
    pub unsafe fn GetMetafileHeader(
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_GetMetafileHeader(hWmf, wmfPlaceableFileHeader, header)
    }
    #[inline]
    pub unsafe fn GetMetafileHeader1(
        hEmf: HENHMETAFILE,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_GetMetafileHeader1(hEmf, header)
    }
    #[inline]
    pub unsafe fn GetMetafileHeader2(
        filename: *const WCHAR,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_GetMetafileHeader2(filename, header)
    }
    #[inline]
    pub unsafe fn GetMetafileHeader3(
        stream: *mut IStream,
        header: *mut Gdiplus_MetafileHeader,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_GetMetafileHeader3(stream, header)
    }
    #[inline]
    pub unsafe fn GetMetafileHeader4(&self, header: *mut Gdiplus_MetafileHeader) -> Gdiplus_Status {
        Gdiplus_Metafile_GetMetafileHeader4(self, header)
    }
    #[inline]
    pub unsafe fn GetHENHMETAFILE(&mut self) -> HENHMETAFILE {
        Gdiplus_Metafile_GetHENHMETAFILE(self)
    }
    #[inline]
    pub unsafe fn PlayRecord(
        &self,
        recordType: Gdiplus_EmfPlusRecordType,
        flags: UINT,
        dataSize: UINT,
        data: *const BYTE,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_PlayRecord(self, recordType, flags, dataSize, data)
    }
    #[inline]
    pub unsafe fn SetDownLevelRasterizationLimit(
        &mut self,
        metafileRasterizationLimitDpi: UINT,
    ) -> Gdiplus_Status {
        Gdiplus_Metafile_SetDownLevelRasterizationLimit(self, metafileRasterizationLimitDpi)
    }
    #[inline]
    pub unsafe fn GetDownLevelRasterizationLimit(&self) -> UINT {
        Gdiplus_Metafile_GetDownLevelRasterizationLimit(self)
    }
    #[inline]
    pub unsafe fn EmfToWmfBits(
        hemf: HENHMETAFILE,
        cbData16: UINT,
        pData16: LPBYTE,
        iMapMode: INT,
        eFlags: INT,
    ) -> UINT {
        Gdiplus_Metafile_EmfToWmfBits(hemf, cbData16, pData16, iMapMode, eFlags)
    }
    #[inline]
    pub unsafe fn new(
        hWmf: HMETAFILE,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
        deleteWmf: BOOL,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile(
            __bindgen_tmp.as_mut_ptr(),
            hWmf,
            wmfPlaceableFileHeader,
            deleteWmf,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new1(hEmf: HENHMETAFILE, deleteEmf: BOOL) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile1(__bindgen_tmp.as_mut_ptr(), hEmf, deleteEmf);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new2(filename: *const WCHAR) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile2(__bindgen_tmp.as_mut_ptr(), filename);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new3(
        filename: *const WCHAR,
        wmfPlaceableFileHeader: *const Gdiplus_WmfPlaceableFileHeader,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile3(__bindgen_tmp.as_mut_ptr(), filename, wmfPlaceableFileHeader);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new4(stream: *mut IStream) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile4(__bindgen_tmp.as_mut_ptr(), stream);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new5(
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile5(__bindgen_tmp.as_mut_ptr(), referenceHdc, type_, description);
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new6(
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile6(
            __bindgen_tmp.as_mut_ptr(),
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new7(
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile7(
            __bindgen_tmp.as_mut_ptr(),
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new8(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile8(
            __bindgen_tmp.as_mut_ptr(),
            fileName,
            referenceHdc,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new9(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile9(
            __bindgen_tmp.as_mut_ptr(),
            fileName,
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new10(
        fileName: *const WCHAR,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile10(
            __bindgen_tmp.as_mut_ptr(),
            fileName,
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new11(
        stream: *mut IStream,
        referenceHdc: HDC,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile11(
            __bindgen_tmp.as_mut_ptr(),
            stream,
            referenceHdc,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new12(
        stream: *mut IStream,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_RectF,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile12(
            __bindgen_tmp.as_mut_ptr(),
            stream,
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
    #[inline]
    pub unsafe fn new13(
        stream: *mut IStream,
        referenceHdc: HDC,
        frameRect: *const Gdiplus_Rect,
        frameUnit: Gdiplus_MetafileFrameUnit,
        type_: Gdiplus_EmfType,
        description: *const WCHAR,
    ) -> Self {
        let mut __bindgen_tmp = MaybeUninit::uninit();
        Gdiplus_Metafile_Metafile13(
            __bindgen_tmp.as_mut_ptr(),
            stream,
            referenceHdc,
            frameRect,
            frameUnit,
            type_,
            description,
        );
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Matrix {
    pub nativeMatrix: *mut Gdiplus_GpMatrix,
    pub lastResult: Gdiplus_Status,
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Pen {
    pub nativePen: *mut Gdiplus_GpPen,
    pub lastResult: Gdiplus_Status,
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_StringFormat {
    pub nativeFormat: *mut Gdiplus_GpStringFormat,
    pub lastError: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GenericDefault@StringFormat@Gdiplus@@SAPEBV12@XZ"]
    pub fn Gdiplus_StringFormat_GenericDefault() -> *const Gdiplus_StringFormat;
}
extern "C" {
    #[link_name = "\u{1}?GenericTypographic@StringFormat@Gdiplus@@SAPEBV12@XZ"]
    pub fn Gdiplus_StringFormat_GenericTypographic() -> *const Gdiplus_StringFormat;
}
impl Gdiplus_StringFormat {
    #[inline]
    pub unsafe fn GenericDefault() -> *const Gdiplus_StringFormat {
        Gdiplus_StringFormat_GenericDefault()
    }
    #[inline]
    pub unsafe fn GenericTypographic() -> *const Gdiplus_StringFormat {
        Gdiplus_StringFormat_GenericTypographic()
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_GraphicsPath {
    pub nativePath: *mut Gdiplus_GpPath,
    pub lastResult: Gdiplus_Status,
}
extern "C" {
    #[link_name = "\u{1}?GetBounds@GraphicsPath@Gdiplus@@QEBA?AW4Status@2@PEAVRectF@2@PEBVMatrix@2@PEBVPen@2@@Z"]
    pub fn Gdiplus_GraphicsPath_GetBounds(
        this: *const Gdiplus_GraphicsPath,
        bounds: *mut Gdiplus_RectF,
        matrix: *const Gdiplus_Matrix,
        pen: *const Gdiplus_Pen,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?GetBounds@GraphicsPath@Gdiplus@@QEBA?AW4Status@2@PEAVRect@2@PEBVMatrix@2@PEBVPen@2@@Z"]
    pub fn Gdiplus_GraphicsPath_GetBounds1(
        this: *const Gdiplus_GraphicsPath,
        bounds: *mut Gdiplus_Rect,
        matrix: *const Gdiplus_Matrix,
        pen: *const Gdiplus_Pen,
    ) -> Gdiplus_Status;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@GraphicsPath@Gdiplus@@QEBAHMMPEBVGraphics@2@@Z"]
    pub fn Gdiplus_GraphicsPath_IsVisible(
        this: *const Gdiplus_GraphicsPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsVisible@GraphicsPath@Gdiplus@@QEBAHHHPEBVGraphics@2@@Z"]
    pub fn Gdiplus_GraphicsPath_IsVisible1(
        this: *const Gdiplus_GraphicsPath,
        x: INT,
        y: INT,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsOutlineVisible@GraphicsPath@Gdiplus@@QEBAHMMPEBVPen@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_GraphicsPath_IsOutlineVisible(
        this: *const Gdiplus_GraphicsPath,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        pen: *const Gdiplus_Pen,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
extern "C" {
    #[link_name = "\u{1}?IsOutlineVisible@GraphicsPath@Gdiplus@@QEBAHHHPEBVPen@2@PEBVGraphics@2@@Z"]
    pub fn Gdiplus_GraphicsPath_IsOutlineVisible1(
        this: *const Gdiplus_GraphicsPath,
        x: INT,
        y: INT,
        pen: *const Gdiplus_Pen,
        g: *const Gdiplus_Graphics,
    ) -> BOOL;
}
impl Gdiplus_GraphicsPath {
    #[inline]
    pub unsafe fn GetBounds(
        &self,
        bounds: *mut Gdiplus_RectF,
        matrix: *const Gdiplus_Matrix,
        pen: *const Gdiplus_Pen,
    ) -> Gdiplus_Status {
        Gdiplus_GraphicsPath_GetBounds(self, bounds, matrix, pen)
    }
    #[inline]
    pub unsafe fn GetBounds1(
        &self,
        bounds: *mut Gdiplus_Rect,
        matrix: *const Gdiplus_Matrix,
        pen: *const Gdiplus_Pen,
    ) -> Gdiplus_Status {
        Gdiplus_GraphicsPath_GetBounds1(self, bounds, matrix, pen)
    }
    #[inline]
    pub unsafe fn IsVisible(
        &self,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_GraphicsPath_IsVisible(self, x, y, g)
    }
    #[inline]
    pub unsafe fn IsVisible1(&self, x: INT, y: INT, g: *const Gdiplus_Graphics) -> BOOL {
        Gdiplus_GraphicsPath_IsVisible1(self, x, y, g)
    }
    #[inline]
    pub unsafe fn IsOutlineVisible(
        &self,
        x: Gdiplus_REAL,
        y: Gdiplus_REAL,
        pen: *const Gdiplus_Pen,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_GraphicsPath_IsOutlineVisible(self, x, y, pen, g)
    }
    #[inline]
    pub unsafe fn IsOutlineVisible1(
        &self,
        x: INT,
        y: INT,
        pen: *const Gdiplus_Pen,
        g: *const Gdiplus_Graphics,
    ) -> BOOL {
        Gdiplus_GraphicsPath_IsOutlineVisible1(self, x, y, pen, g)
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct Gdiplus_Graphics {
    pub nativeGraphics: *mut Gdiplus_GpGraphics,
    pub lastResult: Gdiplus_Status,
}
