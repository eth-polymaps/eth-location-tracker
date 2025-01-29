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
use positioning::beacon::Room;
use positioning::geographic::Position;

pub struct OLED<'d> {
    display: Ssd1306<
        I2CInterface<I2cDriver<'d>>,
        DisplaySize128x64,
        BufferedGraphicsMode<DisplaySize128x64>,
    >,
}

impl Drop for OLED<'_> {
    fn drop(&mut self) {
        info!("Dropping SSD1306 interface");
    }
}

impl OLED<'_> {
    pub fn lat_lon(&mut self, pos: Position, room: Room) -> anyhow::Result<()> {
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let content = [
            "ETH Indoor Navigation".to_string(),
            "".to_string(),
            format!("{:.6}, {:.6}\n", pos.lat, pos.lon),
            "Room:".to_string(),
            room.identifier(),
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

impl OLED<'_> {
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
