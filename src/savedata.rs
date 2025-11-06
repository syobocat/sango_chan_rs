// SPDX-FileCopyrightText: 2025 SyoBoN <syobon@syobon.net>
//
// SPDX-License-Identifier: UPL-1.0

use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SaveData {
    nicknames: HashMap<String, String>,
}

impl SaveData {
    pub fn load() -> anyhow::Result<Self> {
        let file = std::fs::read_to_string("savedata.json").context("Failed to load savedata")?;
        let savedata = serde_json::from_str(&file).context("Failed to parse savedata")?;
        Ok(savedata)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let mut file =
            std::fs::File::create("savedata.json").context("Failed to open file for writing")?;
        serde_json::to_writer_pretty(&mut file, self).context("Failed to write savedata")?;
        Ok(())
    }

    pub fn store_nickname(&mut self, id: &str, nick: &str) -> anyhow::Result<()> {
        self.nicknames.insert(id.to_owned(), nick.to_owned());
        self.save()?;
        Ok(())
    }

    pub fn forget_nickname(&mut self, id: &str) -> anyhow::Result<bool> {
        let res = self.nicknames.remove(id);
        self.save()?;
        Ok(res.is_some())
    }

    pub fn get_nickname(&self, id: &str) -> Option<String> {
        self.nicknames.get(id).cloned()
    }
}
