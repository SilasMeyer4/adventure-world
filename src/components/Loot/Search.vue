<template>
    <div class="roll-loot">
              <!-- Entity -->
      <fieldset class="loot-group entity-window">
      <legend>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" viewBox="0 0 16 16">
      <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001c.03.04.062.078.098.115l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.1zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"/>
    </svg>
    </legend>
      <div class="input-row full-width">
      
      <div class="input-row">
      <div class="input-group vSelectParent">
        <input type="text" v-model="searchString">
        <button>Search</button>
      </div>
    </div>
  
    </div>
    
    </fieldset>
      <!-- Roll Loot Section -->
 </div>


 <div >
        <h3>{{ searchTable.length > 0 ? searchTable.length : 0}} found</h3>
 <!-- Display selected table name -->
    <table v-if="searchTable.length >= 0">
      <thead>
        <tr>
          <!-- Dynamically create headers -->
          <th @click="sortMode === SortMode.NAME_ASC ? changeSortMode(SortMode.NAME_DESC) : changeSortMode(SortMode.NAME_ASC)">Name</th>
          <th @click="sortMode === SortMode.POOL_ASC ? changeSortMode(SortMode.POOL_DESC) : changeSortMode(SortMode.POOL_ASC)">Pool</th>
          <th @click="sortMode === SortMode.POOL_ID_ASC ? changeSortMode(SortMode.POOL_ID_DESC) : changeSortMode(SortMode.POOL_ID_ASC)">Pool_ID</th>
        </tr>
      </thead>
      <tbody>
        <!-- Dynamically create rows -->
        <tr v-for="(row, rowIndex) in searchTable" :key="rowIndex" >
          <td v-for="(value, column) in row" :key="column" class="itemData" @click="getDetailedInfo(rowIndex)">
          <div >
            {{ value }}
          </div>

        </td>     
        </tr>
      </tbody>
    </table>
    <p v-else>No data available</p> <!-- Message if no data -->
  </div>



  </template>
  
  <script setup lang="ts">
  import { defineComponent, ref, watch } from 'vue';
  import type { PropType } from 'vue';

        enum SortMode {
            NAME_ASC,
            NAME_DESC,
            POOL_ASC,
            POOL_DESC,
            POOL_ID_ASC, 
            POOL_ID_DESC
        }

        const searchString = ref<string>('');
        const searchTable = ref<string[][]>([]);
        const sortMode = ref<SortMode>(SortMode.NAME_ASC)



        const sortSearchTable = () => {
            const sortedTable = [...searchTable.value];

            sortedTable.sort((a, b) => {

                let index = 0;
                let isASC: boolean = true;
                switch (sortMode.value) {
                    case SortMode.NAME_DESC:
                        isASC = false;
                        break;
                    case SortMode.POOL_ASC:
                        index = 1;
                        isASC = true;
                        break;
                    case SortMode.POOL_DESC:
                        index = 1;
                        isASC = false;
                        break;
                    case SortMode.POOL_ID_ASC:
                        index = 2;
                        isASC = true;
                        break;
                    case SortMode.POOL_ID_DESC:
                        index = 2;
                        isASC = false;
                        break;
                    default:
                        break;
                }

                let nameA = a[index];
                let nameB = b[index];

                if (sortMode.value !== SortMode.POOL_ID_ASC && sortMode.value !== SortMode.POOL_ID_DESC)
                {
                    nameA = nameA.toLowerCase();
                    nameB = nameB.toLowerCase();
                }

                if (nameA < nameB) {
                    if (isASC)
                    {
                        return -1;
                    }
                    else
                    {
                        return 1;
                    }
                    
                }

                if (nameA > nameB) {
                    if (isASC)
                    {
                        return 1;
                    }
                    else
                    {
                        return -1;
                    }
                }

                return 0;
                });

                searchTable.value = sortedTable;
        };

        const changeSortMode = (mode: SortMode) => {
            sortMode.value = mode;
            sortSearchTable();
        };

        const getDetailedInfo = (_rowIndex: number) => {
            console.log("wiwuwuw");
        }

  </script>

<style scoped>
    .itemData:hover {
        background-color: gray;
        cursor: pointer;
    }
</style>

