use super::*;

mod frequencies;
use frequencies::*;

mod datarates;
use datarates::*;

const US_DBM: i8 = 21;
const DEFAULT_RX2: u32 = 923_300_000;

pub use super::channel::Channel;

#[derive(Default, Clone)]
pub struct US915 {
    pub(crate) plan: FixedChannelPlan<14, US915Region>,
}

impl US915 {
    pub fn set_preferred_join_channels(
        &mut self,
        preferred_channels: &[Channel],
        num_retries: usize,
    ) -> Result<(), fixed_channel_plans::Error> {
        self.plan.set_preferred_join_channels(preferred_channels, num_retries)
    }
}

#[derive(Default, Clone)]
pub(crate) struct US915Region;

impl FixedChannelRegion<14> for US915Region {
    fn datarates() -> &'static [Option<Datarate>; 14] {
        &DATARATES
    }
    fn uplink_channels() -> &'static [u32; 72] {
        &UPLINK_CHANNEL_MAP
    }
    fn downlink_channels() -> &'static [u32; 8] {
        &DOWNLINK_CHANNEL_MAP
    }
    fn get_default_rx2() -> u32 {
        DEFAULT_RX2
    }
    fn get_rx_datarate(tx_datarate: DR, _frame: &Frame, window: &Window) -> Datarate {
        let datarate = match window {
            Window::_1 => {
                // no support for RX1 DR Offset
                match tx_datarate {
                    DR::_0 => DR::_10,
                    DR::_1 => DR::_11,
                    DR::_2 => DR::_12,
                    DR::_3 => DR::_13,
                    DR::_4 => DR::_13,
                    _ => panic!("Invalid TX datarate"),
                }
            }
            Window::_2 => DR::_8,
        };
        DATARATES[datarate as usize].clone().unwrap()
    }
    fn get_dbm() -> i8 {
        US_DBM
    }
}
