use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::{Baseline, Text};
use esp_idf_hal::gpio::AnyIOPin;

use esp_idf_hal::i2c::{I2C0, I2c, I2cConfig, I2cDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::units::Hertz;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::{I2CDisplayInterface, Ssd1306, prelude::*};

use embedded_graphics::prelude::*;
use esp_idf_hal::prelude::*;
use log::info;
use positioning::beacon::Output;

pub struct Oled<'d> {
    display: Ssd1306<
        I2CInterface<I2cDriver<'d>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl Drop for Oled<'_> {
    fn drop(&mut self) {
        info!("Dropping SSD1306 interface");
    }
}

impl Oled<'_> {
    pub fn lat_lon(&mut self, output: Output) -> anyhow::Result<()> {
        let pos = output.position;
        let loc = output.location;

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let content = [
            "ETH Indoor (offline)".to_string(),
            "".to_string(),
            format!("{:.6}, {:.6}", pos.lat, pos.lon),
            format!(
                "{:3}m, [{:3}]",
                output.speed.unwrap_or(0f32),
                output.heading.unwrap_or(0i32)
            ),
            "".to_string(),
            format!("Room: {}", loc.identifier()),
        ];

        self.display
            .clear(BinaryColor::Off)
            .map_err(|e| anyhow::anyhow!("Clear error: {:?}", e))?;

        Text::with_baseline(
            content.join("\n").as_str(),
            Point::zero(),
            text_style,
            Baseline::Top,
        )
        .draw(&mut self.display)
        .map_err(|e| anyhow::anyhow!("Txt error: {:?}", e))?;

        self.display
            .flush()
            .map_err(|e| anyhow::anyhow!("Flush error: {:?}", e))?;

        Ok(())
    }
}

impl Oled<'_> {
    pub fn new(i2c0: I2C0, sda: AnyIOPin, sdc: AnyIOPin) -> anyhow::Result<Self> {
        let i2c_master = i2c_master_init(i2c0, sda, sdc, 400.kHz().into())?;

        let interface = I2CDisplayInterface::new(i2c_master);

        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.init().map_err(|e| anyhow::anyhow!("{:?}", e))?;

        Ok(Self { display })
    }
}

fn i2c_master_init<'d>(
    i2c: impl Peripheral<P = impl I2c> + 'd,
    sda: AnyIOPin,
    scl: AnyIOPin,
    baudrate: Hertz,
) -> anyhow::Result<I2cDriver<'d>> {
    let config = I2cConfig::new().baudrate(baudrate);
    let driver = I2cDriver::new(i2c, sda, scl, &config)?;
    Ok(driver)
}
