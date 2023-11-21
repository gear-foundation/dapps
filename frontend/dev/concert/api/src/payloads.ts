export const payloads = {
  init: function (name: string, symbol: string, base_uri: string) {
    return {
      name,
      symbol,
      base_uri,
    };
  },
  create: function (creator: string, name: string, description: string, number_of_tickets: number, date: number) {
    return {
      Create: {
        creator: creator,
        name,
        description,
        number_of_tickets,
        date: date,
      },
    };
  },
  hold: function () {
    return {
      Hold: {},
    };
  },
  buyTickets: function (amount: number, metadata: any[]) {
    return {
      BuyTickets: {
        amount: amount,
        metadata: metadata,
      },
    };
  },
};
