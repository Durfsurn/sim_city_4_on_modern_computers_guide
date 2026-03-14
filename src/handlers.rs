use super::*;
use std::{collections::HashMap, ops::Not};

use dominator::{Dom, events};
use futures_signals::signal::{Mutable, SignalExt};
use gloo::console::error;
use js_sys::{Array, Promise};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{FileSystemDirectoryHandle, HtmlInputElement};

pub fn check_windows() -> Dom {
    let valid = Mutable::new(String::new());
    let link = "ms-settings:about";

    let ((at, id), checkbox) = checkbox(valid.clone());

    let file_input = input()
        .attr("type", "text")
        .class(["m-1", "px-2", "rounded", "shadow-lg", "h-min"])
        .attr("placeholder", "Paste Windows Specifications here!")
        .class_signal(
            ["my-auto", "bg-green-300"],
            valid.signal_cloned().map(|u| u.is_empty().not()),
        )
        .attr_signal(
            "disabled",
            at.signal()
                .map(move |a| a < id)
                .map(|d| if d { Some("true") } else { None }),
        )
        .visible_signal(at.signal().map(move |a| a >= id))
        .event({
            let valid = valid.clone();
            move |evt: events::Input| {
                #[allow(deprecated)]
                let val = evt.value().unwrap_or_default();
                let map = val
                    .lines()
                    .filter_map(|l| l.split_once("\t"))
                    .collect::<HashMap<_, _>>();
                if let Some(edition) = map.get("Edition")
                    && edition.contains("Windows 11").not()
                {
                    error!(format!("Edition: {}!", edition));
                    return;
                }
                if let Some(version) = map.get("Version").and_then(|v| v[0..2].parse::<u8>().ok())
                    && version.lt(&21)
                {
                    error!(format!("Version: {}!", version));
                    return;
                }
                valid.set("true".into());
            }
        })
        .into_dom();

    div()
        .class(["flex", "space-x-4"])
        .child(
            a().class([
                "w-[400px]",
                "my-auto",
                "text-blue-500",
                "underline",
                "whitespace-pre",
            ])
            .attr("href", link)
            .text("Running Windows 11")
            .into_dom(),
        )
        .child(checkbox)
        .child(file_input)
        .into_dom()
}

pub fn check_device() -> Dom {
    let gb_ram = Mutable::new(String::new());
    let link = "ms-settings:about";

    let ((at, id), checkbox) = checkbox(gb_ram.clone());

    let file_input = input()
        .attr("type", "text")
        .class(["m-1", "px-2", "rounded", "shadow-lg", "h-min"])
        .attr("placeholder", "Paste Device Specifications here!")
        .class_signal(
            ["my-auto", "bg-green-300"],
            gb_ram.signal_cloned().map(|u| u.is_empty().not()),
        )
        .attr_signal(
            "disabled",
            at.signal()
                .map(move |a| a < id)
                .map(|d| if d { Some("true") } else { None }),
        )
        .visible_signal(at.signal().map(move |a| a >= id))
        .event({
            let valid = gb_ram.clone();
            move |evt: events::Input| {
                #[allow(deprecated)]
                let val = evt.value().unwrap_or_default();
                let map = val
                    .lines()
                    .filter_map(|l| l.split_once("\t"))
                    .collect::<HashMap<_, _>>();

                if let Some(ram) = map.get("Installed RAM").and_then(|v| {
                    v.chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<f64>()
                        .ok()
                }) {
                    if ram.lt(&8.0) {
                        error!(format!("Installed RAM: {}!", ram));
                    } else {
                        valid.set(ram.to_string().into());
                    }
                }
            }
        })
        .into_dom();

    div()
        .class(["flex", "space-x-4"])
        .child(
            a().class([
                "w-[400px]",
                "my-auto",
                "text-blue-500",
                "underline",
                "whitespace-pre",
            ])
            .attr("href", link)
            .text_signal(gb_ram.signal_ref(|ram| {
                if ram.is_empty() {
                    "At least 8 GB of RAM".into()
                } else {
                    format!("{ram} GB of RAM")
                }
            }))
            .into_dom(),
        )
        .child(checkbox)
        .child(file_input)
        .into_dom()
}

pub fn simcity_4_exe(name: &'static str) -> Dom {
    let ver_mut = Mutable::new(String::new());

    let ((at, id), checkbox) = checkbox(ver_mut.clone());

    let file_input = input()
        .attr("type", "file")
        .attr("accept", ".exe")
        .class(["m-1", "px-2", "rounded", "shadow-lg", "h-min"])
        .class_signal(
            ["my-auto", "bg-green-300"],
            ver_mut.signal_cloned().map(|u| u.is_empty().not()),
        )
        .attr_signal(
            "disabled",
            at.signal()
                .map(move |a| a < id)
                .map(|d| if d { Some("true") } else { None }),
        )
        .visible_signal(at.signal().map(move |a| a >= id))
        .event({
            let ver_mut = ver_mut.clone();
            move |evt: events::Input| {
                let input = evt.target().unwrap().unchecked_into::<HtmlInputElement>();
                if let Some(file) = input.files().and_then(|files| files.get(0)) {
                    let handle = Box::new(gloo::file::callbacks::read_as_bytes(
                        &gloo::file::Blob::from(file),
                        {
                            let ver_mut = ver_mut.clone();
                            move |bytes: Result<Vec<u8>, _>| match bytes {
                                Ok(bytes) => {
                                    let version = parse_file_version(&bytes);
                                    if let Some(ver) = version
                                        && ver == "1.1.641.0"
                                    {
                                        ver_mut.set(ver);
                                    };
                                }
                                Err(err) => {
                                    web_sys::console::error_1(
                                        &format!("Error reading file: {:?}", err).into(),
                                    );
                                }
                            }
                        },
                    ));

                    Box::leak(handle);
                }
            }
        })
        .into_dom();

    div()
        .class(["flex", "space-x-4"])
        .child(
            span()
                .class(["w-[400px]", "my-auto", "whitespace-pre"])
                .child(span().text(name).into_dom())
                .child_signal(ver_mut.signal_ref(move |ver| {
                    ver.is_empty().not().then_some(
                        span()
                            .class("text-green-500")
                            .text(&format!(" (version: {ver})"))
                            .into_dom(),
                    )
                }))
                .into_dom(),
        )
        .child(checkbox)
        .child(file_input)
        .into_dom()
}
pub fn check_plugins(name: &'static str) -> Dom {
    let is_empty = Mutable::new(String::new());

    let ((at, id), checkbox) = checkbox(is_empty.clone());

    let file_input = button()
        .class(["m-1", "px-2", "rounded", "shadow-lg", "h-min"])
        .class_signal(
            ["my-auto", "bg-green-300"],
            is_empty.signal_cloned().map(|u| u.is_empty().not()),
        )
        .text("Select Documents SimCity 4 Folder")
        .attr_signal(
            "disabled",
            at.signal()
                .map(move |a| a < id)
                .map(|d| if d { Some("true") } else { None }),
        )
        .visible_signal(at.signal().map(move |a| a >= id))
        .event({
            let is_empty = is_empty.clone();
            move |_: events::Click| {
                let is_empty = is_empty.clone();
                spawn_local(async move {
                    match pick_simcity_folder().await {
                        Ok((_top_level, plugins)) => match plugins {
                            Some(contents) if contents.is_empty() => {
                                is_empty.set("true".into());
                            }
                            Some(_) => error!("Plugins folder not empty!"),
                            None => error!("Plugins folder not found!"),
                        },
                        Err(err) => error!("Error picking folder: {:?}", err),
                    }
                });
            }
        })
        .into_dom();

    div()
        .class(["flex", "space-x-4"])
        .child(
            span()
                .class(["w-[400px]", "my-auto", "whitespace-pre"])
                .text(name)
                .into_dom(),
        )
        .child(checkbox)
        .child(file_input)
        .into_dom()
}

pub fn check_ssd() -> Dom {
    let url = Mutable::new(String::new());

    let (disabled, checkbox) = checkbox(url.clone());
    let fi = file_upload(url, disabled, ".jpg,.jpeg,.png");

    div()
        .class(["flex", "space-x-4"])
        .child(
            div()
                .class(["w-[400px]", "my-auto", "whitespace-pre"])
                .child(span().text("Running on an SSD").into_dom())
                .child(
                    pre()
                        .class(["text-[0.9rem]", "bg-neutral-200", "px-2", "rounded-lg", "w-fit", "text-orange-900", "font-bold"])
                        .text("Open Task Manager -> Performance Tab")
                        .into_dom(),
                )
                .into_dom(),
        )
        .child(checkbox)
        .children(fi)
        .into_dom()
}

fn parse_file_version(bytes: &[u8]) -> Option<String> {
    // 1. Validate MZ header
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return None;
    }

    // 2. Read PE header offset
    let pe_offset =
        u32::from_le_bytes([bytes[0x3C], bytes[0x3D], bytes[0x3E], bytes[0x3F]]) as usize;
    if bytes.len() < pe_offset + 4 || &bytes[pe_offset..pe_offset + 4] != b"PE\0\0" {
        return None;
    }

    // 3. Read NumberOfSections (2 bytes after COFF header)
    let num_sections = u16::from_le_bytes([bytes[pe_offset + 6], bytes[pe_offset + 7]]) as usize;
    let optional_header_size =
        u16::from_le_bytes([bytes[pe_offset + 20], bytes[pe_offset + 21]]) as usize;
    let section_table_start = pe_offset + 24 + optional_header_size;

    // 4. Find the .rsrc section
    for i in 0..num_sections {
        let sec_start = section_table_start + i * 40; // each section header = 40 bytes
        if sec_start + 40 > bytes.len() {
            break;
        }

        let name = &bytes[sec_start..sec_start + 8];
        let sec_name = std::str::from_utf8(name)
            .unwrap_or("")
            .trim_end_matches('\0');
        if sec_name == ".rsrc" {
            let raw_data_ptr = u32::from_le_bytes([
                bytes[sec_start + 20],
                bytes[sec_start + 21],
                bytes[sec_start + 22],
                bytes[sec_start + 23],
            ]) as usize;
            let raw_size = u32::from_le_bytes([
                bytes[sec_start + 16],
                bytes[sec_start + 17],
                bytes[sec_start + 18],
                bytes[sec_start + 19],
            ]) as usize;

            // 5. Scan the .rsrc section for VS_FIXEDFILEINFO signature (0xFEEF04BD)
            let end = std::cmp::min(bytes.len(), raw_data_ptr + raw_size - 16);
            let sig_bytes = 0xFEEF04BDu32.to_le_bytes();
            for pos in raw_data_ptr..end {
                if bytes[pos..pos + 4] == sig_bytes[..] {
                    // VS_FIXEDFILEINFO found
                    let dw_file_version_ms = u32::from_le_bytes([
                        bytes[pos + 8],
                        bytes[pos + 9],
                        bytes[pos + 10],
                        bytes[pos + 11],
                    ]);
                    let dw_file_version_ls = u32::from_le_bytes([
                        bytes[pos + 12],
                        bytes[pos + 13],
                        bytes[pos + 14],
                        bytes[pos + 15],
                    ]);
                    let major = dw_file_version_ms >> 16;
                    let minor = dw_file_version_ms & 0xFFFF;
                    let build = dw_file_version_ls >> 16;
                    let revision = dw_file_version_ls & 0xFFFF;
                    return Some(format!("{}.{}.{}.{}", major, minor, build, revision));
                }
            }
        }
    }

    None
}

async fn pick_simcity_folder() -> Result<(Vec<String>, Option<Vec<String>>), JsValue> {
    async fn async_iter_names(dir: &FileSystemDirectoryHandle) -> Result<Vec<String>, JsValue> {
        let mut names = Vec::new();
        let iter = dir.entries();

        loop {
            let next_promise: Promise = js_sys::Reflect::get(&iter, &"next".into())?
                .dyn_into::<js_sys::Function>()?
                .call0(&iter)?
                .dyn_into()?;

            let next = JsFuture::from(next_promise).await?;
            let done = js_sys::Reflect::get(&next, &"done".into())?
                .as_bool()
                .unwrap_or(false);

            if done {
                break;
            }

            let value = js_sys::Reflect::get(&next, &"value".into())?;
            let pair: Array = value.dyn_into()?;
            let name = pair.get(0).as_string().unwrap_or_default();
            names.push(name);
        }

        Ok(names)
    }

    let picker: Promise = js_sys::eval("window.showDirectoryPicker()")?.dyn_into()?;
    let folder_js = JsFuture::from(picker).await?;
    let folder: FileSystemDirectoryHandle = folder_js.dyn_into()?;

    if folder.name() != "SimCity 4" {
        error!("Selected folder is not 'SimCity 4'");
        return Ok((vec![], None));
    }

    let top_level = async_iter_names(&folder).await?;

    let plugins_contents = if top_level.iter().any(|n| n == "Plugins") {
        let plugins_handle = JsFuture::from(folder.get_directory_handle("Plugins"))
            .await?
            .dyn_into::<FileSystemDirectoryHandle>()?;

        Some(async_iter_names(&plugins_handle).await?)
    } else {
        None
    };

    Ok((top_level, plugins_contents))
}
