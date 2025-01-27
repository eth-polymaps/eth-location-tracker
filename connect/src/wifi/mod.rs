use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::{EspEventLoop, System};
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::wifi::EspWifi;
use log::info;
use std::thread::sleep;
use std::time::Duration;

use anyhow::anyhow;
pub struct Wifi<'d> {
    username: String,
    password: String,
    esp_wifi: EspWifi<'d>,
}

impl<'d> Wifi<'d> {
    pub fn new(
        peripherals: Peripherals,
        sys_loop: EspEventLoop<System>,
        nvs: EspNvsPartition<NvsDefault>,
        username: &str,
        password: &str,
    ) -> Wifi<'d> {
        Wifi {
            username: username.to_string(),
            password: password.to_string(),
            esp_wifi: EspWifi::new(peripherals.modem, sys_loop, Some(nvs)).unwrap(),
        }
    }

    pub fn connect(&mut self) -> Result<(), anyhow::Error> {
        let ssid = self
            .username
            .as_str()
            .try_into()
            .map_err(|_| anyhow!("ssid does not fit into String<32> buffer"))?;
        let password = self
            .password
            .as_str()
            .try_into()
            .map_err(|_| anyhow!("password does not fit into String<32> buffer"))?;

        self.esp_wifi
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid,
                password,
                ..Default::default()
            }))?;

        self.esp_wifi.start()?;
        self.esp_wifi.connect()?;

        while !self.esp_wifi.is_connected()? {
            let config = self.esp_wifi.get_configuration()?;
            info!("Waiting for station {:?}", config);
        }

        info!("Should be connected now");
        sleep(Duration::new(2, 0));

        info!("IP info: {:?}", self.esp_wifi.sta_netif().get_ip_info()?);

        info!("IP info: {:?}", self.esp_wifi.sta_netif().get_ip_info()?);

        Ok(())
    }
}

impl Drop for Wifi<'_> {
    fn drop(&mut self) {
        info!("dropping Driver")
    }
}
