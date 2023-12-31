module.exports = class Data1661864644152 {
  name = 'Data1661864644152';

  async up(db) {
    await db.query(
      `CREATE TABLE "nft_contract" ("id" character varying NOT NULL, CONSTRAINT "PK_9191862f0721ccbaf709cf6cbc1" PRIMARY KEY ("id"))`,
    );
    await db.query(
      `CREATE TABLE "bid" ("id" character varying NOT NULL, "price" numeric NOT NULL, "auction_id" character varying, CONSTRAINT "PK_ed405dda320051aca2dcb1a50bb" PRIMARY KEY ("id"))`,
    );
    await db.query(`CREATE INDEX "IDX_9e594e5a61c0f3cb25679f6ba8" ON "bid" ("auction_id") `);
    await db.query(
      `CREATE TABLE "auction" ("id" character varying NOT NULL, "ft_contract" text, "finish_at" numeric NOT NULL, "price" numeric NOT NULL, "is_opened" boolean NOT NULL, "token_id" character varying, CONSTRAINT "PK_9dc876c629273e71646cf6dfa67" PRIMARY KEY ("id"))`,
    );
    await db.query(`CREATE INDEX "IDX_c4ecc3110798e89f9841a4a5b4" ON "auction" ("token_id") `);
    await db.query(
      `CREATE TABLE "transfer" ("id" character varying NOT NULL, "timestamp" numeric NOT NULL, "price" numeric, "token_id" character varying, "from_id" character varying, "to_id" character varying, CONSTRAINT "PK_fd9ddbdd49a17afcbe014401295" PRIMARY KEY ("id"))`,
    );
    await db.query(`CREATE INDEX "IDX_b27b1150b8a7af68424540613c" ON "transfer" ("token_id") `);
    await db.query(`CREATE INDEX "IDX_76bdfed1a7eb27c6d8ecbb7349" ON "transfer" ("from_id") `);
    await db.query(`CREATE INDEX "IDX_0751309c66e97eac9ef1149362" ON "transfer" ("to_id") `);
    await db.query(
      `CREATE TABLE "offer" ("id" character varying NOT NULL, "price" numeric NOT NULL, "accepted" boolean NOT NULL, "cancelled" boolean NOT NULL, "token_id" character varying, "account_id" character varying, CONSTRAINT "PK_57c6ae1abe49201919ef68de900" PRIMARY KEY ("id"))`,
    );
    await db.query(`CREATE INDEX "IDX_dfc2ae0b109f9ed6089118ce92" ON "offer" ("account_id") `);
    await db.query(`CREATE INDEX "IDX_977bbdfffe08d5da04b85adce8" ON "offer" ("token_id", "account_id") `);
    await db.query(
      `CREATE TABLE "token" ("id" character varying NOT NULL, "token_id" text NOT NULL, "name" text NOT NULL, "description" text NOT NULL, "media" text NOT NULL, "reference" text NOT NULL, "is_listed" boolean NOT NULL, "price" numeric, "burnt" boolean NOT NULL, "nft_contract_id" character varying, "owner_id" character varying, "auction_id" character varying, CONSTRAINT "PK_82fae97f905930df5d62a702fc9" PRIMARY KEY ("id"))`,
    );
    await db.query(`CREATE INDEX "IDX_7d80c0ade4d8af19712ecb5ebf" ON "token" ("nft_contract_id") `);
    await db.query(`CREATE INDEX "IDX_77fa31a311c711698a0b944382" ON "token" ("owner_id") `);
    await db.query(`CREATE INDEX "IDX_067ae9b77bdde4e8db6eb860b2" ON "token" ("auction_id") `);
    await db.query(
      `CREATE TABLE "account" ("id" character varying NOT NULL, CONSTRAINT "PK_54115ee388cdb6d86bb4bf5b2ea" PRIMARY KEY ("id"))`,
    );
    await db.query(
      `ALTER TABLE "bid" ADD CONSTRAINT "FK_9e594e5a61c0f3cb25679f6ba8d" FOREIGN KEY ("auction_id") REFERENCES "auction"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "auction" ADD CONSTRAINT "FK_c4ecc3110798e89f9841a4a5b46" FOREIGN KEY ("token_id") REFERENCES "token"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "transfer" ADD CONSTRAINT "FK_b27b1150b8a7af68424540613c7" FOREIGN KEY ("token_id") REFERENCES "token"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "transfer" ADD CONSTRAINT "FK_76bdfed1a7eb27c6d8ecbb73496" FOREIGN KEY ("from_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "transfer" ADD CONSTRAINT "FK_0751309c66e97eac9ef11493623" FOREIGN KEY ("to_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "offer" ADD CONSTRAINT "FK_37580f380749c41a7fef4de8eb7" FOREIGN KEY ("token_id") REFERENCES "token"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "offer" ADD CONSTRAINT "FK_dfc2ae0b109f9ed6089118ce924" FOREIGN KEY ("account_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "token" ADD CONSTRAINT "FK_7d80c0ade4d8af19712ecb5ebf4" FOREIGN KEY ("nft_contract_id") REFERENCES "nft_contract"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "token" ADD CONSTRAINT "FK_77fa31a311c711698a0b9443823" FOREIGN KEY ("owner_id") REFERENCES "account"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
    await db.query(
      `ALTER TABLE "token" ADD CONSTRAINT "FK_067ae9b77bdde4e8db6eb860b2d" FOREIGN KEY ("auction_id") REFERENCES "auction"("id") ON DELETE NO ACTION ON UPDATE NO ACTION`,
    );
  }

  async down(db) {
    await db.query(`DROP TABLE "nft_contract"`);
    await db.query(`DROP TABLE "bid"`);
    await db.query(`DROP INDEX "public"."IDX_9e594e5a61c0f3cb25679f6ba8"`);
    await db.query(`DROP TABLE "auction"`);
    await db.query(`DROP INDEX "public"."IDX_c4ecc3110798e89f9841a4a5b4"`);
    await db.query(`DROP TABLE "transfer"`);
    await db.query(`DROP INDEX "public"."IDX_b27b1150b8a7af68424540613c"`);
    await db.query(`DROP INDEX "public"."IDX_76bdfed1a7eb27c6d8ecbb7349"`);
    await db.query(`DROP INDEX "public"."IDX_0751309c66e97eac9ef1149362"`);
    await db.query(`DROP TABLE "offer"`);
    await db.query(`DROP INDEX "public"."IDX_dfc2ae0b109f9ed6089118ce92"`);
    await db.query(`DROP INDEX "public"."IDX_977bbdfffe08d5da04b85adce8"`);
    await db.query(`DROP TABLE "token"`);
    await db.query(`DROP INDEX "public"."IDX_7d80c0ade4d8af19712ecb5ebf"`);
    await db.query(`DROP INDEX "public"."IDX_77fa31a311c711698a0b944382"`);
    await db.query(`DROP INDEX "public"."IDX_067ae9b77bdde4e8db6eb860b2"`);
    await db.query(`DROP TABLE "account"`);
    await db.query(`ALTER TABLE "bid" DROP CONSTRAINT "FK_9e594e5a61c0f3cb25679f6ba8d"`);
    await db.query(`ALTER TABLE "auction" DROP CONSTRAINT "FK_c4ecc3110798e89f9841a4a5b46"`);
    await db.query(`ALTER TABLE "transfer" DROP CONSTRAINT "FK_b27b1150b8a7af68424540613c7"`);
    await db.query(`ALTER TABLE "transfer" DROP CONSTRAINT "FK_76bdfed1a7eb27c6d8ecbb73496"`);
    await db.query(`ALTER TABLE "transfer" DROP CONSTRAINT "FK_0751309c66e97eac9ef11493623"`);
    await db.query(`ALTER TABLE "offer" DROP CONSTRAINT "FK_37580f380749c41a7fef4de8eb7"`);
    await db.query(`ALTER TABLE "offer" DROP CONSTRAINT "FK_dfc2ae0b109f9ed6089118ce924"`);
    await db.query(`ALTER TABLE "token" DROP CONSTRAINT "FK_7d80c0ade4d8af19712ecb5ebf4"`);
    await db.query(`ALTER TABLE "token" DROP CONSTRAINT "FK_77fa31a311c711698a0b9443823"`);
    await db.query(`ALTER TABLE "token" DROP CONSTRAINT "FK_067ae9b77bdde4e8db6eb860b2d"`);
  }
};
