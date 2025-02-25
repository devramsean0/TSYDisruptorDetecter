use log::info;
use crate::{Disruption, DisruptionDiff};

pub fn diff(old_disruptions: Vec<Disruption>, new_disruptions: &Vec<Disruption>) -> DisruptionDiff {
    // Calculate New Disruptions
    let mut new: Vec<Disruption> = vec![];
    let mut changed: Vec<Disruption> = vec![];
    let mut removed: Vec<Disruption> = vec![];

    for disruption in new_disruptions {
        let old_disruption = old_disruptions.iter().find(|x| x.id == disruption.id);
        match old_disruption {
            Some(old) => {
                if old.hash != disruption.hash {
                    info!("Disruption: {} has changed", disruption.title);
                    changed.push(disruption.clone());
                }
            },
            None => {
                info!("Disruption: {} is new", disruption.title);
                new.push(disruption.clone());
            }
        }
    };
    for disruption in old_disruptions {
        if !new_disruptions.iter().any(|x| x.id == disruption.id) {
            info!("Disruption: {} has been removed", disruption.title);
            removed.push(disruption.clone());
        }
    };
    DisruptionDiff {
        new,
        changed,
        removed
    }
}

pub fn save_diffs_to_db(diff: DisruptionDiff, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    for disruption in diff.new {
        if !disruption.id.is_none() {
            conn.execute("INSERT INTO disruptions_diffs(id, disruption_id, state)
            VALUES (NULL, ?, ?)
            ", (disruption.id.unwrap().to_string(), 0, ))?;
            info!("Disruption update for: {}, saved to diff", disruption.title);
        }
    }
    for disruption in diff.changed {
        if !disruption.id.is_none() {
            conn.execute("INSERT INTO disruptions_diffs(id, disruption_id, state)
            VALUES (NULL, ?, ?)
            ", (disruption.id.unwrap().to_string(), 1, ))?;
            info!("Disruption update for: {}, saved to diff", disruption.title);
        }
    }
    for disruption in diff.removed {
        // Clear up old disruptions from DB
        if !disruption.id.is_none() {
            conn.execute("DELETE FROM disruptions WHERE id = ?", (disruption.clone().id.unwrap().to_string(), ))?;
            conn.execute("INSERT INTO disruptions_diffs(id, disruption_name, state)
            VALUES (NULL, ?, ?)
            ", (disruption.clone().title, 2, ))?;
            info!("Disruption: {} has been removed from DB", disruption.title);
            info!("Disruption update for: {}, saved to diff", disruption.title);
        }
    }
    Ok(())
}