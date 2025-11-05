type AttributeValue = { key: string; value: string };

type FormValues = {
  name: string;
  description: string;
  attributes: AttributeValue[];
  rarity: string;
};

export type { AttributeValue, FormValues };
