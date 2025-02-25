import './lib/setup';

import { LogLevel, SapphireClient } from '@sapphire/framework';
import { GatewayIntentBits, Partials } from 'discord.js';
import 'dotenv/config';
import { drizzle } from 'drizzle-orm/libsql';

const client = new SapphireClient({
	logger: {
		level: LogLevel.Debug
	},
	shards: 'auto',
	intents: [
		GatewayIntentBits.DirectMessageReactions,
		GatewayIntentBits.DirectMessages,
	],
	partials: [Partials.Channel],
	tasks: {
		bull: {
			connection: {
				host: process.env.REDIS_HOST,
				db: process.env.REDIS_DB
			}
		}
	}
});

export const db = drizzle("file:bot.db");

const main = async () => {
	try {
		client.logger.info('Logging in');
		await client.login();
		client.logger.info('logged in');
	} catch (error) {
		client.logger.fatal(error);
		await client.destroy();
		process.exit(1);
	}
};

void main();
