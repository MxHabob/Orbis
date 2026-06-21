package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.solution.PlanningEntityCollectionProperty
import ai.timefold.solver.core.api.domain.solution.PlanningScore
import ai.timefold.solver.core.api.domain.solution.PlanningSolution
import ai.timefold.solver.core.api.domain.solution.ProblemFactCollectionProperty
import ai.timefold.solver.core.api.domain.valuerange.ValueRangeProvider
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore

@PlanningSolution
class Timetable {
    var jobId: String = ""

    @ProblemFactCollectionProperty
    @ValueRangeProvider
    var timeSlots: List<TimeSlot> = emptyList()

    @ProblemFactCollectionProperty
    @ValueRangeProvider
    var rooms: List<Room> = emptyList()

    @ValueRangeProvider
    var days: List<Int> = listOf(0, 1, 2, 3, 4, 5, 6)

    @PlanningEntityCollectionProperty
    var assignments: List<@JvmSuppressWildcards LessonAssignment> = emptyList()

    @ProblemFactCollectionProperty
    var unavailableSlots: List<TeacherUnavailability> = emptyList()

    @PlanningScore
    var score: HardSoftScore? = null
}
