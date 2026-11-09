package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.lookup.PlanningId

class TeacherUnavailability {
    @PlanningId
    var id: String = ""
    var teacherId: String = ""
    var dayOfWeek: Int = 0
    var timeSlot: TimeSlot? = null
}
