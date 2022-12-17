#include <linux/bpf.h>
#include <linux/pkt_cls.h>
#include <bpf/bpf_helpers.h>

#define MAGIC 66

volatile int nr_set = 0;
volatile int nr_found = 0;

SEC("tc")
int ingress(struct __sk_buff *skb)
{
	skb->priority = MAGIC;
        nr_set++;

        return TC_ACT_PIPE;
}

SEC("tc")
int egress(struct __sk_buff *skb)
{
	if (skb->priority == MAGIC)
 	       nr_found++;

        return TC_ACT_PIPE;
}

char _license[] SEC("license") = "GPL";
