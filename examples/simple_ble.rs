#![no_std]

use libtock::ble_composer;
use libtock::ble_composer::BlePayload;
use libtock::result::TockResult;
use libtock::simple_ble::BleAdvertisingDriver;
use libtock::timer::Duration;
use libtock::Drivers;
use serde::Serialize;

#[derive(Serialize)]
struct LedCommand {
    pub nr: u8,
    pub st: bool,
}

#[libtock::main]
async fn main() -> TockResult<()> {
    let Drivers {
        mut leds_driver_factory,
        mut ble_advertising_driver,
        mut timer_context,
        ..
    } = libtock::retrieve_drivers()?;

    let leds_driver = leds_driver_factory.init_driver()?;
    let mut driver = timer_context.create_timer_driver();
    let timer_driver = driver.activate()?;

    let led = leds_driver.leds().next().unwrap();

    let uuid: [u8; 2] = [0x00, 0x18];

    let payload = corepack::to_bytes(LedCommand { nr: 2, st: true }).unwrap();

    let mut buffer = BleAdvertisingDriver::create_advertising_buffer();
    let mut gap_payload = BlePayload::default();

    gap_payload
        .add_flag(ble_composer::flags::LE_GENERAL_DISCOVERABLE)
        .unwrap();

    gap_payload
        .add(ble_composer::gap_types::UUID, &uuid)
        .unwrap();

    gap_payload
        .add(ble_composer::gap_types::COMPLETE_LOCAL_NAME, b"Tock!")
        .unwrap();

    gap_payload.add_service_payload([91, 79], &payload).unwrap();

    let _handle = ble_advertising_driver.initialize(100, &gap_payload, &mut buffer);

    loop {
        led.on()?;
        timer_driver.sleep(Duration::from_ms(500)).await?;
        led.off()?;
        timer_driver.sleep(Duration::from_ms(500)).await?;
    }
}
