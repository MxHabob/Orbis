package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.entity.PlanningEntity
import ai.timefold.solver.core.api.domain.entity.PlanningPin
import ai.timefold.solver.core.api.domain.lookup.PlanningId
import ai.timefold.solver.core.api.domain.variable.PlanningVariable

@PlanningEntity
class LessonAssignment {
    @PlanningId
    var id: String = ""

    var classSubjectId: String = ""
    var classId: String = ""
    var subjectId: String = ""
    var teacherId: String = ""
    var hourIndex: Int = 0

    @PlanningPin
    var pinned: Boolean = false

    @PlanningVariable
    var timeSlot: TimeSlot? = null

    @PlanningVariable
    var dayOfWeek: Int? = null

    @PlanningVariable
    var room: Room? = null
}
