import type { Room } from "../types";
import { getDefaultBuildings } from "../utils";

export class FacilitiesInfoState {
  rooms = $state<Room[]>([]);
  activeBuildingTab = $state(getDefaultBuildings()[0]);
  buildings = $state<string[]>(getDefaultBuildings());

  get totalCapacity() {
    return this.rooms.reduce((acc, r) => acc + r.capacity, 0);
  }

  serialize() {
    return {
      rooms: this.rooms,
      activeBuildingTab: this.activeBuildingTab,
      buildings: this.buildings,
    };
  }

  deserialize(data: any) {
    if (!data) return;
    if (data.rooms) this.rooms = data.rooms;
    if (data.activeBuildingTab) this.activeBuildingTab = data.activeBuildingTab;
    if (data.buildings) this.buildings = data.buildings;
  }
}
