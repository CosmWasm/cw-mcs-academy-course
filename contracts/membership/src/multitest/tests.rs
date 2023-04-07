use std::collections::HashMap;

use crate::multitest::CodeId as MembershipId;
use cosmwasm_std::{coins, Addr, Decimal};
use cw_multi_test::App;
use distribution::multitest::CodeId as DistributionId;
use proxy::multitest::{CodeId as ProxyId, Contract as ProxyContract};

#[test]
fn adding_member() {
    let mut app = App::default();

    let denom = "star";

    let owner = "owner";
    let members = ["member1", "member2"];
    let candidate = "candidate";

    let distribution_id = DistributionId::store_code(&mut app);
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
            distribution_id,
            &members,
            "Membership",
        )
        .unwrap();

    let proxies: HashMap<String, ProxyContract> = data
        .members
        .into_iter()
        .map(|member| {
            let owner = member.owner_addr;
            let proxy = ProxyContract::from_addr(Addr::unchecked(member.proxy_addr));
            (owner, proxy)
        })
        .collect();

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

#[test]
fn distribution() {
    let denom = "star";

    let owner = "owner";
    let donor = "donor";
    let members = ["member1", "member2"];

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked(donor), coins(22, denom))
            .unwrap();
    });

    let distribution_id = DistributionId::store_code(&mut app);
    let proxy_id = ProxyId::store_code(&mut app);
    let membership_id = MembershipId::store_code(&mut app);

    let (_, data) = membership_id
        .instantiate(
            &mut app,
            owner,
            10,
            denom,
            Decimal::percent(10),
            3600 * 24 * 30,
            2,
            proxy_id,
            distribution_id,
            &members,
            "Membership",
        )
        .unwrap();

    let proxies: HashMap<String, ProxyContract> = data
        .members
        .into_iter()
        .map(|member| {
            let owner = member.owner_addr;
            let proxy = ProxyContract::from_addr(Addr::unchecked(member.proxy_addr));
            (owner, proxy)
        })
        .collect();

    proxies[members[0]]
        .donate(&mut app, donor, &coins(22, denom))
        .unwrap();

    proxies[members[0]]
        .withdraw(&mut app, members[0], None, None)
        .unwrap();

    let donor_funds = app
        .wrap()
        .query_all_balances(Addr::unchecked(donor))
        .unwrap();
    assert_eq!(donor_funds, []);

    let member_funds = app
        .wrap()
        .query_all_balances(Addr::unchecked(members[0]))
        .unwrap();
    assert_eq!(member_funds, coins(12, denom));

    let member_funds = app
        .wrap()
        .query_all_balances(Addr::unchecked(members[1]))
        .unwrap();
    assert_eq!(member_funds, []);

    let withdrawable = proxies[members[0]].withdrawable(&app).unwrap();
    assert_eq!(withdrawable.funds, []);

    let withdrawable = proxies[members[1]].withdrawable(&app).unwrap();
    assert_eq!(withdrawable.funds, coins(10, denom));
}
