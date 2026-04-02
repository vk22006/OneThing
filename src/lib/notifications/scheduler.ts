import { NotificationManager } from './notificationManager';

export interface FocusSettings {
	enabled: boolean;
	interval: number;
	focusMode: boolean;
	sound: boolean;
}

export class FocusScheduler {
	private intervalId: ReturnType<typeof setInterval> | null = null;
	private settings: FocusSettings = {
		enabled: true,
		interval: 25, // minutes
		focusMode: false,
		sound: true
	};

	start(onNudge?: () => void): void {
		if (this.intervalId) return;

		const ms = this.settings.interval * 60 * 1000;
		this.intervalId = setInterval(() => {
			if (this.settings.enabled) {
				this.triggerNudge();
				if (onNudge) onNudge();
			}
		}, ms);
	}

	stop(): void {
		if (this.intervalId) {
			clearInterval(this.intervalId);
			this.intervalId = null;
		}
	}

	updateSettings(newSettings: Partial<FocusSettings>): void {
		this.settings = { ...this.settings, ...newSettings };
		if (this.intervalId) {
			this.stop();
			this.start();
		}
	}

	private async triggerNudge(): Promise<void> {
		if (this.settings.focusMode) {
			// Optional: stricter nudge for focus mode
			await NotificationManager.notify(
				'Locked In?',
				"Keep your focus on the 'One Thing'. You're doing great!"
			);
		} else {
			await NotificationManager.notify('Focus Reminder', 'Time to check back on your task.');
		}
	}
}

export const focusScheduler = new FocusScheduler();
