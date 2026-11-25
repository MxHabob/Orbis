import type { Component } from "svelte";

export type SearchItem = {
  id: string;
  title: string;
  subtitle?: string;
  icon: Component<{ class?: string; size?: number }>;
  action: () => void;
  group: string;
};

export type SearchItemGroup = {
  label: string;
  items: SearchItem[];
};

export type SearchResultIcon = SearchItem["icon"];
