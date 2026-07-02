<script lang="ts">
import { page } from "$app/state";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { themeStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { Sun, Moon, Laptop, Type, LayoutGrid } from "lucide-svelte";

let fontSize = $state("medium");
let density = $state("comfortable");
let selectedLocale = $state(getLocale());

onMount(() => {
  fontSize = localStorage.getItem("orbis_font_size") || "medium";
  density = localStorage.getItem("orbis_density") || "comfortable";
  selectedLocale = getLocale();
});

function handleThemeChange(theme: "light" | "dark" | "system") {
  themeStore.set(theme);
  toast.success(m.settings_general_save_success());
}

function handleFontSizeChange(size: string) {
  fontSize = size;
  localStorage.setItem("orbis_font_size", size);
  if (typeof document !== "undefined") {
    if (size === "small") {
      document.documentElement.style.fontSize = "13px";
    } else if (size === "medium") {
      document.documentElement.style.fontSize = "14px";
    } else if (size === "large") {
      document.documentElement.style.fontSize = "16px";
    }
  }
  toast.success(m.settings_general_save_success());
}

function handleDensityChange(dens: string) {
  density = dens;
  localStorage.setItem("orbis_density", dens);
  if (typeof document !== "undefined") {
    document.documentElement.setAttribute("data-density", dens);
  }
  toast.success(m.settings_general_save_success());
}

async function handleLanguageChange(lang: string) {
  if (lang !== "ar" && lang !== "en") return;
  if (lang === getLocale()) return;
  selectedLocale = lang;

  // Navigate to the localized URL using paraglide helper
  const localizedUrl = localizeHref(page.url.pathname, { locale: lang });
  await goto(localizedUrl);
  toast.success(m.settings_general_save_success());
}

const locales = [
  { value: "ar", label: m.header_language_ar() },
  { value: "en", label: m.header_language_en() },
];
</script>

<svelte:head>
  <title>{getLocale() === "ar" ? "المظهر" : "Appearance"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Theme Picker -->
  <div class="flex flex-col gap-6 text-start">
    <div class="border-b border-border/60 pb-3">
      <h2 class="text-sm font-semibold text-foreground">{m.settings_appearance_theme()}</h2>
    </div>

    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
      <!-- Light Theme -->
      <button
        type="button"
        class="flex flex-col gap-3 p-4 rounded-xl border text-start transition-all cursor-pointer select-none
          {themeStore.current === 'light' ? 'bg-primary/5 border-primary shadow-[0_0_8px_-2px_var(--color-primary)]' : 'bg-background hover:bg-accent border-border text-muted-foreground'}"
        onclick={() => handleThemeChange("light")}
      >
          <div class="flex items-center gap-2">
            <Sun size={16} class={themeStore.current === 'light' ? 'text-primary' : 'text-muted-foreground'} />
            <span class="text-xs font-semibold text-foreground">{m.settings_appearance_theme_light()}</span>
          </div>
          <!-- Preview Frame -->
          <div class="w-full h-20 rounded bg-zinc-100 border border-zinc-200/80 p-2 flex flex-col gap-1.5">
            <div class="h-3 w-1/3 rounded bg-zinc-300"></div>
            <div class="grid grid-cols-3 gap-1">
              <div class="h-2 rounded bg-zinc-200"></div>
              <div class="h-2 rounded bg-zinc-200 col-span-2"></div>
            </div>
            <div class="h-6 w-full rounded bg-white border border-zinc-200 shadow-sm flex items-center justify-end px-2 mt-auto">
              <div class="h-3 w-10 rounded bg-lime-500"></div>
            </div>
          </div>
        </button>

        <!-- Dark Theme -->
        <button
          type="button"
          class="flex flex-col gap-3 p-4 rounded-xl border text-start transition-all cursor-pointer select-none
            {themeStore.current === 'dark' ? 'bg-primary/5 border-primary shadow-[0_0_8px_-2px_var(--color-primary)]' : 'bg-background hover:bg-accent border-border text-muted-foreground'}"
          onclick={() => handleThemeChange("dark")}
        >
          <div class="flex items-center gap-2">
            <Moon size={16} class={themeStore.current === 'dark' ? 'text-primary' : 'text-muted-foreground'} />
            <span class="text-xs font-semibold text-foreground">{m.settings_appearance_theme_dark()}</span>
          </div>
          <!-- Preview Frame -->
          <div class="w-full h-20 rounded bg-zinc-950 border border-zinc-800 p-2 flex flex-col gap-1.5">
            <div class="h-3 w-1/3 rounded bg-zinc-800"></div>
            <div class="grid grid-cols-3 gap-1">
              <div class="h-2 rounded bg-zinc-900"></div>
              <div class="h-2 rounded bg-zinc-900 col-span-2"></div>
            </div>
            <div class="h-6 w-full rounded bg-zinc-900 border border-zinc-850 flex items-center justify-end px-2 mt-auto">
              <div class="h-3 w-10 rounded bg-lime-500"></div>
            </div>
          </div>
        </button>

        <!-- System Theme -->
        <button
          type="button"
          class="flex flex-col gap-3 p-4 rounded-xl border text-start transition-all cursor-pointer select-none
            {themeStore.current === 'system' ? 'bg-primary/5 border-primary shadow-[0_0_8px_-2px_var(--color-primary)]' : 'bg-background hover:bg-accent border-border text-muted-foreground'}"
          onclick={() => handleThemeChange("system")}
        >
          <div class="flex items-center gap-2">
            <Laptop size={16} class={themeStore.current === 'system' ? 'text-primary' : 'text-muted-foreground'} />
            <span class="text-xs font-semibold text-foreground">{m.settings_appearance_theme_system()}</span>
          </div>
          <!-- Preview Frame -->
          <div class="w-full h-20 rounded overflow-hidden border border-border/80 flex">
            <!-- Left Half (Light) -->
            <div class="w-1/2 h-full bg-zinc-100 p-2 flex flex-col gap-1.5">
              <div class="h-3 w-2/3 rounded bg-zinc-300"></div>
              <div class="h-2 rounded bg-zinc-200"></div>
              <div class="h-4 w-full rounded bg-white mt-auto"></div>
            </div>
            <!-- Right Half (Dark) -->
            <div class="w-1/2 h-full bg-zinc-950 p-2 flex flex-col gap-1.5 border-l border-zinc-800">
              <div class="h-3 w-2/3 rounded bg-zinc-800"></div>
              <div class="h-2 rounded bg-zinc-900"></div>
              <div class="h-4 w-full rounded bg-zinc-900 mt-auto"></div>
            </div>
          </div>
        </button>
      </div>
    </div>

    <!-- Language & Typography -->
    <div class="flex flex-col gap-6 text-start">
      <div class="border-b border-border/60 pb-3">
        <h2 class="text-sm font-semibold text-foreground">{m.settings_appearance_language()}</h2>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Language Selector -->
        <div class="flex flex-col gap-2 text-start">
          <Label for="app_lang" class="text-xs font-medium text-muted-foreground">
            {m.settings_appearance_language()}
          </Label>
          <Select type="single" value={selectedLocale} onValueChange={(v) => handleLanguageChange(v ?? "")}>
            <SelectTrigger class="h-9 w-full">
              {locales.find(loc => loc.value === selectedLocale)?.label || selectedLocale}
            </SelectTrigger>
            <SelectContent>
              {#each locales as loc}
                <SelectItem value={loc.value}>{loc.label}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <!-- Font Size Selector -->
        <div class="flex flex-col gap-2 text-start">
          <Label for="app_font" class="text-xs font-medium text-muted-foreground">
            {m.settings_appearance_font_size()}
          </Label>
          <div class="flex rounded-md border border-border overflow-hidden h-9 w-full">
            {#each [
              { value: "small", label: m.settings_appearance_font_small() },
              { value: "medium", label: m.settings_appearance_font_medium() },
              { value: "large", label: m.settings_appearance_font_large() }
            ] as item}
              <button
                type="button"
                class="flex-1 flex items-center justify-center text-xs font-semibold cursor-pointer transition-all select-none
                  {fontSize === item.value ? 'bg-accent text-accent-foreground font-bold' : 'bg-background hover:bg-accent/40 text-muted-foreground'}"
                onclick={() => handleFontSizeChange(item.value)}
              >
                {item.label}
              </button>
            {/each}
          </div>
        </div>
      </div>
    </div>

    <!-- Interface Layout/Density -->
    <div class="flex flex-col gap-6 text-start">
      <div class="border-b border-border/60 pb-3">
        <h2 class="text-sm font-semibold text-foreground">{m.settings_appearance_density()}</h2>
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
        {#each [
          { value: "compact", label: m.settings_appearance_density_compact(), desc: "More elements, less whitespace" },
          { value: "comfortable", label: m.settings_appearance_density_comfortable(), desc: "Perfect balance for everyday work" },
          { value: "spacious", label: m.settings_appearance_density_spacious(), desc: "Larger spacing, ideal for touch screen" }
        ] as item}
          <button
            type="button"
            class="flex flex-col gap-2 p-4 rounded-xl border text-start transition-all cursor-pointer select-none
              {density === item.value ? 'bg-primary/5 border-primary shadow-[0_0_8px_-2px_var(--color-primary)]' : 'bg-background hover:bg-accent border-border text-muted-foreground'}"
            onclick={() => handleDensityChange(item.value)}
          >
            <div class="flex items-center gap-2">
              <LayoutGrid size={15} class={density === item.value ? 'text-primary' : 'text-muted-foreground'} />
              <span class="text-xs font-semibold text-foreground">{item.label}</span>
            </div>
            <p class="text-[10px] text-muted-foreground leading-normal">{item.desc}</p>
          </button>
        {/each}
      </div>
    </div>
</div>
