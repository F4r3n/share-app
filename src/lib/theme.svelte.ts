import { writable, type Writable } from 'svelte/store';
import type { Theme_Name, Theme_Mode } from '../lib/config';
// Theme Generator Live Preview State
export let theme_name : Writable<Theme_Name> = writable("modern");
export let theme_mode: Writable<Theme_Mode> = writable("light");
