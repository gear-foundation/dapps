import { isHex } from '@polkadot/util';
import { ProduceForm, ItemInputForm, ItemSwitchForm, ItemForm } from 'components';
import { ACTION, USER } from 'consts';
import { Items } from 'types';

const isValidHex = (value: string) => (!isHex(value) ? 'Address should be hex' : null);
const isExists = (value: string) => (!value ? 'Field is required' : null);

const getForm = (role: string, action: string) => {
  switch (action) {
    case ACTION.PRODUCE:
      return ProduceForm;
    case ACTION.SALE:
      return ItemInputForm;
    case ACTION.APPROVE:
      return ItemSwitchForm;
    case ACTION.PURCHASE:
      return role === USER.CONSUMER ? ItemForm : ItemInputForm;
    default:
      return ItemForm;
  }
};

const getLabel = (action: string) => {
  switch (action) {
    case ACTION.SALE:
      return 'Price';
    case ACTION.PURCHASE:
      return 'Delivery Time';
    default:
      return '';
  }
};

const getName = (action: string) => {
  switch (action) {
    case ACTION.SALE:
      return 'price';
    case ACTION.PURCHASE:
      return 'delivery_time';
    default:
      return '';
  }
};

const getAction = (action: string) => {
  switch (action) {
    case ACTION.SALE:
      return 'sell';
    case ACTION.APPROVE:
      return 'approve';
    case ACTION.SHIP:
      return 'ship';
    case ACTION.PURCHASE:
      return 'purchase';
    case ACTION.RECEIVE:
      return 'receive';
    case ACTION.PROCESS:
      return 'process';
    case ACTION.PACKAGE:
      return 'pack';
    default:
      return 'get info';
  }
};

const getFilteredItems = (items: Items, role: string, action: string) =>
  items
    .filter(([, item]) => {
      const { state, by } = item.state;

      switch (role) {
        case USER.PRODUCER: {
          switch (action) {
            case ACTION.SALE:
              return state === 'Produced';

            case ACTION.APPROVE:
              return state === 'Purchased' && by === USER.DISTRIBUTOR;

            case ACTION.SHIP:
              return state === 'Approved' && by === USER.PRODUCER;

            default:
              return true;
          }
        }

        case USER.DISTRIBUTOR: {
          switch (action) {
            case ACTION.PURCHASE:
              return state === 'ForSale' && by === USER.PRODUCER;

            case ACTION.RECEIVE:
              return state === 'Shipped' && by === USER.PRODUCER;

            case ACTION.PROCESS:
              return state === 'Received' && by === USER.DISTRIBUTOR;

            case ACTION.PACKAGE:
              return state === 'Processed' && by === USER.DISTRIBUTOR;

            case ACTION.SALE:
              return state === 'Packaged' && by === USER.DISTRIBUTOR;

            case ACTION.APPROVE:
              return state === 'Purchased' && by === USER.RETAILER;

            case ACTION.SHIP:
              return state === 'Approved' && by === USER.DISTRIBUTOR;

            default:
              return true;
          }
        }

        case USER.RETAILER: {
          switch (action) {
            case ACTION.PURCHASE:
              return state === 'ForSale' && by === USER.DISTRIBUTOR;

            case ACTION.RECEIVE:
              return state === 'Shipped' && by === USER.DISTRIBUTOR;

            case ACTION.SALE:
              return state === 'Received' && by === USER.RETAILER;

            default:
              return true;
          }
        }

        default: {
          switch (action) {
            case ACTION.PURCHASE:
              return state === 'ForSale' && by === USER.RETAILER;

            default:
              return true;
          }
        }
      }
    })
    .map(([id]) => String(id));

export { isValidHex, isExists, getForm, getLabel, getName, getAction, getFilteredItems };
