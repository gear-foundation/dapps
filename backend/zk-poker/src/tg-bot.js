require("dotenv").config();

const { Telegraf } = require("telegraf");

const APP_URL = process.env.APP_URL;

const bot = new Telegraf(process.env.BOT_TOKEN, {
  telegram: { testEnv: Boolean(process.env.TEST_ENV) },
});

bot.command("start", (ctx) => {
  console.log("someone starts game");
  ctx.telegram.sendGame(ctx.chat.id, "zkPoker");
});

bot.on("callback_query", async (ctx) => {
  try {
    console.log("user started the game", ctx.from.username);
    await ctx.telegram.answerCbQuery(ctx.callbackQuery.id, undefined, {
      url: APP_URL,
    });
  } catch (error) {
    console.error("Error handling callback_query:", error);
  }
});

bot.launch();
console.log("bot started!");
