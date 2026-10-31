package ai.orbis.solver.dto

import com.fasterxml.jackson.annotation.JsonAlias
import com.fasterxml.jackson.databind.PropertyNamingStrategies
import com.fasterxml.jackson.databind.annotation.JsonNaming

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class TimetableProblemDto(
    val jobId: String,
    val orgId: String,
    val cycleId: String,
    val mode: String,
    val timeSlots: List<TimeSlotDto>,
    val rooms: List<RoomDto>,
    val assignments: List<LessonAssignmentDto>,
    val pinnedLessons: List<PinnedLessonDto> = emptyList(),
    val unavailableSlots: List<TeacherUnavailableDto> = emptyList(),
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class TeacherUnavailableDto(
    val teacherId: String,
    val dayOfWeek: Int,
    val timeSlotId: String,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class TimeSlotDto(
    val id: String,
    val slotNumber: Int,
    val startTime: String,
    val endTime: String,
    val isBreak: Boolean,
    val applicableDays: List<Int>? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class RoomDto(
    val id: String,
    val name: String,
    val capacity: Int? = null,
    val roomType: String,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class LessonAssignmentDto(
    val id: String,
    @JsonAlias("class_subject_id")
    val sectionSubjectId: String,
    @JsonAlias("class_id")
    val sectionId: String,
    val subjectId: String,
    val teacherId: String,
    val hourIndex: Int,
    val timeSlotId: String? = null,
    val dayOfWeek: Int? = null,
    val roomId: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class PinnedLessonDto(
    val id: String,
    @JsonAlias("class_id")
    val sectionId: String,
    val teacherId: String,
    val subjectId: String,
    @JsonAlias("class_subject_id")
    val sectionSubjectId: String,
    val timeSlotId: String,
    val dayOfWeek: Int,
    val roomId: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class SolveJobStatusDto(
    val jobId: String,
    val status: String,
    val score: String? = null,
    val bestSolution: TimetableSolutionDto? = null,
    val error: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class TimetableSolutionDto(
    val jobId: String,
    val assignments: List<LessonAssignmentDto>,
    val score: String? = null,
)

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy::class)
data class SubmitResponseDto(
    val jobId: String,
    val accepted: Boolean,
)

enum class JobStatus { pending, running, completed, failed, cancelled }
