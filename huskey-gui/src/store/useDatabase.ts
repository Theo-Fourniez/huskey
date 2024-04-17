import { HuskeyDatabase, PasswordEntry } from "@/types/huskeyTypes";
import { app, invoke } from "@tauri-apps/api";
import { appWindow } from "@tauri-apps/api/window";
import { computed, reactive, ref, watch } from "vue";

interface DatabaseState {
    database: HuskeyDatabase | null;
    hasUnsavedChanges: boolean;
    path: string;
    cachedPassword: string | null;
}

export const databaseState = reactive<DatabaseState>({database: null, hasUnsavedChanges: false, path: "", cachedPassword: null});

export async function tryOpenDatabase(path: string, password: string) : Promise<void> | never{
    console.log("Open database command invoked")
    databaseState.database = await invoke<HuskeyDatabase>('open_database', { path, password });
    databaseState.hasUnsavedChanges = false;
    databaseState.path = path;
    databaseState.cachedPassword = password;
}

export async function tryCreateDatabase(path: string, password: string) : Promise<void> | never{
    console.log("Create database command invoked")
    databaseState.database = await invoke<HuskeyDatabase>('create_database', { path, password });
    databaseState.hasUnsavedChanges = false;
    databaseState.path = path;
    databaseState.cachedPassword = password;
}

export async function trySaveDatabase(password: string) : Promise<void> | never{
    console.log("Save database command invoked")
    await invoke('save_database', {password, path: databaseState.path});
    databaseState.hasUnsavedChanges = false;
    databaseState.cachedPassword = password;
}

export async function tryCloseDatabase() : Promise<void> | never{
    console.log("Close database command invoked")
    await invoke('close_database');
    databaseState.database = null;
    databaseState.hasUnsavedChanges = false;
}

export async function addPasswordEntry(entry: PasswordEntry) : Promise<void> | never{
    console.log("Add password entry command invoked")
    const updatedDatabase: HuskeyDatabase= await invoke('add_password_entry', {entry});
    databaseState.database = updatedDatabase;
    databaseState.hasUnsavedChanges = true;
}

export async function editPasswordEntry(oldEntry: PasswordEntry, newEntry: PasswordEntry) : Promise<void> | never{
    console.log("Edit password entry command invoked with old entry: ", oldEntry, " and new entry: ", newEntry);
    const updatedDatabase: HuskeyDatabase = await invoke('edit_password_entry', {oldEntry, newEntry});
    databaseState.database = updatedDatabase;
    databaseState.hasUnsavedChanges = true;
}

export async function removePasswordEntry(entry: PasswordEntry) : Promise<void> | never{
    console.log("Remove password entry command invoked with entry: ", entry);
    const updatedDatabase: HuskeyDatabase = await invoke('remove_password_entry', {entry});
    databaseState.database = updatedDatabase;
    databaseState.hasUnsavedChanges = true;
}

export const isDatabaseOpen = computed(() => databaseState.database !== null);

// Sets the title of the window to indicate if there are unsaved changes
const UNSAVED_CHANGES_SUFFIX = " (*)";
const DEFAULT_TITLE = await appWindow.title();

async function getUnsavedChangesTitle()  {
    return await appWindow.title() + UNSAVED_CHANGES_SUFFIX;
}

watch(databaseState, async () => {
    console.log("[useDatabase] databaseState changed to ", databaseState);
    if(databaseState.database && databaseState.hasUnsavedChanges){
        if(await appWindow.title() === DEFAULT_TITLE){
            await appWindow.setTitle(await getUnsavedChangesTitle());
        }
    }
    else{
        await appWindow.setTitle(DEFAULT_TITLE);
    }
});