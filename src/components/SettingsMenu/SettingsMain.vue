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
            <PathSettings v-if="currentTab === Tab.PATH_SETTINGS"/>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

import PathSettings from './PathSettings.vue';

enum Tab {
    PATH_SETTINGS,
}

const currentTab = ref(Tab.PATH_SETTINGS);	
const tabLabels: Record<Tab, string> = {
    [Tab.PATH_SETTINGS]: "Create Tab",
};

</script>

<style scoped>

</style>