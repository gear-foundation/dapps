const ENV = {
  NODE: import.meta.env.VITE_NODE_ADDRESS as string,
  CONTRACT: import.meta.env.VITE_CONTRACT as `0x${string}`,
};

const IDL = `
  constructor {
  New : (name: str, symbol: str, decimals: u8);
};

service Vft {
  Burn : (from: actor_id, value: u256) -> bool;
  GrantAdminRole : (to: actor_id) -> null;
  GrantBurnerRole : (to: actor_id) -> null;
  GrantMinterRole : (to: actor_id) -> null;
  Mint : (to: actor_id, value: u256) -> bool;
  RevokeAdminRole : (from: actor_id) -> null;
  RevokeBurnerRole : (from: actor_id) -> null;
  RevokeMinterRole : (from: actor_id) -> null;
  Approve : (spender: actor_id, value: u256) -> bool;
  Transfer : (to: actor_id, value: u256) -> bool;
  TransferFrom : (from: actor_id, to: actor_id, value: u256) -> bool;
  query Admins : () -> vec actor_id;
  query Burners : () -> vec actor_id;
  query Minters : () -> vec actor_id;
  query Allowance : (owner: actor_id, spender: actor_id) -> u256;
  query BalanceOf : (account: actor_id) -> u256;
  query Decimals : () -> u8;
  query Name : () -> str;
  query Symbol : () -> str;
  query TotalSupply : () -> u256;

  events {
    Minted: struct {
      to: actor_id,
      value: u256,
    };
    Burned: struct {
      from: actor_id,
      value: u256,
    };
    Approval: struct {
      owner: actor_id,
      spender: actor_id,
      value: u256,
    };
    Transfer: struct {
      from: actor_id,
      to: actor_id,
      value: u256,
    };
  }
};
`;

export { ENV, IDL };
