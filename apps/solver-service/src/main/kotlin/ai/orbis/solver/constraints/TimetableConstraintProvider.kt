package ai.orbis.solver.constraints

import ai.orbis.solver.domain.LessonAssignment
import ai.orbis.solver.domain.TeacherUnavailability
import ai.orbis.solver.domain.TimeSlot
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore
import ai.timefold.solver.core.api.score.stream.Constraint
import ai.timefold.solver.core.api.score.stream.ConstraintFactory
import ai.timefold.solver.core.api.score.stream.ConstraintProvider
import ai.timefold.solver.core.api.score.stream.Joiners

class TimetableConstraintProvider : ConstraintProvider {

    override fun defineConstraints(factory: ConstraintFactory): Array<Constraint> = arrayOf(
        teacherConflict(factory),
        sectionConflict(factory),
        roomConflict(factory),
        noLessonDuringBreak(factory),
        unavailableSlotDay(factory),
        teacherUnavailableSlot(factory),
    )

    private fun teacherConflict(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .join(
                factory.forEachIncludingUnassigned(LessonAssignment::class.java),
                Joiners.lessThan(LessonAssignment::id, LessonAssignment::id),
                Joiners.equal(LessonAssignment::teacherId, LessonAssignment::teacherId),
                Joiners.equal(LessonAssignment::dayOfWeek, LessonAssignment::dayOfWeek),
                Joiners.filtering { first, second -> overlaps(first, second) },
            )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Teacher conflict")

    private fun sectionConflict(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .join(
                factory.forEachIncludingUnassigned(LessonAssignment::class.java),
                Joiners.lessThan(LessonAssignment::id, LessonAssignment::id),
                Joiners.equal(LessonAssignment::sectionId, LessonAssignment::sectionId),
                Joiners.equal(LessonAssignment::dayOfWeek, LessonAssignment::dayOfWeek),
                Joiners.filtering { first, second -> overlaps(first, second) },
            )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Section conflict")

    private fun roomConflict(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .join(
                factory.forEachIncludingUnassigned(LessonAssignment::class.java),
                Joiners.lessThan(LessonAssignment::id, LessonAssignment::id),
                Joiners.equal(LessonAssignment::room, LessonAssignment::room),
                Joiners.equal(LessonAssignment::dayOfWeek, LessonAssignment::dayOfWeek),
                Joiners.filtering { first, second ->
                    first.room != null && second.room != null && overlaps(first, second)
                },
            )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Room conflict")

    private fun noLessonDuringBreak(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .filter { it.timeSlot?.isBreak == true }
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("No lesson during break")

    private fun unavailableSlotDay(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .filter { assignment ->
                val slot = assignment.timeSlot ?: return@filter false
                val day = assignment.dayOfWeek ?: return@filter false
                day !in slot.applicableDays
            }
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Time slot unavailable on selected day")

    private fun teacherUnavailableSlot(factory: ConstraintFactory): Constraint =
        factory.forEachIncludingUnassigned(LessonAssignment::class.java)
            .join(
                TeacherUnavailability::class.java,
                Joiners.equal(LessonAssignment::teacherId, TeacherUnavailability::teacherId),
                Joiners.equal(LessonAssignment::dayOfWeek, TeacherUnavailability::dayOfWeek),
                Joiners.filtering { assignment, unavail ->
                    overlaps(assignment.timeSlot, unavail.timeSlot)
                },
            )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Teacher unavailable")

    private fun overlaps(first: LessonAssignment, second: LessonAssignment): Boolean {
        return overlaps(first.timeSlot, second.timeSlot)
    }

    private fun overlaps(firstSlot: TimeSlot?, secondSlot: TimeSlot?): Boolean {
        if (firstSlot == null || secondSlot == null) return false
        return firstSlot.startMinuteOfDay < secondSlot.endMinuteOfDay &&
            secondSlot.startMinuteOfDay < firstSlot.endMinuteOfDay
    }
}
