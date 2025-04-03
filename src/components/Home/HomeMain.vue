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
            <Home v-if="currentTab === Tab.HOME"/>
        </div>
    </div>
</template>

<script setup lang="ts">

    import { ref } from 'vue';
    import Home from './Home.vue';

    enum Tab {
        HOME,
    }

    const currentTab = ref(Tab.HOME);	
    const tabLabels: Record<Tab, string> = {
        [Tab.HOME]: "HOME"
    };

</script>