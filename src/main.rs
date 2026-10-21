#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

use x86_64::instructions::hlt;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

struct CharBMP;

impl CharBMP {
    const H: [u8; 8] = [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x00];
    const I: [u8; 8] = [0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00];
}

struct Colour;

impl Colour {
    const RED: u32 = 0x00FF_0000;
    const GREEN: u32 = 0x0000_FF00;
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel_main();
}

fn kernel_main() -> ! {
    assert!(BASE_REVISION.is_supported());
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response()
        && let Some(fb) = framebuffer_response.framebuffers().first()
    {
        let fb_ptr = fb.address().cast::<u32>();
        let fb_width_px = (fb.pitch / 4) as usize;

		let fb_slice = unsafe {core::slice::from_raw_parts_mut(fb_ptr, 1)};
        draw_char(fb_slice, fb_width_px, 100, 100, CharBMP::H, Colour::RED);
        draw_char(fb_slice, fb_width_px, 110, 100, CharBMP::I, Colour::GREEN);
    }
    panic!();
}

fn draw_char(
    fb_slice: &mut [u32], // Przekazujemy bezpieczny bufor zamiast wskaźnika
    fb_width_px: usize,
    start_x: usize,
    start_y: usize,
    char_bitmap: [u8; 8],
    color: u32,
) {
    for (y, &row) in char_bitmap.iter().enumerate() {
        let row_start = (start_y + y) * fb_width_px + start_x;
        // Pobieramy wycinek tylko dla tego wiersza czcionki (8 pikseli)
        if let Some(pixel_row) = fb_slice.get_mut(row_start..row_start + 8) {
            for x in 0..8 {
                if (row & (0x80 >> x)) != 0 {
                    pixel_row[x] = color; // Bezpieczne i szybkie
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt()
    }
}
