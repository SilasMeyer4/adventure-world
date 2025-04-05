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
            <Home v-if="currentTab === Tab.OVERVIEW"/>
        </div>
    </div>
</template>

<script setup lang="ts">

    import { ref } from 'vue';

    enum Tab {
        OVERVIEW,
    }

    const currentTab = ref(Tab.OVERVIEW);	
    const tabLabels: Record<Tab, string> = {
        [Tab.OVERVIEW]: "OVERVIEW"
    };

</script>