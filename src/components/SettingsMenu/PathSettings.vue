<template>
    <div>
        <div>
            <input type="text" name="" id="" placeholder="Path" v-model="databasePath">
            <button @click="choosePath">Choose Path</button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-dialog';
import { appDataDir, join } from '@tauri-apps/api/path';
import { ref } from 'vue';

const databasePath = ref('');

const choosePath = async () => {
    try {
        const appDataPath = await appDataDir();

        const dataFolderPath = await join(appDataPath, 'data');

        const folderPath = await open({
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

</script>