use super::*;
use chrono::prelude::*;
use std::io::Write;

#[allow(clippy::option_map_unit_fn)]
impl<'api, 'pin> Runner<'api, 'pin> {
    pub fn update_cache(&mut self) {
        match self.update() {
            Ok(s) => io::stdout()
                .write_all(s.as_bytes())
                .expect("Couldn't write to stdout"),
            Err(e) => {
                error!("{}", e.to_string());
                io::stdout()
                    .write_all(e.to_string().as_bytes())
                    .expect("Couldn't write to stdout")
            }
        }
    }

    fn update(&mut self) -> Result<String, Error> {
        info!("Starting in update_cache");
        match self
            .pinboard
            .as_ref()
            .unwrap()
            .is_cache_outdated(self.config.as_ref().unwrap().update_time)
        {
            Err(_) => Err(crate::AlfredError::CacheUpdateFailed(
                "Error: comparing timestamp with server failed".to_string(),
            )
            .into()),
            Ok(needs_update) => {
                if needs_update {
                    debug!("  cache neeeds updating.");
                    self.pinboard
                        .as_mut()
                        .unwrap()
                        .update_cache()
                        .map_err(|_| {
                            crate::AlfredError::CacheUpdateFailed(
                                "Error: update_cache failed".to_string(),
                            )
                        })?;
                    self.config
                        .as_mut()
                        .map(|config| config.update_time = Utc::now());
                    self.config.as_mut().unwrap().save().map_err(|_| {
                        crate::AlfredError::CacheUpdateFailed(
                            "Error: saving update timestamp failed".to_string(),
                        )
                    })?;
                    Ok("Updated cache files!".to_string())
                } else {
                    debug!("  cache is up-to-date.");
                    Ok("Cache is already up-to-date!".to_string())
                }
            }
        }
    }
}
