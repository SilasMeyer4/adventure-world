<template>
    <div>
        <!-- Topbar Navigation -->
        <div class="topbar">
            <div 
                v-for="(tab, tabKey) in tabLabels" 
                :key="tabKey" 
                @click="currentTab = tabKey" 
                :class="{ active: currentTab === tabKey }"
            >
                {{ tab }}
            </div>
        </div>
        <div class="menu-body">
            <Button @click="readCharacters">Press me</Button>
        </div>
    </div>
</template>

<script setup lang="ts">

    import { ref } from 'vue';
    import { readDir } from '@tauri-apps/plugin-fs';
    import {
        create_character,
        read_character_from_json,
        create_data_directory,
        Character,
        AbilityScores,
        CharacterStats,
        Sense,
    } from "./characters.ts";

    enum Tab {
        OVERVIEW,
    }

    const currentTab = ref(Tab.OVERVIEW);	
    const tabLabels: Record<Tab, string> = {
        [Tab.OVERVIEW]: "OVERVIEW"
    };

    const readCharacters = (async() => {
        let c: Character = {name: "TestName", 
        senses: [{ name: "TestSense1", value: 15 }, { name: "TestSense3", value: 15 }]};
        await create_character(c.name, c);

        read_character_from_json("test");

        const entries = await readDir("/data/characters");
        const fileNames: string[] = [];

        for (const entry of entries) {
            if (entry.isFile && entry.name) {
                fileNames.push(entry.name);
            }
        }

        console.log(fileNames);

    });

</script>