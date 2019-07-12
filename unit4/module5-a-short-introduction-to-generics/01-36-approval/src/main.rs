pub struct Approval<T> {
    item: T,
    approved: bool,
}

impl<T> Approval<T> {
    pub fn new(item: T) -> Approval<T> {
        Approval {
            item,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn replace(self, other_item: T) -> Approval<T> {
        Approval {
            item: other_item,
            approved: self.approved,
        }
    }

    pub fn approved_item(&self) -> Option<&T> {
        if self.approved {
            Some(&self.item)
        } else {
            None
        }
    }
}

use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let amount = 1000;
    let mut approval_amount = Approval::new(amount);
    assert!(approval_amount.approved_item().is_none());
    approval_amount.approve();
    assert_eq!(approval_amount.approved_item(), Some(&1000));

    let mut approval_ip = Approval::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert!(approval_ip.approved_item().is_none());
    approval_ip.approve();
    assert_eq!(
        approval_ip.approved_item(),
        Some(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    );
}
