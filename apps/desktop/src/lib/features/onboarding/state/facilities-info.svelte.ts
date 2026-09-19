import type { Room } from "../types";
import { getDefaultBuildings } from "../utils";

function createDefaultRooms(building: string): Room[] {
  const rooms: Room[] = [];
  for (let index = 1; index <= 8; index++) {
    rooms.push({
      id: `default-room-${index}`,
      name: `Classroom ${index}`,
      name_ar: `الفصل ${index}`,
      capacity: 30,
      room_type: "classroom",
      building,
      floor: index <= 4 ? 1 : 2,
    });
  }
  rooms.push(
    {
      id: "default-science-lab",
      name: "Science Lab",
      name_ar: "المختبر العلمي",
      capacity: 25,
      room_type: "science_lab",
      building,
      floor: 1,
    },
    {
      id: "default-computer-lab",
      name: "Computer Lab",
      name_ar: "معمل الحاسوب",
      capacity: 25,
      room_type: "computer_lab",
      building,
      floor: 2,
    },
    {
      id: "default-library",
      name: "Library & Learning Resources",
      name_ar: "المكتبة ومصادر التعلم",
      capacity: 40,
      room_type: "library",
      building,
      floor: 1,
    },
    {
      id: "default-activity-hall",
      name: "Activity Hall",
      name_ar: "قاعة الأنشطة والفعاليات",
      capacity: 60,
      room_type: "activity_hall",
      building,
      floor: 1,
    },
    {
      id: "default-virtual-classroom",
      name: "Virtual Classroom (Online)",
      name_ar: "القاعة الافتراضية (تعليم عن بُعد)",
      capacity: 100,
      room_type: "virtual_classroom",
      building,
      floor: 0,
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
    if (Array.isArray(data.rooms) && data.rooms.length > 0)
      this.rooms = data.rooms;
    if (data.activeBuildingTab) this.activeBuildingTab = data.activeBuildingTab;
    if (Array.isArray(data.buildings) && data.buildings.length > 0) {
      this.buildings = data.buildings;
    }
  }
}
