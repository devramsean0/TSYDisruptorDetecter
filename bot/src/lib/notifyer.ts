import {container} from "@sapphire/framework";
import child_process from "node:child_process";
import util from "node:util";
import sqlite3 from "better-sqlite3";

const exec = util.promisify(child_process.exec);


export async function do_notify() {
    container.logger.info("Running Scraper CLI")
    const { stdout, stderr } = await exec("tsy_disruptions_detector");
    if (stderr) {
        container.logger.error(`Scraper CLI: Error: ${stderr}`)
    }
    container.logger.info(`Scraper CLI: Info: ${stdout}`);

    // Pull from DB
    const db = sqlite3("disruptions.db");
    const rows = db.prepare('SELECT * FROM disruptions_diff').all();

    const events = [];
    rows.forEach((row: IDisruptionDiffRow) => {
        switch (row.state) {
            case DisruptionDiffState.New:
                events.push(`New Disruption!\nName: `)
        }
    })
}

interface IDisruptionDiffRow {
    id: number,
    disruption_id: string | undefined,
    disruption_name: string | undefined,
    state: DisruptionDiffState
}

enum DisruptionDiffState {
    New = 0,
    Changed = 1,
    Deleted = 2
}