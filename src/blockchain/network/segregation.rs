#![allow(dead_code)]

use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;

pub fn should_client_be_propagated(client: &str, server: &str) -> bool {
    let client_ip : Ipv4Addr = client.parse().unwrap();
    let server_ip : Ipv4Addr = server.parse().unwrap();

    match client_ip.is_private() || client_ip.is_loopback() {
        true => are_addresses_in_same_network(client_ip, server_ip),
        false => true
    }
}

pub fn get_subnet_mask(ip: Ipv4Addr) -> u8 {
    let mut i = 0;
    let reserved_masks_list: &[u8] = &[8, 12, 16, 32];

    let reserved_ip_ranges: &[Ipv4Network] = &[
        Ipv4Network::new(Ipv4Addr::new(10, 0, 0, 0), reserved_masks_list[0]).unwrap(),
        Ipv4Network::new(Ipv4Addr::new(172, 16, 0, 0), reserved_masks_list[1]).unwrap(),
        Ipv4Network::new(Ipv4Addr::new(192, 168, 0, 0), reserved_masks_list[2]).unwrap()
    ];

    for range in reserved_ip_ranges {
        if range.contains(ip) {
            return reserved_masks_list[i];
        }
        i = i + 1;
    }
    reserved_masks_list[i]
}

pub fn are_addresses_in_same_network(left :Ipv4Addr, right: Ipv4Addr) -> bool {
    let left_subnet = get_subnet_mask(left);
    let right_subnet = get_subnet_mask(right);
    let left_network = Ipv4Network::new(left, left_subnet).unwrap();
    let right_network = Ipv4Network::new(right, right_subnet).unwrap();

    println!("NET : {} - {}", left_subnet, right_subnet);

    left_network.contains(right_network.network())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_true_when_client_and_server_are_global() {
        let client = "8.8.8.8";
        let server = "42.65.97.120";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_true_when_same_network() {
        let client = "10.0.1.3";
        let server = "10.45.65.99";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_true_when_client_is_global_and_server_is_local_network() {
        let client = "8.8.8.8";
        let server = "192.168.1.1";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_client_is_local_network_and_server_is_global() {
        let client = "192.168.0.1";
        let server = "8.8.8.8";
        assert!(!should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_true_when_client_is_global_and_server_is_localhost() {
        let client = "8.8.8.8";
        let server = "127.0.0.1";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_client_is_localhost_and_server_is_global() {
        let client = "127.0.0.1";
        let server = "8.8.8.8";
        assert!(!should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_client_is_local_network_and_server_is_localhost() {
        let client = "192.168.1.1";
        let server = "127.0.0.1";
        assert!(!should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_client_is_localhost_and_server_is_local_network() {
        let client = "127.0.0.1";
        let server = "192.168.1.1";
        assert!(!should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_true_when_same_local_network() {
        let client = "192.168.2.2";
        let server = "192.168.1.1";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_different_local_network() {
        let client = "10.0.0.1";
        let server = "192.168.1.1";
        assert!(!should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_true_when_same_172_network() {
        let client = "172.16.0.1";
        let server = "172.31.254.254";
        assert!(should_client_be_propagated(client, server));
    }

    #[test]
    fn should_return_false_when_different_172_network() {
        let client = "172.16.0.1";
        let server = "172.32.0.1";
        assert!(!should_client_be_propagated(client, server));
    }

}