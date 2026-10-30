package ai.orbis.solver.service

import ai.orbis.solver.domain.Timetable
import ai.orbis.solver.domain.ExamSchedule
import ai.orbis.solver.dto.JobStatus
import ai.orbis.solver.dto.SolveJobStatusDto
import ai.orbis.solver.dto.TimetableProblemDto
import ai.orbis.solver.dto.ExamProblemDto
import ai.orbis.solver.dto.ExamSolveJobStatusDto
import ai.orbis.solver.mapper.ProblemMapper
import ai.orbis.solver.mapper.ExamProblemMapper
import ai.timefold.solver.core.api.solver.SolverManager
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import jakarta.inject.Named
import org.jboss.logging.Logger
import java.util.concurrent.ConcurrentHashMap

@ApplicationScoped
class SolveJobManager {

    @Inject
    @Named("timetableSolver")
    lateinit var solverManager: SolverManager<Timetable, String>

    @Inject
    @Named("examSolver")
    lateinit var examSolverManager: SolverManager<ExamSchedule, String>

    private val log = Logger.getLogger(SolveJobManager::class.java)
    private val jobs = ConcurrentHashMap<String, JobState>()
    private val examJobs = ConcurrentHashMap<String, ExamJobState>()

    fun submit(problem: TimetableProblemDto): Boolean {
        val jobId = problem.jobId
        val timetable = ProblemMapper.toTimetable(problem)
        val state = JobState(
            jobId = jobId,
            status = JobStatus.running,
            problem = problem,
            bestSolution = ProblemMapper.toSolution(problem, timetable),
        )
        if (jobs.putIfAbsent(jobId, state) != null) return false

        try {
            solverManager.solveBuilder()
                .withProblemId(jobId)
                .withProblemFinder { timetable }
                .withBestSolutionConsumer { best -> onBestSolution(jobId, problem, best) }
                .withFinalBestSolutionConsumer { _ -> onSolveComplete(jobId) }
                .withExceptionHandler { _, throwable ->
                    jobs[jobId]?.let { state ->
                        state.status = JobStatus.failed
                        state.error = throwable.message ?: "Solver failed"
                    }
                    log.error("Solving failed for job $jobId", throwable)
                }
                .run()
        } catch (error: RuntimeException) {
            jobs.remove(jobId, state)
            log.error("Failed to start solve job $jobId", error)
            throw IllegalStateException("Failed to start solve job $jobId", error)
        }

        log.infof("Started solve job %s with %d assignments", jobId, problem.assignments.size)
        return true
    }

    fun status(jobId: String): SolveJobStatusDto? {
        val state = jobs[jobId] ?: return null
        return SolveJobStatusDto(
            jobId = jobId,
            status = state.status.name,
            score = state.bestSolution?.score,
            bestSolution = state.bestSolution,
            error = state.error,
        )
    }

    fun cancel(jobId: String): Boolean {
        val state = jobs[jobId] ?: return false
        if (state.status == JobStatus.completed || state.status == JobStatus.failed) {
            return true
        }
        solverManager.terminateEarly(jobId)
        state.status = JobStatus.cancelled
        return true
    }

    private fun onBestSolution(jobId: String, problem: TimetableProblemDto, best: Timetable) {
        jobs[jobId]?.let { state ->
            state.bestSolution = ProblemMapper.toSolution(problem, best)
            state.score = best.score?.toString()
        }
    }

    private fun onSolveComplete(jobId: String) {
        jobs[jobId]?.let { state ->
            state.status = when {
                state.status == JobStatus.cancelled -> JobStatus.cancelled
                state.status == JobStatus.failed -> JobStatus.failed
                state.bestSolution?.score?.let(::hasZeroHardScore) == true -> JobStatus.completed
                else -> {
                    state.error = "The solver could not find a conflict-free timetable"
                    JobStatus.failed
                }
            }
            log.infof("Solve job %s finished with status %s score %s",
                jobId, state.status, state.score)
        }
    }

    private fun hasZeroHardScore(score: String): Boolean =
        score.substringBefore("hard/").toIntOrNull() == 0

    // --- Exam Solver Management ---

    fun submitExam(problem: ExamProblemDto): Boolean {
        val jobId = problem.jobId
        if (examJobs.containsKey(jobId)) {
            return false
        }

        val schedule = ExamProblemMapper.toSchedule(problem)
        examJobs[jobId] = ExamJobState(
            jobId = jobId,
            status = JobStatus.running,
            problem = problem,
            bestSolution = ExamProblemMapper.toSolution(problem, schedule),
        )

        examSolverManager.solveBuilder()
            .withProblemId(jobId)
            .withProblemFinder { schedule }
            .withBestSolutionConsumer { best -> onBestExamSolution(jobId, problem, best) }
            .withFinalBestSolutionConsumer { _ -> onExamSolveComplete(jobId) }
            .withExceptionHandler { _, throwable ->
                examJobs[jobId]?.let { state ->
                    state.status = JobStatus.failed
                    state.error = throwable.message ?: "Exam solver failed"
                }
                log.error("Exam solving failed for job $jobId", throwable)
            }
            .run()

        log.infof("Started exam solve job %s with %d assignments", jobId, problem.assignments.size)
        return true
    }

    fun statusExam(jobId: String): ExamSolveJobStatusDto? {
        val state = examJobs[jobId] ?: return null
        return ExamSolveJobStatusDto(
            jobId = jobId,
            status = state.status.name,
            score = state.bestSolution?.score,
            bestSolution = state.bestSolution,
            error = state.error,
        )
    }

    fun cancelExam(jobId: String): Boolean {
        val state = examJobs[jobId] ?: return false
        if (state.status == JobStatus.completed || state.status == JobStatus.failed) {
            return true
        }
        examSolverManager.terminateEarly(jobId)
        state.status = JobStatus.cancelled
        return true
    }

    private fun onBestExamSolution(jobId: String, problem: ExamProblemDto, best: ExamSchedule) {
        examJobs[jobId]?.let { state ->
            state.bestSolution = ExamProblemMapper.toSolution(problem, best)
            state.score = best.score?.toString()
        }
    }

    private fun onExamSolveComplete(jobId: String) {
        examJobs[jobId]?.let { state ->
            state.status = when {
                state.status == JobStatus.cancelled -> JobStatus.cancelled
                state.bestSolution?.score?.let { score ->
                    score.startsWith("0hard")
                } == true -> JobStatus.completed
                else -> JobStatus.completed
            }
            log.infof("Exam solve job %s finished with status %s score %s",
                jobId, state.status, state.score)
        }
    }

    private class JobState(
        val jobId: String,
        @Volatile var status: JobStatus,
        val problem: TimetableProblemDto,
        @Volatile var bestSolution: ai.orbis.solver.dto.TimetableSolutionDto? = null,
        @Volatile var score: String? = null,
        @Volatile var error: String? = null,
    )

    private class ExamJobState(
        val jobId: String,
        var status: JobStatus,
        val problem: ExamProblemDto,
        var bestSolution: ai.orbis.solver.dto.ExamSolutionDto? = null,
        var score: String? = null,
        var error: String? = null,
    )
}
