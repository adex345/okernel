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
        let stride = (fb.pitch / 4) as usize;

        draw_char(fb_ptr, stride, 100, 100, CharBMP::H, Colour::RED);
        draw_char(fb_ptr, stride, 110, 100, CharBMP::I, Colour::GREEN);
    }
    panic!();
}

fn draw_char(
    fb_ptr: *mut u32,
    fb_width_px: usize,
    start_x: usize,
    start_y: usize,
    char_bitmap: [u8; 8],
    color: u32,
) {
    for (y, &row) in char_bitmap.iter().enumerate() {
        // Obliczamy początek wiersza RAZ na całą linię pikseli
        let row_offset = (start_y + y) * fb_width_px + start_x;
            
        for x in 0..8 {
            // Sprawdzamy, czy dany bit jest zapalony
            if (row & (0x80 >> x)) != 0 {
                // Blok unsafe otacza wyłącznie samą operację zapisu piksela
                unsafe {
                    *fb_ptr.add(row_offset + x) = color;
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
