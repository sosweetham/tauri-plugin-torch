/**
 * Control the device flashlight (torch) on Android and iOS.
 *
 * On Android the torch keeps working while the barcode scanner plugin holds the camera.
 *
 * @module
 */

import { invoke } from "@tauri-apps/api/core";

/**
 * Turns the torch on.
 *
 * Rejects if the device has no torch, the torch is temporarily unavailable (e.g. the device is
 * overheating) or the camera is held by another app.
 *
 * @example
 * ```typescript
 * import { enable } from '@sosweetham/tauri-plugin-torch-api'
 * await enable()
 * ```
 */
export async function enable(): Promise<void> {
    await invoke("plugin:torch|enable");
}

/**
 * Turns the torch off.
 *
 * Rejects in the same cases as {@link enable}.
 *
 * @example
 * ```typescript
 * import { disable } from '@sosweetham/tauri-plugin-torch-api'
 * await disable()
 * ```
 */
export async function disable(): Promise<void> {
    await invoke("plugin:torch|disable");
}

/**
 * Flips the torch.
 *
 * Rejects in the same cases as {@link enable}.
 *
 * @example
 * ```typescript
 * import { toggle } from '@sosweetham/tauri-plugin-torch-api'
 * const on = await toggle()
 * ```
 *
 * @returns Whether the torch is now on.
 */
export async function toggle(): Promise<boolean>;
/**
 * Turns the torch on (`true`) or off (`false`).
 *
 * Resolves to `null` instead of rejecting when the device has no torch.
 *
 * @deprecated Use {@link enable} or {@link disable} instead.
 *
 * @returns `true` if the torch is now on, `null` otherwise.
 */
export async function toggle(value: boolean): Promise<boolean | null>;
export async function toggle(value?: boolean): Promise<boolean | null> {
    const r = await invoke<{ value?: boolean }>("plugin:torch|toggle", {
        payload: { value },
    });
    if (value === undefined) {
        return r.value ?? false;
    }
    return r.value ? r.value : null;
}

/**
 * Whether the torch is currently on.
 *
 * @deprecated Use {@link isEnabled} instead.
 *
 * @returns `true` if the torch is on, `null` otherwise.
 */
export async function check(): Promise<boolean | null> {
    return await invoke<{ value?: boolean }>("plugin:torch|check", {
        payload: {},
    }).then((r) => (r.value ? r.value : null));
}

/**
 * Whether the device has a torch.
 *
 * @example
 * ```typescript
 * import { isAvailable } from '@sosweetham/tauri-plugin-torch-api'
 * if (await isAvailable()) {
 *   // show the torch button
 * }
 * ```
 */
export async function isAvailable(): Promise<boolean> {
    return await invoke<{ available: boolean }>(
        "plugin:torch|is_available",
    ).then((r) => r.available);
}

/**
 * Whether the torch is currently on.
 *
 * This reads the hardware state, so it also reflects the torch being turned off by the system,
 * another app, or a barcode scan ending.
 *
 * On iOS, the torch turned on from Control Center or by another app is not visible to the app,
 * so this returns `false` for it.
 *
 * @example
 * ```typescript
 * import { isEnabled } from '@sosweetham/tauri-plugin-torch-api'
 * const on = await isEnabled()
 * ```
 */
export async function isEnabled(): Promise<boolean> {
    return await invoke<{ enabled: boolean }>("plugin:torch|is_enabled").then(
        (r) => r.enabled,
    );
}
