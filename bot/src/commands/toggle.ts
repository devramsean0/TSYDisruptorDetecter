import { ApplyOptions } from '@sapphire/decorators';
import {Subcommand} from "@sapphire/plugin-subcommands";
import {ChatInputCommandInteraction, MessageFlagsBitField} from "discord.js";
import {usersTable} from "../db/schema";
import {db} from "../index";

@ApplyOptions<Subcommand.Options>({
	description: 'Toggle Disruptions updates',
	subcommands: [
		{
			name: 'on',
			chatInputRun: 'toggleOn',
			default: true
		},
		{
			name: 'off',
			chatInputRun: 'toggleOff',
		}
	]
})
export class UserCommand extends Subcommand {
	public override registerApplicationCommands(registry: Subcommand.Registry) {
		registry.registerChatInputCommand((builder) =>
			builder //
				.setName(this.name)
				.setDescription(this.description)
				.addSubcommand((command) => command.setName("on").setDescription("Enable notifications"))
				.addSubcommand((command) => command.setName("off").setDescription("Disable notifications"))
		);
	}

	async toggleOn(interaction: ChatInputCommandInteraction) {
		const user: typeof usersTable.$inferInsert = {
			discord_id: interaction.user.id,
			state: 1
		}
		await db
			.insert(usersTable)
			.values([user])
			.onConflictDoUpdate({
				target: usersTable.id,
				set: {
					state: 1
				}
			});
		return interaction.reply({content: "Enabled notifications!", flags: [MessageFlagsBitField.Flags.Ephemeral]});
	}

	async toggleOff(interaction: ChatInputCommandInteraction) {
		const user: typeof usersTable.$inferInsert = {
			discord_id: interaction.user.id,
			state: 0
		}
		await db
			.insert(usersTable)
			.values([user])
			.onConflictDoUpdate({
				target: usersTable.id,
				set: {
					state: 0
				}
			});
		return interaction.reply({content: "Disabled notifications!", flags: [MessageFlagsBitField.Flags.Ephemeral]});
	}
}
