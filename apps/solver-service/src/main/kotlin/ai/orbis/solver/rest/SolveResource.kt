package ai.orbis.solver.rest

import ai.orbis.solver.dto.SubmitResponseDto
import ai.orbis.solver.dto.TimetableProblemDto
import ai.orbis.solver.service.SolveJobManager
import jakarta.inject.Inject
import jakarta.ws.rs.*
import jakarta.ws.rs.core.MediaType
import jakarta.ws.rs.core.Response

@Path("/")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
class SolveResource {

    @Inject
    lateinit var jobManager: SolveJobManager

    @POST
    @Path("/solve")
    fun submit(problem: TimetableProblemDto): Response {
        val accepted = jobManager.submit(problem)
        return if (accepted) {
            Response.accepted(SubmitResponseDto(problem.jobId, true)).build()
        } else {
            Response.status(Response.Status.CONFLICT)
                .entity(SubmitResponseDto(problem.jobId, false))
                .build()
        }
    }

    @GET
    @Path("/solve/{jobId}")
    fun status(@PathParam("jobId") jobId: String): Response {
        val status = jobManager.status(jobId)
            ?: return Response.status(Response.Status.NOT_FOUND).build()
        return Response.ok(status).build()
    }

    @DELETE
    @Path("/solve/{jobId}")
    fun cancel(@PathParam("jobId") jobId: String): Response {
        jobManager.cancel(jobId)
        return Response.noContent().build()
    }

    @GET
    @Path("/health")
    fun health(): Response = Response.ok(mapOf("status" to "healthy", "service" to "timetable-solver")).build()
}
