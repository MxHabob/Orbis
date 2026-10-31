package ai.orbis.solver;

import ai.orbis.solver.domain.LessonAssignment;
import ai.orbis.solver.domain.Room;
import ai.orbis.solver.domain.TeacherUnavailability;
import ai.orbis.solver.domain.TimeSlot;
import ai.orbis.solver.domain.Timetable;
import ai.orbis.solver.dto.LessonAssignmentDto;
import ai.orbis.solver.dto.PinnedLessonDto;
import ai.orbis.solver.dto.TimeSlotDto;
import ai.orbis.solver.dto.TimetableProblemDto;
import ai.orbis.solver.mapper.ProblemMapper;
import ai.timefold.solver.core.api.score.buildin.hardsoft.HardSoftScore;
import ai.timefold.solver.core.api.solver.SolverFactory;
import ai.timefold.solver.core.api.solver.SolutionManager;
import com.fasterxml.jackson.databind.json.JsonMapper;
import com.fasterxml.jackson.module.kotlin.KotlinModule;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

class TimetableSolverTest {

    @Test
    void derivesAvailableDaysFromTimeSlotConfiguration() {
        TimetableProblemDto problem = new TimetableProblemDto(
                "job", "org", "period", "incremental",
                List.of(new TimeSlotDto("slot-1", 1, "08:00", "09:00", false, List.of(7))),
                List.of(),
                List.of(new LessonAssignmentDto(
                        "lesson-1", "section-subject-1", "section-1", "subject-1",
                        "teacher-1", 0, null, null, null)),
                List.of(),
                List.of());

        Timetable timetable = ProblemMapper.INSTANCE.toTimetable(problem);

        assertEquals(List.of(7), timetable.getDays());
    }

    @Test
    void acceptsLegacyClassFieldNamesAndSerializesSectionFieldNames() throws Exception {
        var objectMapper = JsonMapper.builder()
                .addModule(new KotlinModule.Builder().build())
                .build();
        String legacyRequest = """
                {
                  "job_id": "job",
                  "org_id": "org",
                  "cycle_id": "period",
                  "mode": "incremental",
                  "time_slots": [{
                    "id": "slot-1",
                    "slot_number": 1,
                    "start_time": "08:00",
                    "end_time": "09:00",
                    "is_break": false,
                    "applicable_days": [7]
                  }],
                  "rooms": [],
                  "assignments": [{
                    "id": "lesson-1",
                    "class_subject_id": "section-subject-1",
                    "class_id": "section-1",
                    "subject_id": "subject-1",
                    "teacher_id": "teacher-1",
                    "hour_index": 0
                  }]
                }
                """;

        TimetableProblemDto problem = objectMapper.readValue(legacyRequest, TimetableProblemDto.class);
        String canonicalJson = objectMapper.writeValueAsString(problem);

        assertEquals("section-1", problem.getAssignments().get(0).getSectionId());
        assertEquals("section-subject-1", problem.getAssignments().get(0).getSectionSubjectId());
        assertTrue(canonicalJson.contains("\"section_id\":\"section-1\""));
        assertTrue(canonicalJson.contains("\"section_subject_id\":\"section-subject-1\""));
        assertFalse(canonicalJson.contains("\"class_id\""));
    }

    @Test
    void rejectsUnavailableDaysAndInvalidPinnedLessons() {
        TimetableProblemDto emptyDays = problem(List.of(
                new TimeSlotDto("slot-1", 1, "08:00", "09:00", false, List.of())));
        assertThrows(IllegalArgumentException.class, () -> ProblemMapper.INSTANCE.validate(emptyDays));

        TimetableProblemDto invalidPinnedDay = new TimetableProblemDto(
                "job", "org", "period", "incremental",
                List.of(new TimeSlotDto("slot-1", 1, "08:00", "09:00", false, List.of(1))),
                List.of(),
                List.of(new LessonAssignmentDto(
                        "lesson-open", "section-subject-1", "section-1", "subject-1",
                        "teacher-1", 0, null, null, null)),
                List.of(new PinnedLessonDto(
                        "lesson-pinned", "section-2", "teacher-2", "subject-2",
                        "section-subject-2", "slot-1", 7, null)),
                List.of());
        assertThrows(IllegalArgumentException.class,
                () -> ProblemMapper.INSTANCE.validate(invalidPinnedDay));
    }

    @Test
    void preservesPinnedLessonsWhileReturningOnlyOpenAssignments() {
        TimetableProblemDto problem = new TimetableProblemDto(
                "job", "org", "period", "incremental",
                List.of(new TimeSlotDto("slot-1", 1, "08:00", "09:00", false, List.of(7))),
                List.of(),
                List.of(new LessonAssignmentDto(
                        "lesson-open", "section-subject-open", "section-open", "subject-open",
                        "teacher-open", 0, null, null, null)),
                List.of(new PinnedLessonDto(
                        "lesson-pinned", "section-pinned", "teacher-pinned", "subject-pinned",
                        "section-subject-pinned", "slot-1", 7, null)),
                List.of());
        Timetable timetable = ProblemMapper.INSTANCE.toTimetable(problem);

        Timetable solution = solverFactory().buildSolver().solve(timetable);
        var result = ProblemMapper.INSTANCE.toSolution(problem, solution);

        assertEquals(HardSoftScore.ZERO, solution.getScore());
        assertEquals(2, solution.getAssignments().size());
        assertEquals(List.of("lesson-open"),
                result.getAssignments().stream().map(LessonAssignmentDto::getId).toList());
        assertEquals("slot-1", solution.getAssignments().stream()
                .filter(LessonAssignment::getPinned)
                .findFirst()
                .orElseThrow()
                .getTimeSlot()
                .getId());
    }

    @Test
    void preventsTeacherConflictsAcrossOverlappingTimeSlots() {
        TimeSlot first = slot("slot-1", 8 * 60, 9 * 60);
        TimeSlot overlapping = slot("slot-2", 8 * 60 + 30, 9 * 60 + 30);
        TimeSlot later = slot("slot-3", 10 * 60, 11 * 60);
        Timetable problem = new Timetable();
        problem.setTimeSlots(List.of(first, overlapping, later));
        problem.setDays(List.of(1));
        LessonAssignment overlappingLesson1 = assignment("lesson-1", "section-1");
        overlappingLesson1.setTimeSlot(first);
        overlappingLesson1.setDayOfWeek(1);
        LessonAssignment overlappingLesson2 = assignment("lesson-2", "section-2");
        overlappingLesson2.setTimeSlot(overlapping);
        overlappingLesson2.setDayOfWeek(1);
        problem.setAssignments(List.of(overlappingLesson1, overlappingLesson2));

        SolverFactory<Timetable> solverFactory =
                SolverFactory.createFromXmlResource("TimetableSolverConfig.xml");
        assertEquals(HardSoftScore.ofHard(-1), score(problem, solverFactory));
        Timetable solution = solverFactory.buildSolver().solve(problem);

        LessonAssignment lesson1 = solution.getAssignments().get(0);
        LessonAssignment lesson2 = solution.getAssignments().get(1);
        assertEquals(HardSoftScore.ZERO, solution.getScore());
        assertNotNull(lesson1.getDayOfWeek());
        assertNotNull(lesson2.getDayOfWeek());
        assertEquals("teacher-1", lesson1.getTeacherId());
        assertEquals("teacher-1", lesson2.getTeacherId());
        assertFalse(overlaps(lesson1.getTimeSlot(), lesson2.getTimeSlot()),
                () -> "Solver left overlapping assignments at "
                        + lesson1.getTimeSlot().getId() + " and "
                        + lesson2.getTimeSlot().getId());
    }

    @Test
    void preventsRoomConflictsWhenLessonsOverlap() {
        TimeSlot first = slot("slot-1", 8 * 60, 9 * 60);
        TimeSlot overlapping = slot("slot-2", 8 * 60 + 30, 9 * 60 + 30);
        Room room = new Room();
        room.setId("room-1");

        LessonAssignment lesson1 = assignment("lesson-1", "section-1");
        lesson1.setTeacherId("teacher-1");
        lesson1.setTimeSlot(first);
        lesson1.setDayOfWeek(1);
        lesson1.setRoom(room);
        LessonAssignment lesson2 = assignment("lesson-2", "section-2");
        lesson2.setTeacherId("teacher-2");
        lesson2.setTimeSlot(overlapping);
        lesson2.setDayOfWeek(1);
        lesson2.setRoom(room);

        Timetable problem = timetable(List.of(first, overlapping), List.of(lesson1, lesson2));
        problem.setRooms(List.of(room));

        assertEquals(HardSoftScore.ofHard(-1), score(problem, solverFactory()));
    }

    @Test
    void preventsSectionConflictsWhenLessonsOverlap() {
        TimeSlot first = slot("slot-1", 8 * 60, 9 * 60);
        TimeSlot overlapping = slot("slot-2", 8 * 60 + 30, 9 * 60 + 30);
        LessonAssignment lesson1 = assignment("lesson-1", "section-1");
        lesson1.setTimeSlot(first);
        lesson1.setDayOfWeek(1);
        LessonAssignment lesson2 = assignment("lesson-2", "section-1");
        lesson2.setTeacherId("teacher-2");
        lesson2.setTimeSlot(overlapping);
        lesson2.setDayOfWeek(1);

        assertEquals(HardSoftScore.ofHard(-1),
                score(timetable(List.of(first, overlapping), List.of(lesson1, lesson2)), solverFactory()));
    }

    @Test
    void rejectsDaysNotConfiguredForTheSelectedTimeSlot() {
        TimeSlot slot = slot("slot-1", 8 * 60, 9 * 60);
        LessonAssignment lesson = assignment("lesson-1", "section-1");
        lesson.setTimeSlot(slot);
        lesson.setDayOfWeek(2);

        assertEquals(HardSoftScore.ofHard(-1),
                score(timetable(List.of(slot), List.of(lesson)), solverFactory()));
    }

    @Test
    void preventsLessonsDuringOverlappingTeacherUnavailability() {
        TimeSlot lessonSlot = slot("slot-1", 8 * 60, 9 * 60);
        TimeSlot unavailableSlot = slot("slot-2", 8 * 60 + 30, 9 * 60 + 30);
        LessonAssignment lesson = assignment("lesson-1", "section-1");
        lesson.setTimeSlot(lessonSlot);
        lesson.setDayOfWeek(1);

        TeacherUnavailability unavailable = new TeacherUnavailability();
        unavailable.setId("unavailable-1");
        unavailable.setTeacherId("teacher-1");
        unavailable.setDayOfWeek(1);
        unavailable.setTimeSlot(unavailableSlot);

        Timetable problem = timetable(List.of(lessonSlot, unavailableSlot), List.of(lesson));
        problem.setUnavailableSlots(List.of(unavailable));

        assertEquals(HardSoftScore.ofHard(-1), score(problem, solverFactory()));
    }

    private static Timetable timetable(List<TimeSlot> slots, List<LessonAssignment> assignments) {
        Timetable timetable = new Timetable();
        timetable.setTimeSlots(slots);
        timetable.setDays(List.of(1));
        timetable.setAssignments(assignments);
        return timetable;
    }

    private static TimetableProblemDto problem(List<TimeSlotDto> slots) {
        return new TimetableProblemDto(
                "job", "org", "period", "full", slots, List.of(),
                List.of(new LessonAssignmentDto(
                        "lesson-1", "section-subject-1", "section-1", "subject-1",
                        "teacher-1", 0, null, null, null)),
                List.of(), List.of());
    }

    private static SolverFactory<Timetable> solverFactory() {
        return SolverFactory.createFromXmlResource("TimetableSolverConfig.xml");
    }

    private static HardSoftScore score(Timetable timetable, SolverFactory<Timetable> solverFactory) {
        return SolutionManager.<Timetable, HardSoftScore>create(solverFactory).update(timetable);
    }

    private static TimeSlot slot(String id, int start, int end) {
        TimeSlot slot = new TimeSlot();
        slot.setId(id);
        slot.setSlotNumber(1);
        slot.setStartMinuteOfDay(start);
        slot.setEndMinuteOfDay(end);
        slot.setApplicableDays(java.util.Set.of(1));
        return slot;
    }

    private static LessonAssignment assignment(String id, String sectionId) {
        LessonAssignment assignment = new LessonAssignment();
        assignment.setId(id);
        assignment.setSectionId(sectionId);
        assignment.setSectionSubjectId(sectionId + "-subject");
        assignment.setSubjectId("subject-1");
        assignment.setTeacherId("teacher-1");
        return assignment;
    }

    private static boolean overlaps(TimeSlot first, TimeSlot second) {
        return first.getStartMinuteOfDay() < second.getEndMinuteOfDay()
                && second.getStartMinuteOfDay() < first.getEndMinuteOfDay();
    }
}
