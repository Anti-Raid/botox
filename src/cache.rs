use std::sync::Arc;

use serenity::all::{Cache, CacheHttp, Http};
use serenity::all::{GuildId, UserId};

/// Helper function to check if a member is on a server, returning a boolean
pub async fn member_on_guild(
    cache_http: impl CacheHttp,
    guild_id: GuildId,
    user_id: UserId,
    fetch_http: bool,
) -> Result<Option<serenity::all::Member>, crate::Error> {
    if let Some(cache) = cache_http.cache() {
        if let Some(guild) = cache.guild(guild_id) {
            if let Some(member) = guild.members.get(&user_id).cloned() {
                return Ok(Some(member));
            }
        }
    }

    if !fetch_http {
        return Ok(None);
    }

    // If we can't find the member in the cache, we can try fetching it
    Ok(Some(cache_http
        .http()
        .get_member(guild_id, user_id)
        .await?))
}

/// A Simple struct that implements the CacheHttp trait because serenity can't seem to keep this stable
#[derive(Debug, Clone)]
pub struct CacheHttpImpl {
    pub cache: Arc<Cache>,
    pub http: Arc<Http>,
}

impl CacheHttp for CacheHttpImpl {
    fn http(&self) -> &Http {
        &self.http
    }

    fn cache(&self) -> Option<&Arc<Cache>> {
        Some(&self.cache)
    }
}

impl CacheHttpImpl {
    pub fn from_ctx(ctx: &serenity::all::Context) -> Self {
        Self {
            cache: ctx.cache.clone(),
            http: ctx.http.clone(),
        }
    }
}

impl From<(Arc<Cache>, Arc<Http>)> for CacheHttpImpl {
    fn from(c: (Arc<Cache>, Arc<Http>)) -> Self {
        Self {
            cache: c.0,
            http: c.1,
        }
    }
}

impl From<serenity::all::Context> for CacheHttpImpl {
    fn from(c: serenity::all::Context) -> Self {
        Self {
            cache: c.cache,
            http: c.http,
        }
    }
}
