import {ref, computed } from 'vue';

export const collapsed = ref(false);
export const toggleSidebar = () => (collapsed.value =!collapsed.value);;

export const SIDEBAR_WIDTH = 180;
export const SIDEBAR_WIDTH_COLLAPSED = 60;
export const sidebarWidth = computed(() => `${collapsed.value ? SIDEBAR_WIDTH_COLLAPSED : SIDEBAR_WIDTH}px`);

export enum NavbarMenus {
    HOME,
    CHARACTERS,
    LOOT,
    SPELLS,
    SETTINGS
}

export const currentMenu = ref(NavbarMenus.HOME);
