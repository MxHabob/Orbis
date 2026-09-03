package ai.orbis.solver.dto

import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.databind.annotation.JsonNaming

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ExamProblemDto(
    val jobId: String,
    val orgId: String,
    val examSessionId: String,
    val slots: List<ExamTimeSlotDto>,
    val assignments: List<ExamAssignmentDto>,
    val maxExamsPerDay: Int = 2,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ExamTimeSlotDto(
    val id: String,
    val date: String,
    val period: String,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ExamAssignmentDto(
    val id: String,
    val examSubjectId: String,
    val subjectId: String,
    val classId: String,
    val conflictGroupId: Int? = null,
    val slotId: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ExamSolutionDto(
    val jobId: String,
    val assignments: List<ExamAssignmentDto>,
    val score: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class ExamSolveJobStatusDto(
    val jobId: String,
    val status: String,
    val score: String? = null,
    val bestSolution: ExamSolutionDto? = null,
    val error: String? = null,
)
