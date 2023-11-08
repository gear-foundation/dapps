import { ACTION, USER } from 'consts';
import { useState } from 'react';
import { useSupplyChainMessage } from './api';

type ProduceValues = { name: string; description: string };
type SaleValues = { item_id: string; price: string };
type ApproveValues = { item_id: string; approve: boolean };
type PurchaseValues = { item_id: string; delivery_time: string };
type ItemIdValue = { item_id: string };

function useSale(role: string) {
  const sendMessage = useSupplyChainMessage();

  return (values: SaleValues, onSuccess: () => void) =>
    sendMessage({ [role]: { PutUpForSale: values } }, { onSuccess });
}

function useApprove(role: string) {
  const sendMessage = useSupplyChainMessage();

  return (values: ApproveValues, onSuccess: () => void) => sendMessage({ [role]: { Approve: values } }, { onSuccess });
}

function useShip(role: string) {
  const sendMessage = useSupplyChainMessage();

  return ({ item_id }: ItemIdValue, onSuccess: () => void) => sendMessage({ [role]: { Ship: item_id } }, { onSuccess });
}

function usePurchase(role: string) {
  const sendMessage = useSupplyChainMessage();

  return (values: PurchaseValues, onSuccess: () => void) =>
    sendMessage({ [role]: { Purchase: values } }, { onSuccess });
}

function useReceive(role: string) {
  const sendMessage = useSupplyChainMessage();

  return ({ item_id }: ItemIdValue, onSuccess: () => void) =>
    sendMessage({ [role]: { Receive: item_id } }, { onSuccess });
}

function useProducerActions() {
  const sendMessage = useSupplyChainMessage();

  const sale = useSale(USER.PRODUCER);
  const approve = useApprove(USER.PRODUCER);
  const ship = useShip(USER.PRODUCER);

  const produce = (values: ProduceValues, onSuccess: () => void) =>
    sendMessage({ [USER.PRODUCER]: { Produce: { token_metadata: values } } }, { onSuccess });

  return { produce, sale, approve, ship };
}

function useDistributorActions() {
  const sendMessage = useSupplyChainMessage();

  const purchase = usePurchase(USER.DISTRIBUTOR);
  const sale = useSale(USER.DISTRIBUTOR);
  const approve = useApprove(USER.DISTRIBUTOR);
  const ship = useShip(USER.DISTRIBUTOR);
  const receive = useReceive(USER.DISTRIBUTOR);

  const process = ({ item_id }: ItemIdValue, onSuccess: () => void) =>
    sendMessage({ [USER.DISTRIBUTOR]: { Process: item_id } }, { onSuccess });
  const pack = ({ item_id }: ItemIdValue, onSuccess: () => void) =>
    sendMessage({ [USER.DISTRIBUTOR]: { Package: item_id } }, { onSuccess });

  return { purchase, process, pack, sale, approve, ship, receive };
}

function useRetailerActions() {
  const purchase = usePurchase(USER.RETAILER);
  const sale = useSale(USER.RETAILER);
  const receive = useReceive(USER.RETAILER);

  return { purchase, receive, sale };
}

function useConsumerActions() {
  const sendMessage = useSupplyChainMessage();

  const purchase = ({ item_id }: ItemIdValue) => sendMessage({ [USER.CONSUMER]: { Purchase: item_id } });

  return { purchase };
}

function useSupplyChainActions() {
  const producer = useProducerActions();
  const distributor = useDistributorActions();
  const retailer = useRetailerActions();
  const consumer = useConsumerActions();

  return { producer, distributor, retailer, consumer };
}

function useSubmit(role: string, action: string) {
  const [itemId, setItemId] = useState('');
  const actions = useSupplyChainActions();

  const resetItem = () => setItemId('');

  const getSubmit = () => {
    let userActions: { [key: string]: (value: any, onSuccess: () => void) => void };

    switch (role) {
      case USER.PRODUCER:
        userActions = actions.producer;
        break;
      case USER.DISTRIBUTOR:
        userActions = actions.distributor;
        break;
      case USER.RETAILER:
        userActions = actions.retailer;
        break;
      default:
        userActions = actions.consumer;
        break;
    }

    switch (action) {
      case ACTION.PRODUCE:
        return userActions.produce;
      case ACTION.SALE:
        return userActions.sale;
      case ACTION.APPROVE:
        return userActions.approve;
      case ACTION.SHIP:
        return userActions.ship;
      case ACTION.PURCHASE:
        return userActions.purchase;
      case ACTION.RECEIVE:
        return userActions.receive;
      case ACTION.PROCESS:
        return userActions.process;
      case ACTION.PACKAGE:
        return userActions.pack;
      default:
        return (values: ItemIdValue) => setItemId(values.item_id);
    }
  };

  return { handleSubmit: getSubmit(), itemId, resetItem };
}

export { useSubmit };
