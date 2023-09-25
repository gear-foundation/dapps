export interface CreateCollectionProps {}

export interface ContractFormValues {
  name: string;
  description: string;
  media: string[];
}

export type DecodedReply = {
  Err?: string;
  CollectionCreated?: {
    collectionAddress: string;
  };
};
