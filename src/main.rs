use rumqttc::{AsyncClient, MqttOptions, QoS};
use tokio::time::{self, sleep_until};
use std::time::Duration;
use log;
use tokio_cron_scheduler::{Job, JobScheduler};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ntp_pool: String,
    mqtt_host: String,
    mqtt_port: u16,
    mqtt_username: String,
    mqtt_password: String
}

#[tokio::main]
async fn main() {
    let mut scheduler = JobScheduler::new().await.unwrap();


    scheduler.add(
        Job::new_async("0 */5 * * * *", |_uuid, _locked| {
            Box::pin(async move {
                // Set up the MQTT Broker
                let mut mqttoptions = MqttOptions::new("rumqtt-sync", "starsrv-db01.cascadia.local", 1883);
                mqttoptions.set_keep_alive(Duration::from_secs(5));
                mqttoptions.set_credentials("moon", "intellistar");

                let (mut client, mut event_loop) = AsyncClient::new(mqttoptions, 10);

                // Get the current NTP timestamp from the pool
                let pool = "0.pool.ntp.org:123";
                let response = ntp_client::Client::new()
                    .target(pool).expect("Failed to target NTP server.")
                    .format(Some("%m/%d/%Y %I:%M:%S.%3f %p"))
                    .request().expect("Failed to request NTP time");

                let res_str = response.get_datetime_str().expect("Failed to get datetime as a string.");

                client.publish("wxstar/heartbeat", QoS::AtLeastOnce, false, 
                "{\"cmd\": \"heartbeat(Time=".to_owned() + &res_str + ")\"}")
                .await.unwrap();

                log::info!("Posted heartbeat command to the MQTT broker.");

                while let Ok(notification) = event_loop.poll().await {
                    log::debug!("Notification = {:?}", notification);
                }

                client.disconnect().await.unwrap();
            })
    }).unwrap()
    ).await.unwrap();

    scheduler.start().await.unwrap();

    loop {
        sleep_until(time::Instant::now() + Duration::from_secs(60)).await;
    }
}