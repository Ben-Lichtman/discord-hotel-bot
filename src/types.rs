use anyhow::{Context as _, Error};
use poise::serenity_prelude::{ChannelId, GuildId, RoleId};
use shuttle_secrets::SecretStore;

// Custom user data passed to all command functions
pub struct Data {
	pub discord_role_everyone: RoleId,
	pub discord_role_hotel_member: RoleId,
	pub discord_category_rooms: ChannelId,
	pub discord_guild: GuildId,
	pub discord_channel_alerts: ChannelId,
}

impl Data {
	pub fn new(secret_store: &SecretStore) -> Self {
		Self {
			discord_role_everyone: secret_store
				.get("DISCORD_ROLE_EVERYONE")
				.context("Failed to get 'DISCORD_ROLE_EVERYONE' from the secret store")
				.expect("Failed to get 'DISCORD_ROLE_EVERYONE' from the secret store")
				.parse()
				.context("Failed to parse 'DISCORD_ROLE_EVERYONE' as RoleId")
				.expect("Failed to parse 'DISCORD_ROLE_EVERYONE' as RoleId"),
			discord_role_hotel_member: secret_store
				.get("DISCORD_ROLE_HOTEL_MEMBER")
				.context("Failed to get 'DISCORD_ROLE_HOTEL_MEMBER' from the secret store")
				.expect("Failed to get 'DISCORD_ROLE_HOTEL_MEMBER' from the secret store")
				.parse()
				.context("Failed to parse 'DISCORD_ROLE_HOTEL_MEMBER' as RoleId")
				.expect("Failed to parse 'DISCORD_ROLE_HOTEL_MEMBER' as RoleId"),
			discord_category_rooms: secret_store
				.get("DISCORD_CATEGORY_ROOMS")
				.context("Failed to get 'DISCORD_CATEGORY_ROOMS' from the secret store")
				.expect("Failed to get 'DISCORD_CATEGORY_ROOMS' from the secret store")
				.parse()
				.context("Failed to parse 'DISCORD_CATEGORY_ROOMS' as ChannelId")
				.expect("Failed to parse 'DISCORD_CATEGORY_ROOMS' as ChannelId"),
			discord_guild: secret_store
				.get("DISCORD_GUILD")
				.context("Failed to get 'DISCORD_GUILD' from the secret store")
				.expect("Failed to get 'DISCORD_GUILD' from the secret store")
				.parse()
				.context("Failed to parse 'DISCORD_GUILD' as GuildId")
				.expect("Failed to parse 'DISCORD_GUILD' as GuildId"),
			discord_channel_alerts: secret_store
				.get("DISCORD_CHANNEL_ALERTS")
				.context("Failed to get 'DISCORD_CHANNEL_ALERTS' from the secret store")
				.expect("Failed to get 'DISCORD_CHANNEL_ALERTS' from the secret store")
				.parse()
				.context("Failed to parse 'DISCORD_CHANNEL_ALERTS' as ChannelId")
				.expect("Failed to parse 'DISCORD_CHANNEL_ALERTS' as ChannelId"),
		}
	}
}

pub type Context<'a> = poise::Context<'a, Data, Error>;
