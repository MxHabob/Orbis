package ai.orbis.solver.domain

import ai.timefold.solver.core.api.domain.solution.PlanningEntityCollectionProperty
import ai.timefold.solver.core.api.domain.solution.PlanningScore
import ai.timefold.solver.core.api.domain.solution.PlanningSolution
import ai.timefold.solver.core.api.domain.solution.ProblemFactCollectionProperty
import ai.timefold.solver.core.api.domain.valuerange.ValueRangeProvider
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore

@PlanningSolution
class ExamSchedule {
    var jobId: String = ""

    @ProblemFactCollectionProperty
    @ValueRangeProvider
    var slots: List<ExamTimeSlot> = emptyList()

    @PlanningEntityCollectionProperty
    var assignments: List<@JvmSuppressWildcards ExamAssignment> = emptyList()

    var maxExamsPerDay: Int = 2

    @PlanningScore
    var score: HardSoftScore? = null
}
