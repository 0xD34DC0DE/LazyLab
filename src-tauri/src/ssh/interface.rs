use std::net::{IpAddr, Ipv4Addr};
use anyhow::Result;
use serde::Serialize;
use tauri::regex::Regex;
use tauri::utils::debug_eprintln;

use super::command::{CommandString, ConcreteCommand, VirtualCommand};

#[derive(Serialize, Debug)]
pub struct Interface {
    name: String,
    mac: MAC,
    cidr: CIDR,
    status: String,
}

impl Interface {
    pub fn new(name: String, mac: MAC, cidr: CIDR, status: String) -> Self {
        Self { name, mac, cidr, status }
    }
}

#[derive(Serialize, Debug, PartialEq)]
struct MAC([u8; 6]);

impl TryFrom<&str> for MAC {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let parts = value.split(':');
        if parts.clone().count() != 6 {
            return Err(anyhow::anyhow!("Invalid MAC address"));
        }
        let mut bytes = [0; 6];
        for (i, part) in parts.enumerate() {
            bytes[i] = u8::from_str_radix(part, 16)?;
        }
        Ok(Self(bytes))
    }
}

impl std::fmt::Display for MAC {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct CIDR(IpAddr, u8);

impl CIDR {
    pub fn new(ip: IpAddr, mask: Ipv4Addr) -> Self {
        Self(
            ip,
            mask.octets().iter()
                .map(|&b| b.count_ones())
                .reduce(|a, b| a + b)
                .unwrap() as u8,
        )
    }
}

impl std::fmt::Display for CIDR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}

impl TryFrom<&str> for CIDR {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        let parts: Vec<&str> = value.split('/').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid CIDR"));
        }
        let ip = parts[0].parse()?;
        let mask = parts[1].parse()?;
        Ok(Self(ip, mask))
    }
}

pub struct ListInterfaces;

impl ListInterfaces {
    const IMPLEMENTATIONS: [&'static dyn ConcreteCommand<Vec<Interface>>; 2] =
        [&ListInterfacesIp {}, &ListInterfacesIfConfig {}];
}

impl VirtualCommand<Vec<Interface>, 2> for ListInterfaces {
    fn implementations(&self) -> [&'static dyn ConcreteCommand<Vec<Interface>>; 2] {
        Self::IMPLEMENTATIONS
    }
}

struct ListInterfacesIp;

impl ConcreteCommand<Vec<Interface>> for ListInterfacesIp {
    fn detection_command(&self) -> CommandString {
        CommandString::Static("ip -V")
    }

    fn execution_command(&self) -> CommandString {
        CommandString::Static("ip a")
    }

    fn parse_detection_output(&self, output: &str) -> Result<bool> {
        Ok(output.contains("ip utility"))
    }

    fn parse_execution_output(&self, output: &str) -> Result<Vec<Interface>> {
        let name_re = Regex::new(r"^(\w+)[:@]").unwrap();
        let state_re = Regex::new(r"state\s(\w+)").unwrap();
        let mac_re = Regex::new(r"(?m)^\s+\w+/\w+\s([\w:]+)").unwrap();
        let cidr_re = Regex::new(r"inet\s([\d.]+/\d+)|inet6\s([\w:]+/\d+)").unwrap();
        let split_re = Regex::new(r"(?m)^\d+: ").unwrap();

        split_re.split(output).filter(|&s| !s.is_empty()).map(|lines| {
            let name = name_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid interface name in '{}'", lines))?
                .as_str().to_string();

            let state = state_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid interface state in '{}'", lines))?
                .as_str().to_string();

            let mac = mac_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid MAC address in '{}'", lines))?
                .as_str().try_into()
                .map_err(|e| anyhow::anyhow!("Invalid MAC address: {}", e))?;

            let cidr = cidr_re.captures(lines)
                .and_then(|cap| cap.get(1).or(cap.get(2)))
                .ok_or(anyhow::anyhow!("No valid CIDR in '{}'", lines))?
                .as_str().try_into()
                .map_err(|e| anyhow::anyhow!("Invalid CIDR: {}", e))?;

            Ok(Interface::new(name, mac, cidr, state))
        }).collect::<Result<Vec<Interface>>>()
    }
}

struct ListInterfacesIfConfig;

impl ConcreteCommand<Vec<Interface>> for ListInterfacesIfConfig {
    fn detection_command(&self) -> CommandString {
        CommandString::Static("ifconfig -s lo")
    }

    fn execution_command(&self) -> CommandString {
        CommandString::Static("ifconfig")
    }

    fn parse_detection_output(&self, output: &str) -> Result<bool> {
        Ok(output.contains("Iface"))
    }

    fn parse_execution_output(&self, output: &str) -> Result<Vec<Interface>> {
        let name_re = Regex::new(r"^[\w-]+").unwrap();
        let status_re = Regex::new(r"<(\w+),").unwrap();
        let mac_re = Regex::new(r"\s((?:\w\w:){5}\w\w)").unwrap();
        let ip_re = Regex::new(r"inet\s([\d.]+)|inet6\s([\w:]+)").unwrap();
        let mask_re = Regex::new(r"netmask\s([\d.]+)").unwrap();

        output.split("\n\n").filter(|&s| !s.is_empty()).map(|lines| {
            let name = name_re.captures(lines)
                .and_then(|cap| cap.get(0))
                .ok_or(anyhow::anyhow!("No valid interface name in '{}'", lines))?
                .as_str().to_string();

            let status = status_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid interface status in '{}'", lines))?
                .as_str().to_string();

            let mac = mac_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid MAC address in '{}'", lines))?
                .as_str().try_into()
                .map_err(|e| anyhow::anyhow!("Invalid MAC address: {}", e))?;

            let ip = ip_re.captures(lines)
                .and_then(|cap| cap.get(1).or(cap.get(2)))
                .ok_or(anyhow::anyhow!("No valid CIDR in '{}'", lines))?
                .as_str().parse()
                .map_err(|e| anyhow::anyhow!("Invalid CIDR: {}", e))?;

            let mask = mask_re.captures(lines)
                .and_then(|cap| cap.get(1))
                .ok_or(anyhow::anyhow!("No valid mask in '{}'", lines))?
                .as_str().parse()
                .map_err(|e| anyhow::anyhow!("Invalid mask: {}", e))?;

            let cidr = CIDR::new(ip, mask);

            Ok(Interface::new(name, mac, cidr, status))
        }).collect::<Result<Vec<Interface>>>()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::ssh::command::{test::MockCommandExecutor, CommandExecutor};

    use super::*;

    #[derive(Debug)]
    struct MockCommand {
        implementation: usize,
    }

    impl MockCommand {
        const IMPLEMENTATIONS: [&'static dyn ConcreteCommand<Vec<Interface>>; 2] =
            [&ListInterfacesIp {}, &ListInterfacesIfConfig {}];

        pub fn new(implementation: usize) -> Self {
            Self { implementation }
        }
    }

    impl VirtualCommand<Vec<Interface>, 1> for MockCommand {
        fn implementations(&self) -> [&'static dyn ConcreteCommand<Vec<Interface>>; 1] {
            [Self::IMPLEMENTATIONS[self.implementation]]
        }
    }

    #[tokio::test]
    async fn test_list_interfaces_ip() {
        let output = [
            "1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000",
            "     link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00",
            "     inet 127.0.0.1/8 scope host lo",
            "        valid_lft forever preferred_lft forever",
            "     inet6 ::1/128 scope host noprefixroute",
            "        valid_lft forever preferred_lft forever",
            "2: eth0@if12: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc noqueue state UP group default qlen 1000",
            "    link/ether bc:24:11:8c:3e:4b brd ff:ff:ff:ff:ff:ff link-netnsid 0",
            "    inet 192.168.0.3/24 brd 192.168.0.255 scope global eth0",
            "        valid_lft forever preferred_lft forever",
            "    inet6 fe80::be24:11ff:fe8c:3e4b/64 scope link",
            "        valid_lft forever preferred_lft forever"
        ].join("\n");

        let executor = MockCommandExecutor::new(
            HashMap::from_iter(
                [
                    ("ip -V", "ip utility, iproute2-6.1.0, libbpf 1.1.0\n"),
                    ("ip a", output.as_str()),
                ].iter()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            ));
        let command = MockCommand::new(0);
        let interfaces = command.execute(&executor).await;
        assert!(interfaces.is_ok(), "{:?}", interfaces.err());
        let interfaces = interfaces.unwrap();
        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].name, "lo");
        assert_eq!(interfaces[0].mac, MAC::try_from("00:00:00:00:00:00").unwrap());
        assert_eq!(interfaces[0].status, "UNKNOWN");
        assert_eq!(interfaces[1].name, "eth0");
        assert_eq!(interfaces[1].mac, MAC::try_from("bc:24:11:8c:3e:4b").unwrap());
        assert_eq!(interfaces[1].status, "UP");
    }

    #[tokio::test]
    async fn test_list_interfaces_ifconfig() {
        let output2 = [
            "br-2a665e4dbc69: flags=4163<UP,BROADCAST,RUNNING,MULTICAST>  mtu 1500",
            "   inet 172.22.0.1  netmask 255.255.0.0  broadcast 172.22.255.255",
            "   inet6 fe80::42:7cff:fe8b:fb54  prefixlen 64  scopeid 0x20<link>",
            "   ether 02:42:7c:8b:fb:54  txqueuelen 0  (Ethernet)",
            "   RX packets 22315  bytes 12240619 (11.6 MiB)",
            "   RX errors 0  dropped 0  overruns 0  frame 0",
            "   TX packets 22992  bytes 11010560 (10.5 MiB)",
            "   TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0",
            "",
            "br-569c181e089f: flags=4099<UP,BROADCAST,MULTICAST>  mtu 1500",
            "   inet 172.19.0.1  netmask 255.255.0.0  broadcast 172.19.255.255",
            "   ether 02:42:bb:47:77:14  txqueuelen 0  (Ethernet)",
            "   RX packets 0  bytes 0 (0.0 B)",
            "   RX errors 0  dropped 0  overruns 0  frame 0",
            "   TX packets 0  bytes 0 (0.0 B)",
            "   TX errors 0  dropped 0 overruns 0  carrier 0  collisions 0"
        ].join("\n");

        let output1 = [
            "Iface  MTU    RX-OK RX-ERR RX-DRP RX-OVR    TX-OK TX-ERR TX-DRP TX-OVR Flg",
            "lo    65536  0     0     0      0       0     0     0     0     LRU"
        ].join("\n");

        let executor = MockCommandExecutor::new(
            HashMap::from_iter(
                [
                    ("ifconfig -s lo", output1.as_str()),
                    ("ifconfig", output2.as_str())
                ].iter().map(|(k, v)| (k.to_string(), v.to_string()))
            ));

        let command = MockCommand::new(1);
        let interfaces = command.execute(&executor).await.unwrap();
        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].name, "br-2a665e4dbc69");
        assert_eq!(interfaces[0].mac, MAC::try_from("02:42:7c:8b:fb:54").unwrap());
        assert_eq!(interfaces[0].status, "UP");
        assert_eq!(interfaces[1].name, "br-569c181e089f");
        assert_eq!(interfaces[1].mac, MAC::try_from("02:42:bb:47:77:14").unwrap());
        assert_eq!(interfaces[1].status, "UP");
    }
}
