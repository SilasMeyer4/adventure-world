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
            <RollLoot v-if="currentTab === Tab.ROLLLOOT"/>
            <Search v-if="currentTab === Tab.SEARCH"/>
            <CreateTable v-if="currentTab === Tab.CREATETAB"/>
            <UpdateTable v-if="currentTab === Tab.UPDATETABLE"/>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

import CreateTable from './CreateTable.vue';
import UpdateTable from './UpdateTable.vue';
import RollLoot from './RollLoot.vue';
import Search from './Search.vue';

enum Tab {
    CREATETAB = "Create Tab",
    DATABASEMANAGER = "Database Manager", 
    UPDATETABLE = "Update Table",
    ROLLLOOT = "Roll Loot",
    SEARCH = "Search"
}

const currentTab = ref(Tab.CREATETAB);	
const tabLabels: Record<Tab, string> = {
    [Tab.CREATETAB]: "Create Tab",
    [Tab.DATABASEMANAGER]: "Database Manager",
    [Tab.UPDATETABLE]: "Update Table",
    [Tab.ROLLLOOT]: "Roll Loot",
    [Tab.SEARCH]: "Search"
};

</script>

<style scoped>

</style>