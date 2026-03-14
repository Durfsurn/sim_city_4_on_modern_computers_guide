use std::{
    ops::{AddAssign, Not},
    sync::atomic::{AtomicU8, Ordering},
};

use dominator::{Dom, events};
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::{JsCast, prelude::wasm_bindgen};
use web_sys::HtmlInputElement;

pub use crate::macros::*;

mod handlers;
mod macros;

#[wasm_bindgen(start)]
fn main_js() -> Result<(), wasm_bindgen::JsValue> {
    console_error_panic_hook::set_once();

    dominator::append_dom(&dominator::body(), render());

    Ok(())
}

pub static CHECKBOX_ID: AtomicU8 = AtomicU8::new(0);
thread_local! {
    pub static CHECKED: Mutable<u8> = Mutable::new(0);
}

pub fn checkbox(checked: Mutable<String>) -> ((Mutable<u8>, u8), Dom) {
    let id = CHECKBOX_ID.load(Ordering::SeqCst);
    CHECKBOX_ID.fetch_add(1, Ordering::SeqCst);

    (
        (CHECKED.with(Mutable::clone), id),
        input()
            .style("transform", "scale(1.5)")
            .attr("type", "checkbox")
            .prop_signal(
                "checked",
                checked.signal_ref(|c| {
                    if c.is_empty().not() {
                        CHECKED.with(Mutable::clone).lock_mut().add_assign(1);
                    }
                    c.is_empty().not()
                }),
            )
            .attr_signal(
                "disabled",
                checked.signal_ref(|c| if c.is_empty() { Some("true") } else { None }),
            )
            .attr_signal(
                "disabled",
                CHECKED
                    .with(Mutable::clone)
                    .signal()
                    .map(move |c| c < id)
                    .map(|d| if d { Some("true") } else { None }),
            )
            .into_dom(),
    )
}

pub fn file_upload(
    url: Mutable<String>,
    (at, id): (Mutable<u8>, u8),
    mime: &'static str,
) -> [Dom; 2] {
    let file_input = input()
        .attr("type", "file")
        .attr("accept", mime)
        .class(["m-1", "px-2", "rounded", "shadow-lg", "h-min"])
        .class_signal(
            ["my-auto", "bg-green-300"],
            url.signal_cloned().map(|u| u.is_empty().not()),
        )
        .attr_signal(
            "disabled",
            at.signal()
                .map(move |a| a < id)
                .map(|d| if d { Some("true") } else { None }),
        )
        .visible_signal(at.signal().map(move |a| a >= id))
        .event({
            let url = url.clone();
            move |evt: events::Input| {
                let input = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                if let Some(file) = input.files().and_then(|files| files.get(0)) {
                    let blob_url = web_sys::Url::create_object_url_with_blob(&file).unwrap();
                    url.set(blob_url);
                }
            }
        });

    let image = img().attr_signal("src", url.signal_cloned()).class([
        "max-h-100",
        "m-1",
        "rounded-lg",
        "shadow-lg",
    ]);

    [file_input.into_dom(), image.into_dom()]
}


fn link(name: &str, link: &str) -> [Dom; 3] {
    let url = Mutable::new(String::new());

    let (disabled, cb) = checkbox(url.clone());
    let fi = file_upload(url, disabled, ".jpg,.jpeg,.png");

    [
        div()
            .class(["flex", "space-x-4", "mt-2"])
            .children([
                a().class([
                    "w-[400px]",
                    "my-auto",
                    "text-blue-500",
                    "underline",
                    "whitespace-pre",
                ])
                .attr("href", link)
                .text(&format!("Download {name}"))
                .into_dom(),
                cb,
            ])
            .into_dom(),
        div()
            .class(["flex", "space-x-4"])
            .children([
                span()
                    .class(["w-[400px]", "my-auto", "whitespace-pre"])
                    .text(&format!("Unzip & Install {name}"))
                    .into_dom(),
                checkbox(Mutable::default()).1,
            ])
            .children(fi)
            .into_dom(),
        pre().into_dom(),
    ]
}

fn render() -> Dom {
    let minimum = || {
        div()
            .class("flex-1")
            .child(h2().class(["mt-4", "text-lg", "font-bold"]).text("Minimum Install Mods & Plugins").into_dom())
            .children(
                link("Disable FPS Limits DLL", "https://github.com/caspervg/sc4-disable-fps-limits/releases/download/v0.1.1/SC4DisableFpsLimits_0.1.1.zip")
            )
            .children(
                link("SC4Fix DLL", "https://community.simtropolis.com/files/file/30883-sc4fix-third-party-patches-for-sc4/")
            )
            .children(
                link("CPU Options DLL", "https://community.simtropolis.com/files/file/36120-sc4-cpu-options/")
            )
            .children(
                link("Graphics Options DLL", "https://community.simtropolis.com/files/file/36091-sc4-graphics-options/")
            )
            .children(
                link("Startup Performance Optimization DLL", "https://community.simtropolis.com/files/file/36244-startup-performance-optimization-dll-for-simcity-4/")
            )
            .children(
                link("Region Thumbnail Fix DLL", "https://community.simtropolis.com/files/file/36396-region-thumbnail-fix-dll/")
            )
            .children(
                link("Transparent Texture Fix DLL", "https://community.simtropolis.com/files/file/36379-transparent-texture-water-bug-fix-dll/")
            )
            .children(
                link("4GB Patch", "https://ntcore.com/files/4gb_patch.zip")
            )
            .children(
                link("DgVoodoo 2", "https://community.simtropolis.com/files/file/36227-dgvoodoo-2-simcity-4-edition/")
            ).into_dom()
    };

    let optional = || {
        div()
            .class("flex-1")
            .child(
                h2().class(["mt-4", "text-lg", "font-bold"])
                    .text("Optional Install Mods & Plugins")
                    .into_dom(),
            )
            .children(link(
                "SC4Pac",
                "https://github.com/memo33/sc4pac-gui/releases",
            ))
            .children(link(
                "Auto Save DLL",
                "https://community.simtropolis.com/files/file/35761-sc4-auto-save/",
            ))
            .children(link("ReShade", "https://reshade.me/"))
            .into_dom()
    };

    div()
        .class(["p-5"])
        //
        .child(
            h2().class(["text-lg", "font-bold"])
                .text("Prerequisites")
                .into_dom(),
        )
        .child(handlers::check_windows())
        .child(handlers::simcity_4_exe("Installed SimCity 4"))
        .child(handlers::check_device())
        .child(handlers::check_ssd())
        .child(handlers::check_plugins("Empty Plugins"))
        .child(
            div()
                .class(["flex"])
                .child(minimum())
                .child(optional())
                .into_dom(),
        )
        .into_dom()
}
