package ai.orbis.solver.constraints

import ai.orbis.solver.domain.LessonAssignment
import ai.orbis.solver.domain.TeacherUnavailability
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore
import ai.timefold.solver.core.api.score.stream.Constraint
import ai.timefold.solver.core.api.score.stream.ConstraintFactory
import ai.timefold.solver.core.api.score.stream.ConstraintProvider
import ai.timefold.solver.core.api.score.stream.Joiners

class TimetableConstraintProvider : ConstraintProvider {

    override fun defineConstraints(factory: ConstraintFactory): Array<Constraint> = arrayOf(
        teacherConflict(factory),
        classConflict(factory),
        roomConflict(factory),
        noLessonDuringBreak(factory),
        teacherUnavailableOnBreakSlot(factory),
        teacherUnavailableSlot(factory),
    )

    private fun teacherConflict(factory: ConstraintFactory): Constraint =
        factory.forEachUniquePair(
            LessonAssignment::class.java,
            Joiners.equal(LessonAssignment::getTeacherId),
            Joiners.equal(LessonAssignment::getTimeSlot),
            Joiners.equal(LessonAssignment::getDayOfWeek),
        )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Teacher conflict")

    private fun classConflict(factory: ConstraintFactory): Constraint =
        factory.forEachUniquePair(
            LessonAssignment::class.java,
            Joiners.equal(LessonAssignment::getClassId),
            Joiners.equal(LessonAssignment::getTimeSlot),
            Joiners.equal(LessonAssignment::getDayOfWeek),
        )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Class conflict")

    private fun roomConflict(factory: ConstraintFactory): Constraint =
        factory.forEachUniquePair(
            LessonAssignment::class.java,
            Joiners.equal(LessonAssignment::getRoom),
            Joiners.equal(LessonAssignment::getTimeSlot),
            Joiners.equal(LessonAssignment::getDayOfWeek),
        )
            .filter { a, b -> a.room != null && b.room != null }
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Room conflict")

    private fun noLessonDuringBreak(factory: ConstraintFactory): Constraint =
        factory.forEach(LessonAssignment::class.java)
            .filter { it.timeSlot?.isBreak == true }
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("No lesson during break")

    private fun teacherUnavailableOnBreakSlot(factory: ConstraintFactory): Constraint =
        factory.forEach(LessonAssignment::class.java)
            .filter { assignment ->
                val slot = assignment.timeSlot ?: return@filter false
                val day = assignment.dayOfWeek ?: return@filter false
                slot.applicableDays.isNotEmpty() && day !in slot.applicableDays
            }
            .penalize(HardSoftScore.ONE_SOFT)
            .asConstraint("Prefer applicable days")

    private fun teacherUnavailableSlot(factory: ConstraintFactory): Constraint =
        factory.forEach(LessonAssignment::class.java)
            .join(
                TeacherUnavailability::class.java,
                Joiners.equal(LessonAssignment::getTeacherId, TeacherUnavailability::getTeacherId),
                Joiners.equal(LessonAssignment::getDayOfWeek, TeacherUnavailability::getDayOfWeek),
                Joiners.filtering { assignment, unavail ->
                    assignment.timeSlot?.id == unavail.timeSlotId
                },
            )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Teacher unavailable")
}
