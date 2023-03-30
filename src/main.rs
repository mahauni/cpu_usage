use std::collections::HashMap;

use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt, DiskUsage, CpuExt};

fn main() {
    // test()
    cpu()
}

fn test() {
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    // We display all disks' information:
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    // Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

    println!("=> system:");
    // RAM and swap information:
    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory : {} bytes", sys.used_memory());
    println!("total swap  : {} bytes", sys.total_swap());
    println!("used swap   : {} bytes", sys.used_swap());

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Number of CPUs:
    println!("NB CPUs: {}", sys.cpus().len());

    let mut map: HashMap<&str, DiskUsage> = HashMap::new();
    // Display processes ID, name na disk usage:
    for (_pid, process) in sys.processes() {
        match map.get_mut(process.name()) {
            Some(v) => {
                let p = process.disk_usage();
                v.read_bytes += p.read_bytes;
                v.total_read_bytes += p.total_read_bytes;
                v.written_bytes += p.written_bytes;
            },
            None => {
                let p = process.disk_usage();
                if p.read_bytes != 0 || p.total_written_bytes != 0 || p.written_bytes != 0 {
                    map.insert(process.name(), p);
                }
            }, 
        }
        // println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
    }

    for (k, v) in map {
        println!("process: {} disk usage -> total read bytes: {}, total writen bytes: {}", k, v.total_read_bytes, v.total_written_bytes);
    }
}

fn cpu() {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        for cpu in sys.cpus() {
            println!("{}: {}%", cpu.name(), cpu.cpu_usage());
        }
        // Sleeping for 500 ms to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}