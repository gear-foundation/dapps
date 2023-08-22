use super::ADMIN;
use gstd::prelude::*;
use gtest::{Program, System};
use student_nft_io::{
    CourseId, EmoteAction, EmoteId, Lesson, LessonId, NftId, StudentNftAction, StudentNftEvent,
    StudentNftInit, StudentNftState,
};

pub trait StudentNft {
    fn student_nft(system: &System) -> Program;
    fn mint(&self, from: u64, error: bool);
    fn create_course(&self, from: u64, name: &str, description: &str, error: bool);
    fn start_course(&self, from: u64, course_id: CourseId, error: bool);
    fn add_course_helper(&self, from: u64, course_id: CourseId, helper: u64, error: bool);
    fn remove_course_helper(&self, from: u64, course_id: CourseId, helper: u64, error: bool);
    fn add_lesson(&self, from: u64, course_id: CourseId, lesson: &Lesson, error: bool);
    #[allow(clippy::too_many_arguments)]
    fn approve_hw(
        &self,
        from: u64,
        nft_id: NftId,
        course_id: CourseId,
        lesson_id: LessonId,
        solution_url: &str,
        comment: Option<String>,
        rate: u8,
        error: bool,
    );
    fn emote(&self, from: u64, id: EmoteId, action: EmoteAction, error: bool);
    fn add_lesson_review(
        &self,
        from: u64,
        course_id: CourseId,
        lesson_id: LessonId,
        review: &str,
        error: bool,
    );
    fn finish_course(&self, from: u64, course_id: CourseId, error: bool);
    fn complete_course(&self, from: u64, course_id: CourseId, error: bool);
    fn send_tx(&self, from: u64, action: StudentNftAction, error: bool);
    fn get_state(&self) -> StudentNftState;
}

impl StudentNft for Program<'_> {
    fn student_nft(system: &System) -> Program {
        let student_nft = Program::current(system);
        assert!(!student_nft.send(ADMIN, StudentNftInit {}).main_failed());

        student_nft
    }

    fn mint(&self, from: u64, error: bool) {
        self.send_tx(from, StudentNftAction::Mint, error);
    }

    fn create_course(&self, from: u64, name: &str, description: &str, error: bool) {
        self.send_tx(
            from,
            StudentNftAction::CreateCourse {
                name: name.to_owned(),
                description: description.to_owned(),
            },
            error,
        );
    }

    fn start_course(&self, from: u64, course_id: CourseId, error: bool) {
        self.send_tx(from, StudentNftAction::StartCourse { course_id }, error);
    }

    fn add_course_helper(&self, from: u64, course_id: CourseId, helper: u64, error: bool) {
        self.send_tx(
            from,
            StudentNftAction::AddCourseHelper {
                course_id,
                helper: helper.into(),
            },
            error,
        );
    }

    fn remove_course_helper(&self, from: u64, course_id: CourseId, helper: u64, error: bool) {
        self.send_tx(
            from,
            StudentNftAction::RemoveCourseHelper {
                course_id,
                helper: helper.into(),
            },
            error,
        );
    }

    fn add_lesson(&self, from: u64, course_id: CourseId, lesson: &Lesson, error: bool) {
        self.send_tx(
            from,
            StudentNftAction::AddLesson {
                course_id,
                lesson: lesson.clone(),
            },
            error,
        );
    }

    fn approve_hw(
        &self,
        from: u64,
        nft_id: NftId,
        course_id: CourseId,
        lesson_id: LessonId,
        solution_url: &str,
        comment: Option<String>,
        rate: u8,
        error: bool,
    ) {
        self.send_tx(
            from,
            StudentNftAction::ApproveHw {
                nft_id,
                course_id,
                lesson_id,
                solution_url: solution_url.to_owned(),
                comment,
                rate,
            },
            error,
        )
    }

    fn add_lesson_review(
        &self,
        from: u64,
        course_id: CourseId,
        lesson_id: LessonId,
        review: &str,
        error: bool,
    ) {
        self.send_tx(
            from,
            StudentNftAction::AddLessonReview {
                course_id,
                lesson_id,
                review: review.to_owned(),
            },
            error,
        )
    }

    fn emote(&self, from: u64, id: EmoteId, action: EmoteAction, error: bool) {
        self.send_tx(from, StudentNftAction::Emote { id, action }, error);
    }

    fn complete_course(&self, from: u64, course_id: CourseId, error: bool) {
        self.send_tx(from, StudentNftAction::CompleteCourse { course_id }, error);
    }

    fn finish_course(&self, from: u64, course_id: CourseId, error: bool) {
        self.send_tx(from, StudentNftAction::FinishCourse { course_id }, error);
    }

    fn send_tx(&self, from: u64, action: StudentNftAction, error: bool) {
        let result = self.send(from, action);
        assert!(!result.main_failed());

        let maybe_error = result.log().iter().find_map(|log| {
            let mut payload = log.payload();
            if let Ok(StudentNftEvent::Error(error)) = StudentNftEvent::decode(&mut payload) {
                Some(error)
            } else {
                None
            }
        });

        assert_eq!(maybe_error.is_some(), error);
    }

    fn get_state(&self) -> StudentNftState {
        self.read_state().expect("Unexpected invalid state.")
    }
}
