use std::collections::HashMap;

use crate::multitest::CodeId as MembershipId;
use cosmwasm_std::Decimal;
use cw_multi_test::App;
use proxy::multitest::{CodeId as ProxyId, Contract as ProxyContract};

#[test]
fn adding_member() {
    let mut app = App::default();

    let denom = "star";

    let owner = "owner";
    let members = ["member1", "member2"];
    let candidate = "candidate";

    let proxy_id = ProxyId::store_code(&mut app);
    let membership_id = MembershipId::store_code(&mut app);

    let (membership, data) = membership_id
        .instantiate(
            &mut app,
            owner,
            10,
            denom,
            Decimal::percent(15),
            3600 * 24 * 30,
            2,
            proxy_id,
            &members,
            "Membership",
        )
        .unwrap();

    let mut proxies: HashMap<String, ProxyContract> = HashMap::new();

    for member in data.members {
        proxies.insert(
            member.owner_addr.into_string(),
            ProxyContract::from_addr(member.proxy_addr),
        );
    }

    assert_eq!(proxies.len(), 2);
    assert!(
        membership
            .is_member(&app, proxies[members[0]].addr().as_str())
            .unwrap()
            .is_member
    );
    assert!(
        membership
            .is_member(&app, proxies[members[1]].addr().as_str())
            .unwrap()
            .is_member
    );

    let data = proxies[members[0]]
        .propose_member(&mut app, members[0], candidate)
        .unwrap();

    assert!(data.is_none());

    let data = proxies[members[1]]
        .propose_member(&mut app, members[1], candidate)
        .unwrap();

    let data = data.unwrap();

    assert_eq!(data.owner_addr, candidate);

    assert!(
        membership
            .is_member(&app, data.proxy_addr.as_str())
            .unwrap()
            .is_member
    );
}
