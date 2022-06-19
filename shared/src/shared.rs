use std::time::Duration;

use naia_shared::{LinkConditionerConfig, SharedConfig, SocketConfig};

use crate::channels::{Channels, CHANNEL_CONFIG};

pub fn shared_config() -> SharedConfig<Channels> {
    // Set tick rate to ~60 FPS
    let tick_interval = Some(Duration::from_millis(20));

    let link_condition = Some(LinkConditionerConfig {
        incoming_latency: tick_interval.unwrap().as_millis() as u32 + 10,
        incoming_jitter: 1,
        incoming_loss: 0.0,
    });

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}
