#!/bin/bash

set -eu

cleanup()
{
    ip netns delete ns1 &> /dev/null || true
    ip netns delete ns2 &> /dev/null || true
}
#trap cleanup EXIT

# Cleanup any leftovers
cleanup

# Create netns and create veth pair
ip netns add ns1
ip netns add ns2
ip link add veth0 netns ns1 type veth peer netns ns2 name veth1

# Assign IPs and routes to veth
ip -netns ns1 addr add 10.90.0.2/24 dev veth0
ip -netns ns1 link set veth0 up
ip -netns ns1 route add default via 10.90.0.2
ip -netns ns2 addr add 10.91.0.2/24 dev veth1
ip -netns ns2 link set veth1 up
ip -netns ns2 route add default via 10.91.0.2

# Create GRE tunnels
ip -netns ns1 tunnel add gre1 mode gre local 10.90.0.2 remote 10.91.0.2 ttl 32
ip -netns ns1 addr add 10.99.0.1/24 dev gre1
ip -netns ns1 link set gre1 up
ip -netns ns2 tunnel add gre1 mode gre local 10.91.0.2 remote 10.90.0.2 ttl 32
ip -netns ns2 addr add 10.99.0.2/24 dev gre1
ip -netns ns2 link set gre1 up

# Verify tunnel is correctly set up
ip netns exec ns1 ping 10.99.0.2 -W 1 -c 1 &>/dev/null
