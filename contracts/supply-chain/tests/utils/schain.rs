use super::{prelude::*, Action, MetaStateReply};
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};

type ActionSupplyChain<T> = Action<T, SupplyChainEvent>;

pub struct SupplyChain<'a>(InnerProgram<'a>);

impl Program for SupplyChain<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> SupplyChain<'a> {
    pub fn initialize(system: &'a System, ft_program: ActorId, nft_program: ActorId) -> Self {
        Self::initialize_custom(
            system,
            InitSupplyChain {
                producers: [PRODUCER.into()].into(),
                distributors: [DISTRIBUTOR.into()].into(),
                retailers: [RETAILER.into()].into(),

                ft_program,
                nft_program,
            },
        )
        .succeed()
    }

    pub fn initialize_custom(system: &System, config: InitSupplyChain) -> SupplyChainInit {
        let program = InnerProgram::current(system);

        let failed = program.send(FOREIGN_USER, config).main_failed();

        SupplyChainInit(program, failed)
    }

    pub fn meta_state(&self) -> SupplyChainMetaState {
        SupplyChainMetaState(&self.0)
    }

    pub fn produce(&self, from: u64) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::Produce {
                    token_metadata: Default::default(),
                },
            ),
            |item_id| SupplyChainEvent::Produced(item_id.into()),
        )
    }

    pub fn put_up_for_sale_by_producer(
        &self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PutUpForSaleByProducer {
                    item_id: item_id.into(),
                    price,
                },
            ),
            |item_id| SupplyChainEvent::ForSaleByProducer(item_id.into()),
        )
    }

    pub fn purchase_by_distributor(
        &self,
        from: u64,
        item_id: u128,
        delivery_time: u64,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PurchaseByDistributor {
                    item_id: item_id.into(),
                    delivery_time,
                },
            ),
            |item_id| SupplyChainEvent::PurchasedByDistributor(item_id.into()),
        )
    }

    pub fn approve_by_producer(
        &self,
        from: u64,
        item_id: u128,
        approve: bool,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::ApproveByProducer {
                    item_id: item_id.into(),
                    approve,
                },
            ),
            |item_id| SupplyChainEvent::ApprovedByProducer(item_id.into()),
        )
    }

    pub fn ship_by_producer(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0
                .send(from, SupplyChainAction::ShipByProducer(item_id.into())),
            |item_id| SupplyChainEvent::ShippedByProducer(item_id.into()),
        )
    }

    pub fn receive_by_distributor(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::ReceiveByDistributor(item_id.into()),
            ),
            |item_id| SupplyChainEvent::ReceivedByDistributor(item_id.into()),
        )
    }

    pub fn process_by_distributor(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::ProcessByDistributor(item_id.into()),
            ),
            |item_id| SupplyChainEvent::ProcessedByDistributor(item_id.into()),
        )
    }

    pub fn package_by_distributor(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PackageByDistributor(item_id.into()),
            ),
            |item_id| SupplyChainEvent::PackagedByDistributor(item_id.into()),
        )
    }

    pub fn put_up_for_sale_by_distributor(
        &self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PutUpForSaleByDistributor {
                    item_id: item_id.into(),
                    price,
                },
            ),
            |item_id| SupplyChainEvent::ForSaleByDistributor(item_id.into()),
        )
    }

    pub fn purchase_by_retailer(
        &self,
        from: u64,
        item_id: u128,
        delivery_time: u64,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PurchaseByRetailer {
                    item_id: item_id.into(),
                    delivery_time,
                },
            ),
            |item_id| SupplyChainEvent::PurchasedByRetailer(item_id.into()),
        )
    }

    pub fn approve_by_distributor(
        &self,
        from: u64,
        item_id: u128,
        approve: bool,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::ApproveByDistributor {
                    item_id: item_id.into(),
                    approve,
                },
            ),
            |item_id| SupplyChainEvent::ApprovedByDistributor(item_id.into()),
        )
    }

    pub fn ship_by_distributor(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0
                .send(from, SupplyChainAction::ShipByDistributor(item_id.into())),
            |item_id| SupplyChainEvent::ShippedByDistributor(item_id.into()),
        )
    }

    pub fn receive_by_retailer(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0
                .send(from, SupplyChainAction::ReceiveByRetailer(item_id.into())),
            |item_id| SupplyChainEvent::ReceivedByRetailer(item_id.into()),
        )
    }

    pub fn put_up_for_sale_by_retailer(
        &self,
        from: u64,
        item_id: u128,
        price: u128,
    ) -> ActionSupplyChain<u128> {
        Action(
            self.0.send(
                from,
                SupplyChainAction::PutUpForSaleByRetailer {
                    item_id: item_id.into(),
                    price,
                },
            ),
            |item_id| SupplyChainEvent::ForSaleByRetailer(item_id.into()),
        )
    }

    pub fn purchase_by_consumer(&self, from: u64, item_id: u128) -> ActionSupplyChain<u128> {
        Action(
            self.0
                .send(from, SupplyChainAction::PurchaseByConsumer(item_id.into())),
            |item_id| SupplyChainEvent::PurchasedByConsumer(item_id.into()),
        )
    }
}

pub struct SupplyChainInit<'a>(InnerProgram<'a>, bool);

impl<'a> SupplyChainInit<'a> {
    #[track_caller]
    pub fn failed(self) {
        assert!(self.1)
    }

    #[track_caller]
    pub fn succeed(self) -> SupplyChain<'a> {
        assert!(!self.1);
        SupplyChain(self.0)
    }
}

pub struct SupplyChainMetaState<'a>(&'a InnerProgram<'a>);

impl SupplyChainMetaState<'_> {
    pub fn item_price(self, item_id: u128) -> MetaStateReply<u128> {
        MetaStateReply(self.item_info(item_id).0.price)
    }

    pub fn item_info(self, item_id: u128) -> MetaStateReply<ItemInfo> {
        if let SupplyChainStateReply::ItemInfo(reply) = self
            .0
            .meta_state(SupplyChainStateQuery::ItemInfo(item_id.into()))
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn participants(self) -> MetaStateReply<Participants> {
        if let SupplyChainStateReply::Participants(reply) = self
            .0
            .meta_state(SupplyChainStateQuery::Participants)
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn ft_program(self) -> MetaStateReply<ActorId> {
        if let SupplyChainStateReply::FTProgram(reply) =
            self.0.meta_state(SupplyChainStateQuery::FTProgram).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn nft_program(self) -> MetaStateReply<ActorId> {
        if let SupplyChainStateReply::NFTProgram(reply) = self
            .0
            .meta_state(SupplyChainStateQuery::NFTProgram)
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn existing_items(self) -> MetaStateReply<BTreeMap<ItemId, ItemInfo>> {
        if let SupplyChainStateReply::ExistingItems(reply) = self
            .0
            .meta_state(SupplyChainStateQuery::ExistingItems)
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }

    pub fn roles(self, actor_id: u64) -> MetaStateReply<Vec<Role>> {
        if let SupplyChainStateReply::Roles(reply) = self
            .0
            .meta_state(SupplyChainStateQuery::Roles(actor_id.into()))
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!()
        }
    }
}
