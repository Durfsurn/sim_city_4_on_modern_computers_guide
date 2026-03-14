use std::{
    ops::{AddAssign, Not},
    sync::atomic::{AtomicU8, Ordering},
};

use dominator::{Dom, events};
use futures_signals::signal::{Mutable, SignalExt};
use wasm_bindgen::prelude::wasm_bindgen;

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

fn link(name: &str, link: &str, instr: &str) -> Dom {
    let clicked = Mutable::new(String::new());

    div()
        .class(["flex", "space-x-4", "mt-2"])
        .children([
            div()
                .class(["w-[400px]", "my-auto"])
                .child(
                    a().class(["text-blue-500", "underline", "whitespace-pre"])
                        .attr("href", link)
                        .attr("target", "_blank")
                        .text(&format!("Download {name}"))
                        .event({
                            let clicked = clicked.clone();
                            move |_: events::Click| {
                                clicked.set("true".into());
                            }
                        })
                        .into_dom(),
                )
                .child(
                    pre()
                        .class([
                            "text-[0.9rem]",
                            "bg-neutral-200",
                            "px-2",
                            "rounded-lg",
                            "w-fit",
                            "text-orange-900",
                            "font-bold",
                        ])
                        .text(instr)
                        .into_dom(),
                )
                .into_dom(),
            checkbox(clicked.clone()).1,
        ])
        .into_dom()
}

fn render() -> Dom {
    let minimum_plugins = || {
        div()
            .class("flex-1")
            .child(h2().class(["mt-4", "text-lg", "font-bold"]).text("Minimum Install Plugins").into_dom())
            .child(
                link("Disable FPS Limits DLL", "https://github.com/caspervg/sc4-disable-fps-limits/releases/download/v0.1.1/SC4DisableFpsLimits_0.1.1.zip", "Copy .dll to Plugins folder.")
            )
            .child(
                link("SC4Fix DLL", "https://community.simtropolis.com/files/file/30883-sc4fix-third-party-patches-for-sc4/", "")
            )
            .child(
                link("CPU Options DLL", "https://community.simtropolis.com/files/file/36120-sc4-cpu-options/", "")
            )
            .child(
                link("Graphics Options DLL", "https://community.simtropolis.com/files/file/36091-sc4-graphics-options/", "")
            )
            .child(
                link("Startup Performance Optimization DLL", "https://community.simtropolis.com/files/file/36244-startup-performance-optimization-dll-for-simcity-4/", "")
            )
            .child(
                link("Region Thumbnail Fix DLL", "https://community.simtropolis.com/files/file/36396-region-thumbnail-fix-dll/", "")
            )
            .child(
                link("Transparent Texture Fix DLL", "https://community.simtropolis.com/files/file/36379-transparent-texture-water-bug-fix-dll/", "")
            ).child(handlers::check_document_plugins()).into_dom()
    };
    let minimum_mods =
        || {
            div()
            .class("flex-1")
            .child(
                h2().class(["mt-4", "text-lg", "font-bold"])
                    .text("Minimum Install Mods")
                    .into_dom(),
            )
            .child(link(
                "4GB Patch",
                "https://ntcore.com/files/4gb_patch.zip",
                "",
            ))
            .child(link(
                "DgVoodoo 2",
                "https://community.simtropolis.com/files/file/36227-dgvoodoo-2-simcity-4-edition/",
                "",
            )).child(handlers::check_install_folder()).into_dom()
        };

    let optional = || {
        div()
            .class("flex-1")
            .child(
                h2().class(["mt-4", "text-lg", "font-bold"])
                    .text("Optional Install Mods & Plugins")
                    .into_dom(),
            )
            .child(link(
                "SC4Pac",
                "https://github.com/memo33/sc4pac-gui/releases",
                "",
            ))
            .child(link(
                "Auto Save DLL",
                "https://community.simtropolis.com/files/file/35761-sc4-auto-save/",
                "",
            ))
            .child(link("ReShade", "https://reshade.me/", ""))
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
        .child(
            div()
                .class(["flex", "flex-col", "space-y-1"])
                .child(handlers::check_windows())
                .child(handlers::simcity_4_exe("Installed SimCity 4"))
                .child(handlers::check_device())
                .child(handlers::check_ssd())
                .child(handlers::check_document_plugins_folder_empty(
                    "Empty Plugins",
                ))
                .into_dom(),
        )
        .child(
            div()
                .class(["flex"])
                .child(minimum_plugins())
                .child(minimum_mods())
                .child(optional())
                .into_dom(),
        )
        .into_dom()
}
