<template>
    <div>
        <div>
            <input type="text" name="" id="" placeholder="Path" v-model="databasePath">
            <button @click="choosePath">Choose Path</button>
        </div>

        <button @click="openDataDir">Open Data Directory</button>
    </div>
</template>

<script setup lang="ts">
import { open as openDialog} from '@tauri-apps/plugin-dialog';
import { appDataDir, join } from '@tauri-apps/api/path';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const databasePath = ref('');

const choosePath = async () => {
    try {
        const appDataPath = await appDataDir();

        const dataFolderPath = await join(appDataPath, 'data');

        const folderPath = await openDialog({
            directory: true,
            defaultPath: dataFolderPath,
        });

        if (folderPath) {
            databasePath.value = folderPath;
        }
    } catch (error) {
        console.error(error);
    }
}

const openDataDir = (async() => {
  try {
    // Get the directory path for your app's data
    const appDataPath = await appDataDir();
    invoke("open_in_file_system", {path: appDataPath});

  } catch (error) {
    console.error("Failed to open folder:", error);
  }
});

</script>