use std::process::Command as SysCommand;

pub fn install_service(app_name: &str) {
    let udev_rule = format!(
        r#"
        ACTION=="add|remove", SUBSYSTEM=="usb", DRIVER=="usb", RUN+="/usr/local/bin/{} connect --config=/usr/local/etc/autoroute.conf"
    "#,
        app_name
    );

    let service_file = format!(
        r#"
        [Unit]
        Description=Initial USB MIDI connect
        [Service]
        ExecStart=/usr/local/bin/{} connect --config=/usr/local/etc/autoroute.conf
        [Install]
        WantedBy=multi-user.target
    "#,
        app_name
    );

    // Write udev rule
    std::fs::write("/etc/udev/rules.d/33-midiusb.rules", udev_rule)
        .expect("Failed to write udev rule");

    // Write systemd service
    std::fs::write("/lib/systemd/system/midi.service", service_file)
        .expect("Failed to write systemd service file");

    // Enable the service and restart udev
    SysCommand::new("systemctl")
        .arg("enable")
        .arg("midi.service")
        .output()
        .expect("Failed to enable service");

    SysCommand::new("udevadm")
        .arg("control")
        .arg("--reload")
        .output()
        .expect("Failed to reload udev");

    SysCommand::new("service")
        .arg("udev")
        .arg("restart")
        .output()
        .expect("Failed to restart udev");

    println!("Service installed successfully for '{}'.", app_name);
}
