#![no_std]
#![no_main]

use core::panic::PanicInfo;
use limine::request::FramebufferRequest;
use limine::{BaseRevision, RequestsEndMarker, RequestsStartMarker};

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

// Bitmapy znaków (8x8 pikseli)
const BITMAP_H: [u8; 8] = [0x00,0x44,0x44,0x7c,0x44,0x44,0x44,0x00];
const BITMAP_I: [u8; 8] = [0x00,0x7e,0x18,0x18,0x18,0x18,0x7e,0x00];
const BITMAP_E: [u8; 8] = [0x00,0x7e,0x40,0x7c,0x40,0x40,0x7e,0x00];
const BITMAP_L: [u8; 8] = [0x00,0x40,0x40,0x40,0x40,0x40,0x7e,0x00];
const BITMAP_O: [u8; 8] = [0x00,0x3c,0x42,0x42,0x42,0x42,0x3c,0x00];
const BITMAP_W: [u8; 8] = [0x00,0x42,0x42,0x42,0x5a,0x5a,0x24,0x00];
const BITMAP_R: [u8; 8] = [0x00,0x70,0x48,0x48,0x70,0x50,0x4e,0x00];
const BITMAP_D: [u8; 8] = [0x00,0x78,0x44,0x42,0x42,0x44,0x78,0x00];
const BITMAP_EXCLAMATION: [u8; 8] = [0x00,0x18,0x18,0x18,0x18,0x00,0x18,0x00];
const BITMAP_SPACE: [u8; 8] = [0; 8];

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

        draw_char(fb_ptr, stride, 100, 100, BITMAP_H, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 110, 100, BITMAP_E, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 120, 100, BITMAP_L, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 130, 100, BITMAP_L, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 140, 100, BITMAP_O, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 150, 100, BITMAP_SPACE, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 160, 100, BITMAP_W, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 170, 100, BITMAP_O, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 180, 100, BITMAP_R, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 190, 100, BITMAP_L, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 200, 100, BITMAP_D, 0x00FF_FFFF);
        draw_char(fb_ptr, stride, 210, 100, BITMAP_EXCLAMATION, 0x00FF_FFFF);
    }
    system_halt();
}

fn draw_char(
    fb_ptr: *mut u32,
    stride: usize,
    start_x: usize,
    start_y: usize,
    bitmap: [u8; 8],
    color: u32,
) {
    for (y, &row) in bitmap.iter().enumerate() {
        for x in 0..8 {
            if (row & (0x80 >> x)) != 0 {
                let pixel_offset = (start_y + y) * stride + (start_x + x);
                unsafe {
                    *fb_ptr.add(pixel_offset) = color;
                }
            }
        }
    }
}

fn system_halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    system_halt();
}
