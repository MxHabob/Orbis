package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.lookup.PlanningId

class TimeSlot {
    @PlanningId
    var id: String = ""
    var slotNumber: Int = 0
    var startTime: String = ""
    var endTime: String = ""
    var isBreak: Boolean = false
    var applicableDays: MutableSet<Int> = mutableSetOf()
}

class Room {
    @PlanningId
    var id: String = ""
    var name: String = ""
    var capacity: Int? = null
    var roomType: String = "classroom"
}
