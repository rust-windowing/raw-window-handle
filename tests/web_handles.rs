//! Tests to ensure web handle examples work correctly.

#![cfg(target_family = "wasm")]

use core::mem::ManuallyDrop;
use raw_window_handle::{WebCanvasWindowHandle, WebOffscreenCanvasWindowHandle};
use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, OffscreenCanvas};

#[wasm_bindgen_test::wasm_bindgen_test]
#[test]
fn html_canvas_element() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into()
        .unwrap();

    canvas.set_attribute("width", "100").unwrap();
    canvas.set_attribute("height", "100").unwrap();

    // Convert to the raw index and convert to the handle.
    let index = (&canvas).into_abi();
    let handle = WebCanvasWindowHandle::new(index as usize);

    // To get the canvas element back, convert the index back.
    let other_end: ManuallyDrop<HtmlCanvasElement> =
        unsafe { HtmlCanvasElement::ref_from_abi(handle.obj as u32) };
    assert_eq!(&*other_end, &canvas);
}

#[wasm_bindgen_test::wasm_bindgen_test]
#[test]
fn offscreen_canvas() {
    let canvas = OffscreenCanvas::new(100, 100).unwrap();

    // Convert to the raw index and convert to the handle.
    let index = (&canvas).into_abi();
    let handle = WebOffscreenCanvasWindowHandle::new(index as usize);

    // To get the canvas element back, convert the index back.
    let other_end: ManuallyDrop<OffscreenCanvas> =
        unsafe { OffscreenCanvas::ref_from_abi(handle.obj as u32) };
    assert_eq!(&*other_end, &canvas);
}

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
