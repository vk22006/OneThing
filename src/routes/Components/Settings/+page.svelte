<script lang="ts">
    import Header from '../../../lib/layout/Header.svelte';
    import { theme, type Theme } from "$lib/stores/theme";
    import { notificationSettings } from "$lib/stores/notifications";
    import { onMount } from "svelte";
    import { loadSettings } from "$lib/stores/notifications";

    let pageTitle = $state('Settings');

    onMount(() => {
        loadSettings();
    });

    function setTheme(t: Theme) {
        theme.set(t);
    }

    function updateSetting(key: string, value: any) {
        notificationSettings.update(s => ({ ...s, [key]: value }));
    }
</script>

<Header page={pageTitle} />

<div class="px-8 py-8 max-w-4xl mx-auto w-full">
    <!-- Appearance Section -->
    <div class="bg-white border border-gray-200 rounded-[20px] shadow-sm p-8 mb-8">
        <h2 class="text-xl font-bold text-gray-900 mb-6 tracking-tight flex items-center gap-2">
            <span class="w-2 h-6 bg-blue-500 rounded-full"></span>
            Appearance
        </h2>
        <div class="flex gap-4">
            <button
                onclick={() => setTheme("light")}
                class="flex-1 py-4 px-6 rounded-2xl border-2 transition-all duration-200 font-semibold { $theme === 'light' ? 'border-blue-500 bg-blue-50 text-blue-700 shadow-sm' : 'border-gray-100 text-gray-500 hover:border-gray-200 hover:bg-gray-50' }">
                Light Mode
            </button>
            <button
                onclick={() => setTheme("dark")}
                class="flex-1 py-4 px-6 rounded-2xl border-2 transition-all duration-200 font-semibold { $theme === 'dark' ? 'border-blue-500 bg-blue-50 text-blue-700 shadow-sm' : 'border-gray-100 text-gray-500 hover:border-gray-200 hover:bg-gray-50' }">
                Dark Mode
            </button>
        </div>
    </div>

    <!-- Notifications Section -->
    <div class="bg-white border border-gray-200 rounded-[20px] shadow-sm p-8">
        <h2 class="text-xl font-bold text-gray-900 mb-6 tracking-tight flex items-center gap-2">
            <span class="w-2 h-6 bg-cyan-500 rounded-full"></span>
            Notifications
        </h2>
        
        <div class="space-y-6">
            <!-- Main Toggle -->
            <div class="flex items-center justify-between p-4 bg-gray-50 rounded-2xl">
                <div>
                    <p class="font-bold text-gray-900">Enable System Notifications</p>
                    <p class="text-sm text-gray-500">Get reminders for deadlines and focus nudges</p>
                </div>
                <button 
                    onclick={() => updateSetting('enabled', !$notificationSettings.enabled)}
                    aria-label="Toggle system notifications"
                    title="Toggle system notifications"
                    class="w-14 h-8 rounded-full transition-colors relative {$notificationSettings.enabled ? 'bg-cyan-500' : 'bg-gray-300'}">
                    <div class="absolute top-1 left-1 w-6 h-6 bg-white rounded-full transition-transform {$notificationSettings.enabled ? 'translate-x-6' : ''}"></div>
                </button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6 {$notificationSettings.enabled ? '' : 'opacity-50 pointer-events-none transition-opacity'}">
                <!-- Sound Toggle -->
                <div class="p-4 border border-gray-100 rounded-2xl hover:border-cyan-200 transition-colors">
                    <div class="flex items-center justify-between mb-2">
                        <p class="font-bold text-gray-900">Notification Sound</p>
                        <input type="checkbox" checked={$notificationSettings.sound} onchange={(e) => updateSetting('sound', e.currentTarget.checked)} class="w-5 h-5 accent-cyan-500" />
                    </div>
                    <p class="text-xs text-gray-500">Play a subtle sound for every alert</p>
                </div>

                <!-- Focus Mode Toggle -->
                <div class="p-4 border border-gray-100 rounded-2xl hover:border-cyan-200 transition-colors">
                    <div class="flex items-center justify-between mb-2">
                        <p class="font-bold text-gray-900">Focus Mode</p>
                        <input type="checkbox" checked={$notificationSettings.focusMode} onchange={(e) => updateSetting('focusMode', e.currentTarget.checked)} class="w-5 h-5 accent-cyan-500" />
                    </div>
                    <p class="text-xs text-gray-500">Suppresses all but urgent deadline alerts</p>
                </div>

                <!-- Interval Slider -->
                <div class="p-4 border border-gray-100 rounded-2xl md:col-span-2">
                    <div class="flex justify-between mb-4">
                        <p class="font-bold text-gray-900">Focus Nudge Interval</p>
                        <span class="px-3 py-1 bg-cyan-100 text-cyan-700 rounded-full font-bold">{$notificationSettings.focusInterval} mins</span>
                    </div>
                    <input 
                        type="range" min="5" max="60" step="5" 
                        value={$notificationSettings.focusInterval}
                        oninput={(e) => updateSetting('focusInterval', parseInt(e.currentTarget.value))}
                        class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-cyan-500"
                    />
                    <div class="flex justify-between mt-2 text-[10px] text-gray-400 font-medium">
                        <span>5 MIN</span>
                        <span>30 MIN</span>
                        <span>60 MIN</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
