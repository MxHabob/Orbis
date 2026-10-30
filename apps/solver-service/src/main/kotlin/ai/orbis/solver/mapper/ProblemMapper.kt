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
import java.time.LocalTime

object ProblemMapper {
    fun validate(problem: TimetableProblemDto) {
        require(problem.jobId.isNotBlank()) { "job_id is required" }
        require(problem.orgId.isNotBlank()) { "org_id is required" }
        require(problem.cycleId.isNotBlank()) { "period_id is required" }

        val slotIds = problem.timeSlots.map { it.id }
        require(slotIds.all { it.isNotBlank() } && slotIds.distinct().size == slotIds.size) {
            "Time slot IDs must be non-empty and unique"
        }
        val slotsById = problem.timeSlots.associateBy { it.id }
        val teachingSlots = problem.timeSlots.filterNot { it.isBreak }
        require(teachingSlots.isNotEmpty()) { "At least one teaching time slot is required" }
        for (slot in problem.timeSlots) {
            val start = runCatching { LocalTime.parse(slot.startTime) }.getOrElse {
                throw IllegalArgumentException("Invalid time slot start time: ${slot.id}")
            }
            val end = runCatching { LocalTime.parse(slot.endTime) }.getOrElse {
                throw IllegalArgumentException("Invalid time slot end time: ${slot.id}")
            }
            require(start < end) { "Time slot end must be after start: ${slot.id}" }
            if (!slot.isBreak) {
                require(slot.slotNumber > 0) { "Teaching time slot order must be positive" }
                val days = slot.applicableDays.orEmpty()
                require(days.isNotEmpty() && days.distinct().size == days.size &&
                    days.all { it in 1..7 }
                ) { "Each teaching time slot must have valid applicable days (1-7)" }
            }
        }

        val roomIds = problem.rooms.map { it.id }
        require(roomIds.all { it.isNotBlank() } && roomIds.distinct().size == roomIds.size) {
            "Room IDs must be non-empty and unique"
        }
        val roomIdsByValue = roomIds.toSet()

        require(problem.assignments.isNotEmpty()) {
            "At least one unassigned lesson is required"
        }
        require(problem.mode == "full" || problem.mode == "incremental") {
            "mode must be full or incremental"
        }
        val assignmentIds = problem.assignments.map { it.id } + problem.pinnedLessons.map { it.id }
        require(assignmentIds.all { it.isNotBlank() } &&
            assignmentIds.distinct().size == assignmentIds.size
        ) { "Lesson assignment IDs must be non-empty and unique" }

        for (assignment in problem.assignments) {
            require(assignment.sectionSubjectId.isNotBlank() &&
                assignment.sectionId.isNotBlank() &&
                assignment.subjectId.isNotBlank() &&
                assignment.teacherId.isNotBlank() &&
                assignment.hourIndex >= 0
            ) { "Each lesson assignment must include a section, subject, teacher, and valid hour index" }
        }

        for (pinned in problem.pinnedLessons) {
            require(pinned.sectionSubjectId.isNotBlank() &&
                pinned.sectionId.isNotBlank() &&
                pinned.subjectId.isNotBlank() &&
                pinned.teacherId.isNotBlank()
            ) { "Each pinned lesson must include a section, subject, and teacher" }
            val slot = slotsById[pinned.timeSlotId]
                ?: throw IllegalArgumentException("Pinned lesson references an unknown time slot")
            require(!slot.isBreak &&
                pinned.dayOfWeek in slot.applicableDays.orEmpty()
            ) { "Pinned lesson day must match an available teaching time slot" }
            require(pinned.roomId == null || pinned.roomId in roomIdsByValue) {
                "Pinned lesson references an unknown room"
            }
        }

        for (unavailable in problem.unavailableSlots) {
            require(unavailable.teacherId.isNotBlank() && unavailable.dayOfWeek in 1..7) {
                "Teacher unavailability must include a teacher and a valid day (1-7)"
            }
            val slot = slotsById[unavailable.timeSlotId]
                ?: throw IllegalArgumentException("Teacher unavailability references an unknown time slot")
            require(!slot.isBreak) { "Teacher unavailability must reference a teaching time slot" }
            require(unavailable.dayOfWeek in slot.applicableDays.orEmpty()) {
                "Teacher unavailability day must match the time slot"
            }
        }
    }

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
        validate(problem)
        val slotById = problem.timeSlots.associate { dto ->
            val start = LocalTime.parse(dto.startTime)
            val end = LocalTime.parse(dto.endTime)
            dto.id to TimeSlot().apply {
                id = dto.id
                slotNumber = dto.slotNumber
                startTime = dto.startTime
                endTime = dto.endTime
                startMinuteOfDay = start.hour * 60 + start.minute
                endMinuteOfDay = end.hour * 60 + end.minute
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
            days = teachingSlots.flatMap { it.applicableDays }.distinct().sorted()
            assignments = pinned + open
            unavailableSlots = problem.unavailableSlots.mapIndexed { index, dto ->
                TeacherUnavailability().apply {
                    id = "${dto.teacherId}-${dto.dayOfWeek}-${dto.timeSlotId}-$index"
                    teacherId = dto.teacherId
                    dayOfWeek = dto.dayOfWeek
                    timeSlot = slotById[dto.timeSlotId]
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
        sectionSubjectId = pinned.sectionSubjectId
        sectionId = pinned.sectionId
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
        sectionSubjectId = dto.sectionSubjectId
        sectionId = dto.sectionId
        subjectId = dto.subjectId
        teacherId = dto.teacherId
        hourIndex = dto.hourIndex
        pinned = false
    }

    private fun toDto(entity: LessonAssignment): LessonAssignmentDto = LessonAssignmentDto(
        id = entity.id,
        sectionSubjectId = entity.sectionSubjectId,
        sectionId = entity.sectionId,
        subjectId = entity.subjectId,
        teacherId = entity.teacherId,
        hourIndex = entity.hourIndex,
        timeSlotId = entity.timeSlot?.id,
        dayOfWeek = entity.dayOfWeek,
        roomId = entity.room?.id,
    )
}

fun HardSoftScore?.isFeasible(): Boolean = this != null && this.hardScore == 0
