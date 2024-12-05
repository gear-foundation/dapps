const getMintDetails = (attributesValue?: { key: string; value: string }[], rarity?: string) => {
  const attributes = attributesValue?.reduce((accumulator, { key, value }) => ({ ...accumulator, [key]: value }), {});

  const jsonContent = JSON.stringify({ attributes, rarity });

  const file = new Blob([jsonContent], { type: 'application/json' });
  return file;
};

export { getMintDetails };
