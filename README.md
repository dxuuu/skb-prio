# skb-prio

Quickstart:
```
$ cargo build
```

Run in one terminal, using whatever ifindexes you want:
```
$ sudo ./target/debug/prio --ifindex-ingress 1 --ifindex-egress 1
```

Run in another terminal:
```
$ ping localhost
```

In the first window you should see something like:
```
set=0, found=0
set=0, found=0
set=0, found=0
set=2, found=0
set=4, found=0
set=6, found=0
set=8, found=0
set=10, found=0
set=12, found=0
set=14, found=0
^C%

```
