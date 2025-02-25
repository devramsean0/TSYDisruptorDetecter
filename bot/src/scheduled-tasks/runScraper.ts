import { ScheduledTask } from '@sapphire/plugin-scheduled-tasks';
import { exec } from "node:child_process";
export class RunScraperTask extends ScheduledTask {
    public constructor(context: ScheduledTask.LoaderContext, options: ScheduledTask.Options) {
        super(context, {
            ...options,
            interval: 60_000 // 60 seconds
        });
    }

    public async run() {
        this.container.logger.info("Running Scraper CLI")
        exec("tsy_disruptions_detector", (error, stdout, stderr) => {
            if (error) {
                this.container.logger.error(`[Scraper CLI] error: ${error}`);
            }
            if (stderr) {
                this.container.logger.error(`[Scraper CLI] error: ${stderr}`);
            }
            this.container.logger.info(`[Scraper CLI] output: ${stdout}`);
        })
    }
}

declare module '@sapphire/plugin-scheduled-tasks' {
    interface ScheduledTasks {
        interval: never;
    }
}