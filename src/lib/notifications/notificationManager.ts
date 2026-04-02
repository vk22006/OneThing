import {
	isPermissionGranted,
	requestPermission,
	sendNotification
} from '@tauri-apps/plugin-notification';
import { invoke } from '@tauri-apps/api/core';
import { browser } from '$app/environment';

export class NotificationManager {
	static async checkPermission(): Promise<boolean> {
		if (!browser) return false;

		let permissionGranted = await isPermissionGranted();
		if (!permissionGranted) {
			const permission = await requestPermission();
			permissionGranted = permission === 'granted';
		}
		return permissionGranted;
	}

	static async notify(title: string, body: string): Promise<void> {
		if (!browser) return;

		if (await this.checkPermission()) {
			sendNotification({ title, body });
		}
	}

	static async schedule(
		id: string,
		title: string,
		body: string,
		scheduledAt: Date | number,
		type: string = 'deadline'
	): Promise<void> {
		if (!browser) return;

		// Accept Date, milliseconds, or unix seconds. Normalize to unix seconds.
		const timestamp =
			scheduledAt instanceof Date
				? Math.floor(scheduledAt.getTime() / 1000)
				: scheduledAt > 10_000_000_000
					? Math.floor(scheduledAt / 1000)
					: Math.floor(scheduledAt);

		await invoke('schedule_notification', {
			id,
			title,
			body,
			scheduledAt: timestamp,
			notificationType: type
		});
	}
}
