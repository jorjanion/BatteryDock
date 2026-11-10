use super::bluetooth::BluetoothBatteryCollector;
use super::dualsense::DualSenseCollector;
use super::logitech_hidpp::LogitechHidppCollector;
use super::xinput::XInputCollector;
use super::{BatteryCollector, DeviceBattery};
use crossbeam_channel::{Receiver, Sender};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct BatteryEngine {
    update_receiver: Receiver<Vec<DeviceBattery>>,
    refresh_sender: Sender<()>,
    _is_running: Arc<AtomicBool>,
}

impl BatteryEngine {
    pub fn start(poll_interval: Duration) -> Self {
        let (update_sender, update_receiver) = crossbeam_channel::unbounded();
        let (refresh_sender, refresh_receiver) = crossbeam_channel::bounded(1);
        let is_running = Arc::new(AtomicBool::new(true));
        let running_clone = is_running.clone();

        thread::spawn(move || {
            let collectors: Vec<Box<dyn BatteryCollector>> = vec![
                Box::new(LogitechHidppCollector::new()),
                Box::new(XInputCollector::new()),
                Box::new(DualSenseCollector::new()),
                Box::new(BluetoothBatteryCollector::new()),
            ];

            // Initial poll immediately
            Self::execute_poll(&collectors, &update_sender);

            while running_clone.load(Ordering::Relaxed) {
                // Wait for either the timer or a manual refresh signal
                crossbeam_channel::select! {
                    recv(refresh_receiver) -> _ => {
                        Self::execute_poll(&collectors, &update_sender);
                    }
                    default(poll_interval) => {
                        Self::execute_poll(&collectors, &update_sender);
                    }
                }
            }
        });

        Self {
            update_receiver,
            refresh_sender,
            _is_running: is_running,
        }
    }

    fn execute_poll(
        collectors: &[Box<dyn BatteryCollector>],
        sender: &Sender<Vec<DeviceBattery>>,
    ) {
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

        let _ = sender.send(all_devices);
    }

    pub fn receiver(&self) -> &Receiver<Vec<DeviceBattery>> {
        &self.update_receiver
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
        BatteryEngine::execute_poll(&collectors, &sender);
        let devices = receiver.recv_timeout(Duration::from_secs(5)).expect("Failed to poll");
        println!("=== Detected devices count: {} ===", devices.len());
        for d in &devices {
            println!("Detected Device: {} ({}%) [charging: {}]", d.name, d.percentage, d.is_charging);
        }
    }
}
