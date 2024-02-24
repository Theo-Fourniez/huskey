import { HuskeyDatabase, PasswordEntry } from "@/types/huskeyTypes";
import { invoke } from "@tauri-apps/api";
import { reactive, ref } from "vue";

export const database = ref<HuskeyDatabase | undefined>(undefined);
export const hasUnsavedChanges = ref(false);

export async function tryOpenDatabase(path: string, password: string) : Promise<void> | never{
    console.log("Open database command invoked")
    database.value = await invoke<HuskeyDatabase>('open_database', { path, password });
    hasUnsavedChanges.value = false;
}

export async function trySaveDatabase(password: string) : Promise<void> | never{
    console.log("Save database command invoked")
    await invoke('save_database', {password});
    hasUnsavedChanges.value = false;
}

export async function tryCloseDatabase() : Promise<void> | never{
    console.log("Close database command invoked")
    await invoke('close_database');
    database.value = undefined;
    hasUnsavedChanges.value = false;
}

export async function addPasswordEntry(entry: PasswordEntry) : Promise<void> | never{
    console.log("Add password entry command invoked")
    await invoke('add_password_entry', {entry});
    hasUnsavedChanges.value = true;
}