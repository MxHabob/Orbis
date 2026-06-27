package ai.orbis.solver.mapper

import ai.orbis.solver.domain.ExamAssignment
import ai.orbis.solver.domain.ExamSchedule
import ai.orbis.solver.domain.ExamTimeSlot
import ai.orbis.solver.dto.ExamAssignmentDto
import ai.orbis.solver.dto.ExamProblemDto
import ai.orbis.solver.dto.ExamSolutionDto

object ExamProblemMapper {

    fun toSolution(problem: ExamProblemDto, schedule: ExamSchedule): ExamSolutionDto {
        val mapped = schedule.assignments.map { toDto(it) }
        return ExamSolutionDto(
            jobId = problem.jobId,
            assignments = mapped,
            score = schedule.score?.toString(),
        )
    }

    fun toSchedule(problem: ExamProblemDto): ExamSchedule {
        val slotById = problem.slots.associate { dto ->
            dto.id to ExamTimeSlot().apply {
                id = dto.id
                date = dto.date
                period = dto.period
            }
        }

        val assignmentsList = problem.assignments.map { dto ->
            ExamAssignment().apply {
                id = dto.id
                examSubjectId = dto.examSubjectId
                subjectId = dto.subjectId
                classId = dto.classId
                conflictGroupId = dto.conflictGroupId
                slot = dto.slotId?.let { slotById[it] }
            }
        }

        return ExamSchedule().apply {
            jobId = problem.jobId
            slots = slotById.values.toList()
            assignments = assignmentsList
            maxExamsPerDay = problem.maxExamsPerDay
        }
    }

    private fun toDto(entity: ExamAssignment): ExamAssignmentDto = ExamAssignmentDto(
        id = entity.id,
        examSubjectId = entity.examSubjectId,
        subjectId = entity.subjectId,
        classId = entity.classId,
        conflictGroupId = entity.conflictGroupId,
        slotId = entity.slot?.id,
    )
}
