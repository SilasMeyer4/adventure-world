<template>
  <div class="roll-loot">
            <!-- Entity -->
    <fieldset class="loot-group entity-window">
    <legend>Entity</legend>
    <div class="input-row full-width">
    
    <div class="input-row">
    <div class="input-group vSelectParent">
      <label for="classLootDropdown">Name</label>
      <v-select
        v-model="selectedEntity"
        :options="entityOptions"
        label="name"
        placeholder="Select a table"
        class="custom-v-select"
        :clearable="false"
        :close-on-select="true"
      />
    </div>
  </div>

  </div>
  
  </fieldset>
    <!-- Roll Loot Section -->
    <div class="loot-container">


  <!-- Class Loot -->

  <fieldset class="loot-group">
    <legend>Class</legend>
    <div class="input-row full-width">
    <div class="input-group">
      <label for="classLootAmount">Amount</label>
      <input type="number" id="classLootAmount" v-model="classLootAmount" placeholder="Amount">
    </div>

    <div class="input-group">
      <label for="classLootChance">Chance</label>
      <input type="number" id="classLootChance" v-model="classLootChance" placeholder="Chance">
    </div>

    <div class="input-row">
    <div class="input-group vSelectParent">
      <label for="classLootDropdown">Table</label>
      <v-select
        v-model="selectedClassTable"
        :options="classLootOptions"
        label="name"
        placeholder="Select a table"
        class="custom-v-select"
        :clearable="false"
        :close-on-select="true"
      />
    </div>
  </div>
  </div>
  </fieldset>

  <!-- Type Loot -->
  <fieldset class="loot-group">
    <legend>Type</legend>
    <div class="input-row full-width">
    <div class="input-group">
      <label for="typeLootAmount">Amount</label>
      <input type="number" id="typeLootAmount" v-model="typeLootAmount" placeholder="Amount">
    </div>

    <div class="input-group">
      <label for="typeLootChance">Chance</label>
      <input type="number" id="typeLootChance" v-model="typeLootChance" placeholder="Chance">
    </div>

    <div class="input-row">
    <div class="input-group vSelectParent">
      <label for="typeLootDropdown">Table</label>
      <v-select
        v-model="selectedTypeTable"
        :options="typeLootOptions"
        label="name"
        placeholder="Select a table"
        class="custom-v-select"
        :clearable="false"
        :close-on-select="true"
      />
    </div>
  </div>
  </div>
  </fieldset>
</div>

    <div class="button-group">
      <button>Roll Loot</button>
    </div>

    <!-- Output -->
    <div class="output-container">
      <div class="history-group">
        <label for="historyText">History</label>
        <div id="historyText" class="history-box" v-html="historyText" ref="historyContainer"></div>
      </div>

      <div class="output-group">
        <label for="outputText">Output</label>
        <div id="outputText" class="output-box" v-html="outputText" ref="outputContainer"></div>
      </div>
    </div>
  </div>

  <div class="button-group clear-group">
      <button @click="clearInput">Clear Input</button>
      <button>Clear Output</button>
      <button>Save Loot As Picture</button>
      <button>Save History As Picture</button>
      <button @click="isDetailAreaShown ? closeDetailArea() : openDetailArea()">Show Details</button>
  </div>

  <div class="detailArea-wrapper">
    <div
      class="detailArea"
      :class="{ open: isDetailAreaShown }"
      :style="{ width: detailAreaWidth + 'px' }"
      ref="detailArea"
    >
      <div class="resizer" @mousedown="startResizing"></div>
      <button class="area-close-button" @click="closeDetailArea">-></button>

      <!-- Table Inside the Detail Area -->
       <div style="display: flex; justify-content: space-between; gap: 20px; align-items: center;">
        <div></div>
        <h3>Dropped Loot</h3>
        <button>Save Detailed Loot as Picture</button>
        <button>Clear Detail Data</button>
      </div>
      
      <div id="outputText" class="detailedLootContainer" ref="detailedLootContainer">
      <div v-for="(loot, index) in lootData" :key="index" class="loot-table-container">
        <h2>{{ loot.poolName }}</h2>
      <table class="loot-table" v-if="loot.tableHeaders.length> 0">
        
        <thead>
          <tr>
            <!-- Dynamically create headers -->
            <th v-for="header in loot.tableHeaders" :key="header">{{ header }}</th>
          </tr>
        </thead>
        <tbody>
          <!-- Dynamically create rows -->
          <tr v-for="(row, rowIndex) in loot.tableData" :key="rowIndex">
            <td v-for="(value, column) in row" :key="column">{{ value }}</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineComponent, ref, onMounted, nextTick, watch, onUnmounted } from 'vue';
import type { PropType } from 'vue';

    const lootData = ref<any[]>([]);
    const classLootAmount = ref<number>(0);
    const classLootChance = ref<number>(0);
    const classLootDropdown = ref<string>('');
    const typeLootAmount = ref<number>(0);
    const typeLootChance = ref<number>(0);
    const typeLootDropdown = ref<string>('');
    const selectedTypeTable = ref<string>('');
    const selectedClassTable = ref<string>('');

    const selectedEntity = ref<string>('');
    const entityOptions = ref<string[]>([]);
    
    const classLootOptions = ref<string[]>([]);
    const typeLootOptions = ref<string[]>([]);

    const historyText = ref<string>(''); 
    const outputText = ref<string>('');
    const outputContainer = ref<HTMLElement | null>(null);
    const historyContainer = ref<HTMLElement | null>(null);
    const detailedLootContainer = ref<HTMLElement | null>(null);

    const isDetailAreaShown = ref<boolean>(false);
    const detailAreaWidth = ref<number>(250);
    let isDetailAreaResizing = false;
    let savedDetailAreaWidth = detailAreaWidth.value;

    const viewPortWidth = ref(window.innerWidth);




    onMounted(() => {

    });

    onUnmounted(() => {
      window.removeEventListener('resize', updateViewPortWidth);
      document.removeEventListener('mousemove', resizeDetailArea);
      document.removeEventListener('mouseup', stopResizing);
    });

    const clearInput = () => {
      selectedEntity.value = '';

      selectedClassTable.value = '';
      classLootAmount.value = 0;
      classLootChance.value = 0;

      selectedTypeTable.value = '';
      typeLootAmount.value = 0;
      typeLootChance.value = 0;
    }




    const startResizing = () => {
      isDetailAreaResizing = true;
      document.addEventListener('mousemove', resizeDetailArea);
      document.addEventListener('mouseup', stopResizing);
    };
    
    const resizeDetailArea = (event: MouseEvent) => {
      if (isDetailAreaResizing) {
        const newWidth = window.innerWidth - event.clientX;
        const margin = viewPortWidth.value / 20;
        if (newWidth >= margin && newWidth <= viewPortWidth.value - margin ) {
          detailAreaWidth.value = newWidth;
        }
      }
    };

    const stopResizing = () => {
      isDetailAreaResizing = false;
      document.removeEventListener('mousemove', resizeDetailArea);
      document.removeEventListener('mouseup', stopResizing);
    }

    const closeDetailArea = () => {
      isDetailAreaShown.value = false;
      savedDetailAreaWidth = detailAreaWidth.value;
      detailAreaWidth.value = 250
    }

    const openDetailArea = () => {
      isDetailAreaShown.value = true;
      detailAreaWidth.value = savedDetailAreaWidth
    }

    const updateViewPortWidth = () => {
      viewPortWidth.value = window.innerWidth;
    }

    watch(viewPortWidth, (newWidth, oldWidth) => {
      const changeRate = newWidth / oldWidth;
      if (isDetailAreaShown)
      {
        detailAreaWidth.value *= changeRate;
      }
        savedDetailAreaWidth *= changeRate;    
    });

    const scrollToBottom = async() => {
    await nextTick(() => {
    if (outputContainer.value) {
      outputContainer.value.scrollTop = outputContainer.value.scrollHeight;
    }
    if (historyContainer.value) {
      historyContainer.value.scrollTop = historyContainer.value.scrollHeight;
    }
  });
};

</script>

<style scoped>

.loot-fields {
  display: flex;
  gap: 10px; /* Space between the input fields */
  align-items: center; /* Vertically align inputs if they have different heights */
}

input[type="number"],
input[type="text"] {
  padding: 8px;
  font-size: 14px;
  border: 1px solid #ccc;
  border-radius: 4px;
  width: 100px; /* or adjust according to your design */
  box-sizing: border-box;
}

.input-container {
  display: flex;
  justify-content: center; /* Centers the input groups horizontally */
  align-items: center; /* Centers the input groups vertically */
  gap: 20px; /* Adds space between the two groups */
  flex-wrap: wrap; /* Ensures the content wraps on smaller screens */
  max-width: 100%; /* Ensures it doesn't stretch beyond its container */
  margin: 0 auto; /* Centers the container */
}

.loot-container {
  display: flex;
  justify-content: space-between; /* Spread evenly */
  gap: 20px; /* Space between Class and Type groups */
  flex-wrap: wrap; /* Ensure responsiveness */
  max-width: 100%;
  margin: 20px auto;
}

.loot-group {
  flex: 1; /* Make both groups equal width */
  min-width: 300px; /* Prevent collapsing */
  display: flex;
  border: 1px solid #fff; /* Proper border */
  border-radius: 10px; /* Rounded corners */
  background-color: rgba(255, 255, 255, 0); /* Slight background for visibility */
  align-items: center;
  width: 100%;
}

.input-group {
  display: flex;
  flex-direction: column;
  margin-bottom: 15px;
  flex: 1; /* Each input takes equal space */
  min-width: 150px; /* Prevents shrinking too much */
  align-items: center;
  margin: auto;
}

label {
  margin-bottom: 5px;
}

input[type="text"] {
  padding: 8px;
  width: 100%;
  box-sizing: border-box;
  border: 1px solid #ccc;
  border-radius: 4px;
}
/* Container to align history and output side by side */
.output-container {
  display: flex;
  justify-content: center; /* Center the content */
  align-items: flex-start;
  gap: 20px; /* Space between history and output */
  width: 90%; /* Slight margin from the edges */
  max-width: 100%;
  margin: 20px auto; /* Center the container */
  padding: 10px;
  box-sizing: border-box;
}

.history-group,
.output-group {
  flex: 1;
  min-width: 200px;
}

.input-row {
  display: flex;
  gap: 15px;
  justify-content: space-between;
  width: 100%;
}

.full-width {
  width: 100%;
  display: flex;
}

/* Shared styling for history and output boxes */
.history-group .history-box,
.output-group .output-box {
  padding: 10px;
  font-size: 14px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #1a1a1a;
  color: white;
  box-shadow: 0 0 5px rgba(0, 0, 0, 0.2);
  width: 100%;
  height: 300px; /* Increased height for better visibility */
  overflow-y: auto; /* Enable scrolling */
  white-space: pre-wrap; /* Preserve line breaks */
  word-wrap: break-word; /* Prevent overflow */
  box-sizing: border-box;
  font-family: monospace; /* Console-like font */
}

/* Flex ratios for resizing */
.history-group {
  flex: 0 1 40%; /* 40% width */
  min-width: 200px;
}

.output-group {
  flex: 0 1 60%; /* 60% width */
  min-width: 300px;
}

/* Labels for both groups */
.history-group label,
.output-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: white;
}

legend {
  padding: 5px 10px;
  font-weight: bold;
  font-size: 16px;
  border-radius: 5px;
  color: white;
}

.clear-group {
  display: flex;
  justify-content: center;
  gap: 15px;
}

.detailArea-wrapper {
  position: fixed;
  top: 0;
  right: 0;
  height: 100%;
  display: flex;
  align-items: flex-end;
}

.detailArea {
  position: absolute;
  top: 0;
  right: -1000px; /* Hidden by default */
  height: 100%;
  background-color: #222;
  color: bisque;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 1rem;
  box-shadow: -2px 0 5px rgba(0, 0, 0, 0.5);
  transition: right 0.6s ease; /* Smooth slide effect */
  z-index: 9999;
  overflow-y: auto;
  overflow-x: auto;
}

.detailArea.open {
  right: 0; /* Slide in when open */
}

.loot-table-container {
  justify-content: center; /* Center the table horizontally */
  align-items: center; /* Center the table vertically */
  width: 100%; /* Take up full width */
  margin: 10px 0; 
  max-height: fit-content;
}

.detailedLootContainer {
  background: #242424;
}

.area-close-button {
  align-self: flex-end; /* Pushes the button to the right */
  background: black(192, 0, 0);
  color: wheat;
  border: white;
  padding: 5px 10px;
  cursor: pointer;
  font-size: 20px;
}

.resizer {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 5px;
  background: gray;
  cursor: ew-resize;
}

</style>