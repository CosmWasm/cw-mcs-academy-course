# Creators donation system

## Actors

* Content Creators
* Donors

* Creator group build from multiple creators

## Use cases

### Donate

User can donate creator

When user donates, part of donation goes to content creator, and the rest goes to this content
creator group

The funds send to the content creators' group are split between its members basing on their
weight

Weight is based on how much donors particular creator brings to the pool

The internal next weight counter is incremented with every donation on behalf of the creator

### Withdrawal

The creator can withdraw funds accumulated on his account

When funds are withdrawn, the creators weight is set to internal next weight, and internal next
weight is reset to 1

### Leave the group

Content creator should be able to leave the group

### Join the group

Anyone can create a join group request

### Accepting the request

Any group member can accept a join request he didn't accept yet

After N acceptances would be given to the request, requesting account is added to the group

After every group member accepts the request, requesting account is added to the group

### Forced weight update

Anyone can force weight update on inactive content creator.

If content creator did not withdraw or was forced to reduce weight for longer than a month,
his weight is halfed.

## Proxy Contract

### Instantiate

```
{
    "onwer": "onwer_addr",
    "weight": 20,
    "denom": "STAR"
}
```

### Execs

#### Donate

```
{
    "donate": {}
}
```

#### Whithdraw

Only by owner

```
{
    "withdraw": {
        "receiver": "receiver_addrs",
        "amount": 10
    }
}
```

#### Close

Only by owner

```
{
    "close": {}
}
```

#### Propose Member

Only by owner
```
{
    "propose_member": {
        "member_addr"
    }
}
```

#### Update weight

```
{}
```

### Quries

#### Owner

```
{
    "owner": {}
}

{
    "addr": "owner_addr",
    "weight": 20
}
```

#### Is Closed

```
{
    "is_closed": {}
}
```

## Distribute

### Instantiate

```
{}
```

### Distribute

```
{
    "distribute": {}
}
```


### Withdraw

```
{
    "weight": 20,
    "diff": -10
}
```

### New Member

```
{
    "new_member": {
        "weight": 20
    }
}
```

## Membership

### Instantiate

This should receive the whole system config

```
{
    "proxy_code_id": 1024,
    "distribute_code_id": 1025
}
```

### Execs

#### Propose

```
{
    "addr": "member_addr"
}
```

### Query

#### Is Member

```
{
    "is_member": {
        "addr": "proxy_addr"
    }
}
```
