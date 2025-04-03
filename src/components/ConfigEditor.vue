<template>

    <button @click="save_config">Save Config</button>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { Ref } from "vue";

import { onMounted, ref } from 'vue';

interface Config {
  db_path: string;
}

const config_data = ref<String>("");


async function load_config() {
    const config = await invoke("get_config_as_string");
    return config;
}

const save_config = (async () => {
    await invoke("set_config", );
    await invoke("save_config");
});

onMounted(async () => {
    const config: String = (await load_config()) as String;

    config_data.value = config;

})


</script>