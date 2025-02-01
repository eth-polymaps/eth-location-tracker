use embedded_svc::wifi::{ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::{EspEventLoop, System};
use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;

use crate::timer;
use anyhow::anyhow;
use esp_idf_hal::modem;

pub struct Wifi<'d> {
    username: String,
    password: String,
    blocking_wifi: BlockingWifi<EspWifi<'d>>,
}

impl<'d> Wifi<'d> {
    pub fn new(
        modem: modem::Modem,
        sys_loop: EspEventLoop<System>,
        nvs: EspNvsPartition<NvsDefault>,
        username: &str,
        password: &str,
    ) -> anyhow::Result<Wifi<'d>> {
        let wifi = EspWifi::new(modem, sys_loop.clone(), Some(nvs))?;
        let blocking_wifi = BlockingWifi::wrap(wifi, sys_loop)?;
        Ok(Wifi {
            username: username.to_string(),
            password: password.to_string(),
            blocking_wifi,
        })
    }

    pub fn connect(&mut self, synchronize_timer: bool) -> Result<(), anyhow::Error> {
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

        self.blocking_wifi
            .set_configuration(&Configuration::Client(ClientConfiguration {
                ssid,
                password,
                ..Default::default()
            }))?;

        self.blocking_wifi.start()?;
        self.blocking_wifi.connect()?;

        info!(
            "IP info: {:?}",
            self.blocking_wifi.wifi().sta_netif().get_ip_info()?
        );

        if synchronize_timer {
            timer::synchronize()?;
        }

        Ok(())
    }
}

impl Drop for Wifi<'_> {
    fn drop(&mut self) {
        info!("dropping Driver")
    }
}
