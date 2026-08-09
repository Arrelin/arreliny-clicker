use input_linux::sys::{EV_KEY, input_event};
use std::os::fd::AsRawFd;
use std::time::{Duration, Instant};

use crate::clicker::InputDevice;
use crate::input::key_label;

pub fn run() {
    let mut key = input_linux::sys::BTN_LEFT as u16;
    let mut seconds = 10u64;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--tester" => {}
            "--key" => {
                key = arguments
                    .next()
                    .and_then(|value| value.parse().ok())
                    .expect("--key requires a numeric evdev key code");
            }
            "--seconds" => {
                seconds = arguments
                    .next()
                    .and_then(|value| value.parse().ok())
                    .filter(|value| *value > 0)
                    .expect("--seconds requires a positive number");
            }
            _ => {}
        }
    }

    let devices = InputDevice::devices();
    let mut pollfds: Vec<libc::pollfd> = devices
        .iter()
        .map(|device| libc::pollfd {
            fd: device.handler.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        })
        .collect();
    let mut events: [input_event; 64] = unsafe { std::mem::zeroed() };
    let started = Instant::now();
    let deadline = started + Duration::from_secs(seconds);
    let mut report_at = started + Duration::from_secs(1);
    let mut interval_count = 0u64;
    let mut total_count = 0u64;

    println!("Testing {} for {seconds} seconds", key_label(key));
    println!("Listening on {} input devices", devices.len());

    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let timeout = remaining.min(Duration::from_millis(100));
        let timeout_ms = timeout.as_millis().max(1).min(i32::MAX as u128) as i32;
        let result = unsafe {
            libc::poll(
                pollfds.as_mut_ptr(),
                pollfds.len() as libc::nfds_t,
                timeout_ms,
            )
        };
        if result < 0 {
            break;
        }

        for (index, pollfd) in pollfds.iter_mut().enumerate() {
            if pollfd.revents & libc::POLLIN == 0 {
                continue;
            }
            pollfd.revents = 0;
            if let Ok(length) = devices[index].read(&mut events) {
                for event in &events[..length] {
                    if event.type_ == EV_KEY as u16 && event.code == key && event.value == 1 {
                        interval_count += 1;
                        total_count += 1;
                    }
                }
            }
        }

        let now = Instant::now();
        if now >= report_at {
            let elapsed = now.duration_since(report_at - Duration::from_secs(1));
            println!(
                "{:.0} events/s",
                interval_count as f64 / elapsed.as_secs_f64()
            );
            interval_count = 0;
            report_at = now + Duration::from_secs(1);
        }
    }

    let elapsed = started.elapsed();
    println!("Total: {total_count}");
    println!("Duration: {:.3} s", elapsed.as_secs_f64());
    println!(
        "Average: {:.2} events/s",
        total_count as f64 / elapsed.as_secs_f64()
    );
}
