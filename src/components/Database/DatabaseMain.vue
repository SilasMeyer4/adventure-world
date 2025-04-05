<template>
    <div class="main-container">
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
  
      <!-- Main Content -->
      <div class="menu-body">
        <div class="left-column">
          <!-- Table and Search Section -->
          <div class="search-bar">
            <input 
              type="text" 
              v-model="searchQuery" 
              placeholder="Search..." 
            />
          </div>

          <!--Table-->
          <LootGroups v-if="currentTab === Tab.LOOT_GROUPS" :searchQuery="searchQuery"></LootGroups>
  
        <!-- Right Column (Details) -->
        <div class="right-column" v-if="selectedItem">
          <h2>Details</h2>
          <div class="details">
            <p><strong>Name:</strong> {{ selectedItem.name }}</p>
            <p><strong>Kind:</strong> {{ selectedItem.description }}</p>
        
            <!-- Add other item details as needed -->
          </div>
        </div>
      </div>
    </div>
    </div>
  </template>

<script setup lang="ts">

    import Items from './Items.vue';
    import Spells from './Spells.vue';
    import Entities from './Entities.vue';
    import Places from './Places.vue';
    import Languages from './Languages.vue';
    import LootGroups from './LootGroups.vue';

    import { ref, computed, watch } from 'vue';
    import { invoke } from '@tauri-apps/api/core';

    // State variables
    const searchQuery = ref(''); // Search query for filtering table
    const selectedItem = ref(); // The item selected from the table

    enum Tab {
        ITEMS = "ITEM<S",
        SPELLS = "SPELLS",
        ENTITIES = "ENTITIES",
        PLACES = "PLACES" ,
        LANGUAGES = "LANGUAGES",
        LOOT_GROUPS = "LOOT GROUPS",
    }

    const currentTab = ref(Tab.ITEMS);	
    const tabLabels: Record<Tab, string> = {
        [Tab.ITEMS]: "ITEMS",
        [Tab.SPELLS]: "SPELLS",
        [Tab.ENTITIES]: "ENTITIES",
        [Tab.PLACES]: "PLACES",
        [Tab.LANGUAGES]: "LANGUAGES",
        [Tab.LOOT_GROUPS]: "LOOT GROUPS",

    };

 watch(currentTab, (newValue, oldValue) => {
    console.log(newValue); //.value);the new value of searchQuery
});

</script>


<style scoped>
.main-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.menu-body {
  display: flex;
  flex: 1;
}

.left-column {
  flex: 1;
  padding: 20px;
  border-right: 1px solid #ddd;
}

.search-bar input {
  width: 100%;
  padding: 10px;
  margin-bottom: 20px;
  border-radius: 4px;
  border: 1px solid #ccc;
}

.table-container table {
  width: 100%;
  border-collapse: collapse;
}

.table-container th, .table-container td {
  padding: 10px;
  border: 1px solid #ddd;
}

.table-container tr:hover {
  background-color: #f1f1f1;
}

.right-column {
  flex: 2;
  padding: 20px;
  background-color: #333;
}

.details p {
  margin: 10px 0;
}

</style>