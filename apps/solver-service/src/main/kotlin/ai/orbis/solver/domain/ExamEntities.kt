package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.entity.PlanningEntity
import ai.timefold.solver.core.api.domain.lookup.PlanningId
import ai.timefold.solver.core.api.domain.variable.PlanningVariable

class ExamTimeSlot {
    @PlanningId
    var id: String = ""
    var date: String = ""
    var period: String = "" // "morning" or "afternoon"
}

@PlanningEntity
class ExamAssignment {
    @PlanningId
    var id: String = ""
    var examSubjectId: String = ""
    var subjectId: String = ""
    var classId: String = ""
    var conflictGroupId: Int? = null

    @PlanningVariable
    var slot: ExamTimeSlot? = null
}
