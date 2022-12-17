# skb-prio

### Quickstart

```
$ cargo build
```

Run in one terminal, run:
```
$ sudo ./setup.sh
$ sudo ./target/debug/prio --ifindex-ingress 6 --ifindex-egress 5
```

The ifindexes should be stable, but if not, check that `6` is the
GRE interface (`gre1`) and that `5` is the egress interface (`veth0`).

Run in another terminal:
```
$ sudo ip netns exec ns1 ping 10.99.0.2 -W 1 -c 1
```

This sends a ping over the GRE interface.

In the first window you should see something like:
```
set=0, found=0
set=0, found=0
set=0, found=0
set=1, found=0
set=1, found=0
set=1, found=0
^C%

```

This implies that `skb->priority` is not preserved during encap.

### Details

Basically `setup.sh` creates 2 network namespaces: ns1 and ns2.
It then creates a veth pair between the two netns's. A GRE tunnel
is then built over the two veths.

For the above test, we send a ping over the GRE tunnel from ns1
to ns2. The packet should follow the following path:

1. original packet goes onto `gre1`
2. original packet gets encapped and egressed on `gre1`
3. encapped packet goes onto `veth0`
4. encapped packet gets egressed on `veth0`
