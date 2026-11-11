use super::bluetooth::BluetoothBatteryCollector;
use super::dualsense::DualSenseCollector;
use super::logitech_hidpp::LogitechHidppCollector;
use super::xinput::XInputCollector;
use super::{BatteryCollector, DeviceBattery};
use crate::config::AppConfig;
use crate::notifications::NotificationManager;
use crossbeam_channel::{Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct BatteryEngine {
    update_receiver: Receiver<Vec<DeviceBattery>>,
    refresh_sender: Sender<()>,
    latest_devices: Arc<Mutex<Vec<DeviceBattery>>>,
    _is_running: Arc<AtomicBool>,
}

impl BatteryEngine {
    pub fn start(poll_interval: Duration) -> Self {
        Self::start_with_notifications(poll_interval, false)
    }

    pub fn start_with_notifications(poll_interval: Duration, enable_notifications: bool) -> Self {
        let (update_sender, update_receiver) = crossbeam_channel::bounded(1);
        let (refresh_sender, refresh_receiver) = crossbeam_channel::bounded(1);
        let latest_devices = Arc::new(Mutex::new(Vec::new()));
        let latest_clone = latest_devices.clone();
        let is_running = Arc::new(AtomicBool::new(true));
        let running_clone = is_running.clone();
        let receiver_drain = update_receiver.clone();

        thread::spawn(move || {
            let collectors: Vec<Box<dyn BatteryCollector>> = vec![
                Box::new(LogitechHidppCollector::new()),
                Box::new(XInputCollector::new()),
                Box::new(DualSenseCollector::new()),
                Box::new(BluetoothBatteryCollector::new()),
            ];
            let mut notification_mgr = NotificationManager::new();
            let mut last_cfg = AppConfig::try_load().unwrap_or_default();
            let mut current_interval = poll_interval;

            let poll_and_dispatch =
                |notify_mgr: &mut NotificationManager,
                 cfg_cache: &mut AppConfig,
                 interval_out: &mut Duration| {
                    let devices = Self::collect_all(&collectors);
                    if let Ok(mut guard) = latest_clone.lock() {
                        *guard = devices.clone();
                    }

                    if enable_notifications {
                        if let Some(fresh_cfg) = AppConfig::try_load() {
                            *interval_out =
                                Duration::from_secs(fresh_cfg.poll_interval_secs.max(1));
                            *cfg_cache = fresh_cfg;
                        }
                        notify_mgr.check_and_notify(&devices, cfg_cache);
                    }

                    // Keep channel bounded to 1 latest snapshot even when UI window is SW_HIDE
                    while receiver_drain.try_recv().is_ok() {}
                    let _ = update_sender.try_send(devices);
                };

            // Initial poll immediately
            poll_and_dispatch(&mut notification_mgr, &mut last_cfg, &mut current_interval);

            while running_clone.load(Ordering::Relaxed) {
                let wait_dur = current_interval;
                // Wait for either the timer or a manual refresh signal
                crossbeam_channel::select! {
                    recv(refresh_receiver) -> _ => {
                        poll_and_dispatch(&mut notification_mgr, &mut last_cfg, &mut current_interval);
                    }
                    default(wait_dur) => {
                        poll_and_dispatch(&mut notification_mgr, &mut last_cfg, &mut current_interval);
                    }
                }
            }
        });

        Self {
            update_receiver,
            refresh_sender,
            latest_devices,
            _is_running: is_running,
        }
    }

    fn collect_all(collectors: &[Box<dyn BatteryCollector>]) -> Vec<DeviceBattery> {
        let mut all_devices: Vec<DeviceBattery> = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        for collector in collectors {
            let devices = collector.poll_devices();
            for dev in devices {
                if !seen_ids.contains(&dev.id) {
                    seen_ids.insert(dev.id.clone());
                    all_devices.push(dev);
                }
            }
        }
        all_devices
    }

    #[cfg(test)]
    fn execute_poll(
        collectors: &[Box<dyn BatteryCollector>],
        sender: &Sender<Vec<DeviceBattery>>,
        latest_devices: &Mutex<Vec<DeviceBattery>>,
    ) {
        let all_devices = Self::collect_all(collectors);
        if let Ok(mut guard) = latest_devices.lock() {
            *guard = all_devices.clone();
        }
        let _ = sender.send(all_devices);
    }

    pub fn receiver(&self) -> &Receiver<Vec<DeviceBattery>> {
        &self.update_receiver
    }

    pub fn latest_snapshot(&self) -> Vec<DeviceBattery> {
        self.latest_devices
            .lock()
            .map(|g| g.clone())
            .unwrap_or_default()
    }

    pub fn trigger_refresh(&self) {
        let _ = self.refresh_sender.try_send(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_engine_polls_devices() {
        let collectors: Vec<Box<dyn BatteryCollector>> = vec![
            Box::new(LogitechHidppCollector::new()),
            Box::new(XInputCollector::new()),
            Box::new(DualSenseCollector::new()),
            Box::new(BluetoothBatteryCollector::new()),
        ];
        let (sender, receiver) = crossbeam_channel::unbounded();
        let latest = Mutex::new(Vec::new());
        BatteryEngine::execute_poll(&collectors, &sender, &latest);
        let devices = receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("Failed to poll");
        assert_eq!(*latest.lock().unwrap(), devices);
        println!("=== Detected devices count: {} ===", devices.len());
        for d in &devices {
            println!(
                "Detected Device: {} ({}%) [charging: {}]",
                d.name, d.percentage, d.is_charging
            );
        }
    }
}
