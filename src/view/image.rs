use std::{fs, sync::LazyLock};

use tinybmp::Bmp;

use super::color;

static SELECTED_BACKGROUND_RAW: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| {
    let raw = fs::read("selected-background.bmp");
    match raw {
        Ok(data) => Some(data),
        Err(e) => {
            log::warn!("Failed to read selected-background.bmp: {}", e);
            None
        }
    }
});

pub static SELECTED_BACKGROUND: LazyLock<Option<Bmp<color::Color>>> =
    LazyLock::new(|| match &*SELECTED_BACKGROUND_RAW {
        Some(raw) => match Bmp::from_slice(raw) {
            Ok(bmp) => Some(bmp),
            Err(e) => {
                log::error!(
                    "Failed to parse selected-background.bmp as BMP image: {:?}",
                    e
                );
                None
            }
        },
        None => None,
    });

static LOGO_CROPPED_RAW: LazyLock<Option<Vec<u8>>> = LazyLock::new(|| {
    let raw = fs::read("logo-cropped.bmp");
    match raw {
        Ok(data) => Some(data),
        Err(e) => {
            log::warn!("Failed to read logo-cropped.bmp: {}", e);
            None
        }
    }
});

pub static LOGO_CROPPED: LazyLock<Option<Bmp<color::Color>>> =
    LazyLock::new(|| match &*LOGO_CROPPED_RAW {
        Some(raw) => match Bmp::from_slice(raw) {
            Ok(bmp) => Some(bmp),
            Err(e) => {
                println!("Failed to parse logo-cropped.bmp as BMP image: {:?}", e);
                None
            }
        },
        None => None,
    });
