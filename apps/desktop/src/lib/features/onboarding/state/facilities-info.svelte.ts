import type { Room } from "../types";
import { getDefaultBuildings } from "../utils";

function createDefaultRooms(building: string): Room[] {
  const rooms: Room[] = [];
  for (let index = 1; index <= 10; index++) {
    rooms.push({
      id: `default-room-${index}`,
      name: `Classroom ${index}`,
      name_ar: `الفصل ${index}`,
      capacity: 30,
      room_type: "classroom",
      building,
      floor: index <= 5 ? 1 : 2,
    });
  }
  rooms.push(
    {
      id: "default-computer-lab-1",
      name: "Computer Lab 1",
      name_ar: "مختبر حاسب 1",
      capacity: 25,
      room_type: "computer_lab",
      building,
      floor: 2,
    },
    {
      id: "default-science-lab-1",
      name: "Science Lab 1",
      name_ar: "مختبر علمي 1",
      capacity: 25,
      room_type: "science_lab",
      building,
      floor: 1,
    },
  );
  return rooms;
}

export class FacilitiesInfoState {
  buildings = $state<string[]>(getDefaultBuildings());
  activeBuildingTab = $state(this.buildings[0] || "");
  rooms = $state<Room[]>(createDefaultRooms(this.activeBuildingTab));

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
    if (Array.isArray(data.rooms) && data.rooms.length > 0) this.rooms = data.rooms;
    if (data.activeBuildingTab) this.activeBuildingTab = data.activeBuildingTab;
    if (Array.isArray(data.buildings) && data.buildings.length > 0) {
      this.buildings = data.buildings;
    }
  }
}
