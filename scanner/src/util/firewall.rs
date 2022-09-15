use std::net::Ipv4Addr;

extern crate pfctl;
extern crate iptables;

// constants
static ANCHOR_NAME: &'static str = "pfctl-testing";
static ipt = iptables::new(false).unwrap();

pub fn clear_pfctl_chain() {
    pfctl::PfCtl::new()
        .unwrap()
        .try_remove_anchor(ANCHOR_NAME, pfctl::AnchorKind::Filter)
        .unwrap();
}

pub fn set_pfctl_rule() {
    let mut pf = pfctl::PfCtl::new().unwrap();
    pf.try_enable().unwrap();
    pf.try_add_anchor(ANCHOR_NAME, pfctl::AnchorKind::Filter).unwrap();

    let rule = pfctl::FilterRuleBuilder::default()
        .action(pfctl::FilterRuleAction::Drop(pfctl::DropAction::Drop))
        .proto(pfctl::Proto::Tcp)
        .from(Ipv4Addr::new(192, 168, 0, 69))
        .build()
        .unwrap();

    pf.add_rule(ANCHOR_NAME, &rule).unwrap();
}

pub fn create_iptables_chain() {
    ipt.new_chain("lan", "RUSTYURL");
}

pub fn clear_iptables_chain() {
    ipt.flush_chain("lan", "RUSTYURL");
}

pub fn set_iptables_rule() {
    ipt.append("nat", "RUSTYURL", "-j DROP");
}
