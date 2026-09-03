package ai.orbis.solver.mapper

import ai.orbis.solver.domain.LessonAssignment
import ai.orbis.solver.domain.Room
import ai.orbis.solver.domain.TeacherUnavailability
import ai.orbis.solver.domain.TimeSlot
import ai.orbis.solver.domain.Timetable
import ai.orbis.solver.dto.LessonAssignmentDto
import ai.orbis.solver.dto.PinnedLessonDto
import ai.orbis.solver.dto.TimetableProblemDto
import ai.orbis.solver.dto.TimetableSolutionDto
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore

object ProblemMapper {

    fun toSolution(problem: TimetableProblemDto, timetable: Timetable): TimetableSolutionDto {
        val openOnly = timetable.assignments
            .filter { !it.pinned }
            .map { toDto(it) }
        return TimetableSolutionDto(
            jobId = problem.jobId,
            assignments = openOnly,
            score = timetable.score?.toString(),
        )
    }

    fun toTimetable(problem: TimetableProblemDto): Timetable {
        val slotById = problem.timeSlots.associate { dto ->
            dto.id to TimeSlot().apply {
                id = dto.id
                slotNumber = dto.slotNumber
                startTime = dto.startTime
                endTime = dto.endTime
                isBreak = dto.isBreak
                applicableDays = (dto.applicableDays ?: emptyList()).toMutableSet()
            }
        }

        val roomById = problem.rooms.associate { dto ->
            dto.id to Room().apply {
                id = dto.id
                name = dto.name
                capacity = dto.capacity
                roomType = dto.roomType
            }
        }

        val teachingSlots = slotById.values.filter { !it.isBreak }

        val pinned = problem.pinnedLessons.map { toPinnedAssignment(it, slotById, roomById) }
        val open = problem.assignments.map { toOpenAssignment(it) }

        return Timetable().apply {
            jobId = problem.jobId
            timeSlots = teachingSlots
            rooms = roomById.values.toList()
            assignments = pinned + open
            unavailableSlots = problem.unavailableSlots.mapIndexed { index, dto ->
                TeacherUnavailability().apply {
                    id = "${dto.teacherId}-${dto.dayOfWeek}-${dto.timeSlotId}-$index"
                    teacherId = dto.teacherId
                    dayOfWeek = dto.dayOfWeek
                    timeSlotId = dto.timeSlotId
                }
            }
        }
    }

    private fun toPinnedAssignment(
        pinned: PinnedLessonDto,
        slots: Map<String, TimeSlot>,
        rooms: Map<String, Room>,
    ): LessonAssignment = LessonAssignment().apply {
        id = pinned.id
        classSubjectId = pinned.classSubjectId
        classId = pinned.classId
        subjectId = pinned.subjectId
        teacherId = pinned.teacherId
        hourIndex = 0
        this.pinned = true
        timeSlot = slots[pinned.timeSlotId]
        dayOfWeek = pinned.dayOfWeek
        room = pinned.roomId?.let { rooms[it] }
    }

    private fun toOpenAssignment(dto: LessonAssignmentDto): LessonAssignment = LessonAssignment().apply {
        id = dto.id
        classSubjectId = dto.classSubjectId
        classId = dto.classId
        subjectId = dto.subjectId
        teacherId = dto.teacherId
        hourIndex = dto.hourIndex
        pinned = false
    }

    private fun toDto(entity: LessonAssignment): LessonAssignmentDto = LessonAssignmentDto(
        id = entity.id,
        classSubjectId = entity.classSubjectId,
        classId = entity.classId,
        subjectId = entity.subjectId,
        teacherId = entity.teacherId,
        hourIndex = entity.hourIndex,
        timeSlotId = entity.timeSlot?.id,
        dayOfWeek = entity.dayOfWeek,
        roomId = entity.room?.id,
    )
}

fun HardSoftScore?.isFeasible(): Boolean = this != null && this.hardScore >= 0
