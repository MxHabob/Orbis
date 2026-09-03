package ai.orbis.solver.constraints

import ai.orbis.solver.domain.ExamAssignment
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore
import ai.timefold.solver.core.api.score.stream.Constraint
import ai.timefold.solver.core.api.score.stream.ConstraintFactory
import ai.timefold.solver.core.api.score.stream.ConstraintProvider
import ai.timefold.solver.core.api.score.stream.Joiners
import ai.timefold.solver.core.api.score.stream.ConstraintCollectors

class ExamConstraintProvider : ConstraintProvider {
    override fun defineConstraints(factory: ConstraintFactory): Array<Constraint> = arrayOf(
        classConflict(factory),
        conflictGroupConflict(factory),
        maxExamsPerDay(factory),
    )

    private fun classConflict(factory: ConstraintFactory): Constraint =
        factory.forEachUniquePair(
            ExamAssignment::class.java,
            Joiners.equal(ExamAssignment::classId),
            Joiners.equal(ExamAssignment::slot),
        )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Class exam slot conflict")

    private fun conflictGroupConflict(factory: ConstraintFactory): Constraint =
        factory.forEachUniquePair(
            ExamAssignment::class.java,
            Joiners.equal(ExamAssignment::conflictGroupId),
            Joiners.filtering { a, b ->
                a.conflictGroupId != null && a.slot != null && b.slot != null && a.slot!!.date == b.slot!!.date
            }
        )
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Conflict group date conflict")

    private fun maxExamsPerDay(factory: ConstraintFactory): Constraint =
        factory.forEach(ExamAssignment::class.java)
            .groupBy(
                { a -> a.slot?.date },
                ConstraintCollectors.count()
            )
            .filter { date, count ->
                date != null && count > 2
            }
            .penalize(HardSoftScore.ONE_HARD)
            .asConstraint("Max exams per day limit")
}
