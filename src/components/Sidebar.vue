<template>
    <div class="sidebar" :style="{width: sidebarWidth}">
        <h1 @click="toggleSidebar" class="sidebar-header">
            <span v-if="collapsed">
                <div>A</div>
                <div>V</div>

            </span>
            <span v-else>Adventure World</span>
        </h1>

        <div >
                <div class="sidebar-element" @click="currentMenu = NavbarMenus.HOME" :class="{ active: currentMenu === NavbarMenus.HOME }">
                    <i class="fa-solid fa-map"></i> <!--map icon-->
                <span v-if="!collapsed">Home</span>
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.CHARACTERS" :class="{ active: currentMenu === NavbarMenus.CHARACTERS }">
                <i class="fa-solid fa-user-large"></i> <!--user icon-->
                <span v-if="!collapsed">Characters</span>
                
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.LOOT" :class="{ active: currentMenu === NavbarMenus.LOOT }">
                <i class="fa-solid fa-gem"></i> <!--gem icon-->
                <span v-if="!collapsed">Loot</span>
                
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.DATABASE" :class="{ active: currentMenu === NavbarMenus.DATABASE }">
                <i class="fa-solid fa-database"></i> <!--Database icon-->
                <span v-if="!collapsed">Database</span>
                
            </div>

            <div class="sidebar-element" @click="currentMenu = NavbarMenus.SETTINGS" :class="{ active: currentMenu === NavbarMenus.SETTINGS }">
                
                <i class="fa-solid fa-gear"></i> <!--settings icon-->
                <span v-if="!collapsed">Settings</span>
                
            </div>
        </div>

    

        <span class="collapsed-icon" @click="toggleSidebar">
            <i class="fa-solid fa-bars"></i>
        </span>
    </div>
</template>


<script setup lang="ts">
import { collapsed, toggleSidebar, sidebarWidth, NavbarMenus, currentMenu } from './SidebarState';

</script>


<style>
:root {
    --sidebar-bg-color: #333;
    --sidebar-item-hover: #555;
    --sidebar-item-active: #444;
}
</style>

<style scoped>
.sidebar {
    color: black;
    background-color: var(--sidebar-bg-color);
    float: left;
    position: fixed;
    z-index: 1;;
    top: 0;
    left: 0;
    bottom: 0;
    padding: 0.5em;

    transition: 0.3s ease;

    display: flex;
    flex-direction: column;
    align-items: flex-start;
    box-shadow: 2px 0 10px rgba(0, 0, 0, 0.2);
        /* Ensure the sidebar elements are properly contained */
        overflow: hidden;  /* Prevents elements from overflowing */
    box-sizing: border-box; /* Ensures padding is accounted for */
}

/* Title at the top */
.sidebar h1 {
    font-size: 1.5rem;
    font-weight: bold;
    margin: 0;
    color: var(--sidebar-text-color);
    cursor: pointer;
    padding: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: color 0.3s;
}

.sidebar-header{
    display: flex;
    justify-content: center; /* Center the items horizontally */
    align-items: center; /* Center the items vertically */
    gap: 10px; /* Space between the "A" and "V" */
    width: 100%; /* Ensure the header takes up the full width */
}

.sidebar-element {
    display: flex;
    align-items: left;
    padding: 20px;
    font-size: 1.1rem;
    font-weight: bold;
    gap: 15px;
    cursor: pointer;
    transition: background-color 0.3s, color 0.3s;
    width: 100%;
    box-sizing: border-box; /* Ensures proper box sizing inside the sidebar */
    margin-left: -5px;
}

/* Collapsed sidebar icon rotation */
.collapsed-icon {
    position: absolute;
    bottom: 20px;
    left: 50%;
    transform: translateX(-50%);
    cursor: pointer;
    padding: 0.75em;
    color: var(--sidebar-text-color-inactive);
    font-size: 1.5rem;
    transition: color 0.3s ease;
}

.sidebar-element span {
    transition: opacity 0.3s ease;
}


/* Active Menu Item */
.sidebar-element.active {
    background-color: var(--sidebar-item-active);
    color: var(--sidebar-text-color);
}

.collapsed .collapsed-icon {
    left: 50%;
}

.collapsed-icon:hover {
    color: var(--sidebar-text-color);
 
}

.sidebar-element:hover {
    background-color: var(--sidebar-item-hover);
    transform: scale(1.05);
    transition: background 0.3s ease, transform 0.2s ease;
}

/* Icon size */
.sidebar-element i {
    font-size: 1.5rem;
    width: 30px;
    text-align: center;
    color: var(--sidebar-text-color);
}

/* Hide text when collapsed */
.collapsed .sidebar-element {
    justify-content: center;
    padding-left: 10px;  /* Reduce left padding */
    margin-left: 0; /* Remove left margin */
    margin-right: 0; /* Remove right margin */
}

.collapsed .sidebar-element i {
    margin-right: 0;
    font-size: 1rem; /* Smaller icon size in collapsed state */
}


.collapsed .sidebar-element i + span {
    display: none; /* Hide text */
}

/* When collapsed, hide text */
.collapsed .sidebar-element span {
    opacity: 0;
}

.rotate-180 {
    transform: rotate(180deg);
    transition: 0.2s linear;
}
</style>