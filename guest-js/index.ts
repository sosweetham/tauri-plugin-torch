import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:torch|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function toggle(value: boolean): Promise<boolean | null> {
  return await invoke<{ value?: boolean }>("plugin:torch|toggle", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function check(): Promise<boolean | null> {
  return await invoke<{ value?: boolean }>("plugin:torch|check", {
    payload: {},
  }).then((r) => (r.value ? r.value : null));
}
