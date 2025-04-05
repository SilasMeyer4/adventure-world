<template>
    <!-- Table content here -->
     
    <div class="table-container">
        <button @click="addLootGroups">Dummy Data</button>
      <table>
        <thead>
          <tr>
            <th>Name</th>
            <th>Probability</th>
            <th>Amount</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="lootgroup in filteredLootGroups"
            :key="lootgroup.name"
            @click="selectLootGroup(lootgroup)"
          >
            <td>{{ lootgroup.name }}</td>
            <td>{{ lootgroup.probability }}</td>
            <td>{{ lootgroup.amount }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </template>

<script setup lang="ts">

import { invoke } from '@tauri-apps/api/core';
import { ref, computed, watch, defineProps, onMounted } from 'vue';


const lootGroups = ref<LootGroup[]>([]);

onMounted(() => {

    console.log("vue lootgroups");
});


const props = defineProps({
  searchQuery: String,
});

interface LootGroup {
  name: string;
  probability?: number;  // optional field, corresponds to Option<u8>
  amount?: number;       // optional field, corresponds to Option<u32>
}

// Fetch loot groups based on the search query
const fetchLootGroups = async () => {
    if (props.searchQuery === undefined) return; // Return early if searchQuery is undefined

  if (!props.searchQuery.trim()) {
    lootGroups.value = []; // Clear results if the search query is empty
    return;
  }

  try {
    lootGroups.value = await invoke('search_loot_group_by_name', {
      name: props.searchQuery,
    });
  } catch (error) {
    console.error('Error fetching loot groups:', error);
  }
};


// Watch for changes to searchQuery prop and fetch loot groups when it changes
watch(
  () => props.searchQuery,
  (newQuery) => {
    fetchLootGroups(); // Call the function to fetch loot groups on searchQuery change
  }
);

// Computed property for filtered loot groups (could add additional filters here)
const filteredLootGroups = computed(() => lootGroups.value);

// Function to handle loot group selection (optional)
const selectLootGroup = (lootgroup: LootGroup) => {
  console.log('Loot group selected:', lootgroup);
};


const addLootGroups = async () => {
    await invoke('add_loot_group', {lootgroup: { name: 'Example Loot Group', probability: 50, amount: 100 } });
    await invoke('add_loot_group', {lootgroup: { name: 'Exametet Group', probability: 50, amount: 100 } });
    await invoke('add_loot_group', {lootgroup: { name: 'Steven', probability: 10, amount: 20 } });
    await invoke('add_loot_group', {lootgroup: { name: 'Test', probability: 30, amount: 100 } });
    await invoke('add_loot_group', {lootgroup: { name: 'ch', probability: 50, amount: 135 } });
    await invoke('add_loot_group', {lootgroup: { name: 'Bibu', probability: 60, amount: 100 } });
}

</script>