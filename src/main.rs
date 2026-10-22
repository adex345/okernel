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

// 1. Surowy punkt wejścia ABI dla bootloadera (Zgodny z C)
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Przekazujemy sterowanie do bezpiecznego i czystego świata Rusta
    kernel_main();
}

// 2. Właściwa, główna funkcja Twojego jądra (Czysty Rust)
fn kernel_main() -> ! {
    assert!(BASE_REVISION.is_supported());
    // Cała logika systemu żyje tutaj
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.response()
        && let Some(fb) = framebuffer_response.framebuffers().first()
    {
        // Używamy .cast::<u32>() zamiast rzutowania 'as', aby zachować czystość typów wskaźnikowych
        let fb_ptr = fb.address().cast::<u32>();
        let stride = (fb.pitch / 4) as usize;

        // Rysowanie napisu "HI"
        draw_char(fb_ptr, stride, 100, 100, BITMAP_H, 0x00FF_0000); // Czerwone H
        draw_char(fb_ptr, stride, 110, 100, BITMAP_I, 0x0000_FF00); // Zielone I
    }

    // Bezpieczne zatrzymanie procesora
    system_halt();
}

// Funkcja pomocnicza do rysowania pikseli
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

// Funkcja wprowadzająca procesor w bezpieczny stan uśpienia
fn system_halt() -> ! {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

// Obsługa błędów krytycznych (Panic)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    system_halt();
}
