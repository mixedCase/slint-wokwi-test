use std::{cell::RefCell, rc::Rc};

use display_interface_spi::SPIInterfaceNoCS;
use esp_idf_hal::{delay::Ets, gpio::OutputPin, prelude::Peripherals, spi, units::FromValueType};
use ili9341::{DisplaySize320x480, Ili9341};

static INITIAL_INSTANT: once_cell::sync::OnceCell<std::time::Instant> =
    once_cell::sync::OnceCell::new();

struct DrawBuffer<'a, Display> {
    display: Display,
    buffer: &'a mut [slint::platform::software_renderer::Rgb565Pixel],
}

impl<DI: display_interface::WriteOnlyDataCommand, RST: OutputPin>
    slint::platform::software_renderer::LineBufferProvider
    for &mut DrawBuffer<'_, ili9341::Ili9341<DI, RST>>
{
    type TargetPixel = slint::platform::software_renderer::Rgb565Pixel;

    fn process_line(
        &mut self,
        line: usize,
        range: core::ops::Range<usize>,
        render_fn: impl FnOnce(&mut [slint::platform::software_renderer::Rgb565Pixel]),
    ) {
        let buffer = &mut self.buffer[range.clone()];
        render_fn(buffer);

        let line = line as u16;
        self.display.draw_raw_iter(0, line, 320, line, buffer.iter().map(|x| x.0)).unwrap();
    }
}

#[derive(Default)]
pub struct EspBackend {
    window: RefCell<Option<Rc<slint::platform::software_renderer::MinimalSoftwareWindow<1>>>>,
}

impl slint::platform::Platform for EspBackend {
    fn create_window_adapter(&self) -> Rc<dyn slint::platform::WindowAdapter> {
        let window = slint::platform::software_renderer::MinimalSoftwareWindow::new();
        self.window.replace(Some(window.clone()));
        window
    }

    fn duration_since_start(&self) -> core::time::Duration {
        let the_beginning = *INITIAL_INSTANT.get_or_init(std::time::Instant::now);
        std::time::Instant::now() - the_beginning
    }

    fn run_event_loop(&self) {
        let peripherals = Peripherals::take().unwrap();
        let spi = peripherals.spi2;

        let cs = peripherals.pins.gpio8;
        let rst = peripherals.pins.gpio9.into_output().unwrap();
        let dc = peripherals.pins.gpio3.into_output().unwrap();
        let mosi = peripherals.pins.gpio7;
        let sclk = peripherals.pins.gpio6;
        let miso = peripherals.pins.gpio2;

        let config = spi::config::Config::new()
            .baudrate(4u32.MHz().into())
            .data_mode(embedded_hal::spi::MODE_0);

        let device = esp_idf_hal::spi::Master::<spi::SPI2, _, _, _, _>::new(
            spi,
            spi::Pins {
                sclk,
                sdo: miso,
                sdi: Some(mosi),
                cs: Some(cs),
            },
            config,
        )
        .unwrap();
        let spi_iface = SPIInterfaceNoCS::new(device, dc);
        let display = Ili9341::new(
            spi_iface,
            rst,
            &mut Ets,
            ili9341::Orientation::Landscape,
            DisplaySize320x480,
        )
        .unwrap();

        let mut buffer_provider = DrawBuffer {
            display,
            buffer: &mut [slint::platform::software_renderer::Rgb565Pixel(0); 320],
        };

        self.window
            .borrow()
            .as_ref()
            .unwrap()
            .set_size(slint::PhysicalSize::new(320, 240));

        loop {
            slint::platform::update_timers_and_animations();

            if let Some(window) = self.window.borrow().clone() {
                window.draw_if_needed(|renderer| {
                    renderer.render_by_line(&mut buffer_provider);
                });
                if window.has_active_animations() {
                    continue;
                }
            }
            // TODO
        }
    }
}
