package ai.orbis.solver.service

import ai.orbis.solver.domain.Timetable
import ai.orbis.solver.dto.JobStatus
import ai.orbis.solver.dto.SolveJobStatusDto
import ai.orbis.solver.dto.TimetableProblemDto
import ai.orbis.solver.mapper.ProblemMapper
import ai.timefold.solver.core.api.solver.SolverManager
import jakarta.enterprise.context.ApplicationScoped
import jakarta.inject.Inject
import org.jboss.logging.Logger
import java.util.concurrent.ConcurrentHashMap

@ApplicationScoped
class SolveJobManager {

    @Inject
    lateinit var solverManager: SolverManager<Timetable, String>

    private val log = Logger.getLogger(SolveJobManager::class.java)
    private val jobs = ConcurrentHashMap<String, JobState>()

    fun submit(problem: TimetableProblemDto): Boolean {
        val jobId = problem.jobId
        if (jobs.containsKey(jobId)) {
            return false
        }

        val timetable = ProblemMapper.toTimetable(problem)
        jobs[jobId] = JobState(
            jobId = jobId,
            status = JobStatus.running,
            problem = problem,
            bestSolution = ProblemMapper.toSolution(problem, timetable),
        )

        solverManager.solveAndListen(
            jobId,
            { timetable },
            { best -> onBestSolution(jobId, problem, best) },
            { _, _ -> onSolveComplete(jobId) },
        )

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
                state.bestSolution?.score?.let { score ->
                    score.startsWith("0hard")
                } == true -> JobStatus.completed
                else -> JobStatus.completed
            }
            log.infof("Solve job %s finished with status %s score %s",
                jobId, state.status, state.score)
        }
    }

    private class JobState(
        val jobId: String,
        var status: JobStatus,
        val problem: TimetableProblemDto,
        var bestSolution: ai.orbis.solver.dto.TimetableSolutionDto? = null,
        var score: String? = null,
        var error: String? = null,
    )
}
