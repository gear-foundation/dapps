use gstd::{errors::Result as GstdResult, exec, msg, prelude::*, ActorId, MessageId};
use hashbrown::{HashMap, HashSet};
use student_nft_io::{
    ActualCourse, Course, CourseId, EmoteAction, EmoteId, EmoteState, Hw, Lesson, LessonId, Nft,
    NftId, StudentNftAction, StudentNftEvent, StudentNftInit, StudentNftState,
};

#[derive(Debug, Default)]
pub struct Emote {
    pub upvotes: HashSet<ActorId>,
    pub reactions: HashMap<ActorId, String>,
}

impl From<&Emote> for EmoteState {
    fn from(value: &Emote) -> Self {
        EmoteState {
            upvotes: value.upvotes.iter().copied().collect(),
            reactions: value
                .reactions
                .iter()
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
        }
    }
}

#[derive(Debug, Default)]
struct StudentNFT {
    nfts: HashMap<NftId, Nft>,
    nft_owners: HashMap<ActorId, NftId>,
    courses: HashMap<CourseId, Course>,
    emotes: HashMap<EmoteId, Emote>,

    nft_nonce: NftId,
    course_nonce: CourseId,
    emote_nonce: EmoteId,
}

impl From<&StudentNFT> for StudentNftState {
    fn from(value: &StudentNFT) -> Self {
        StudentNftState {
            nfts: value.nfts.iter().map(|(k, v)| (*k, v.clone())).collect(),
            nft_owners: value.nft_owners.iter().map(|(k, v)| (*k, *v)).collect(),
            courses: value.courses.iter().map(|(k, v)| (*k, v.clone())).collect(),
            emotes: value.emotes.iter().map(|(k, v)| (*k, v.into())).collect(),
            nft_nonce: value.nft_nonce,
            course_nonce: value.course_nonce,
            emote_nonce: value.emote_nonce,
        }
    }
}

static mut STUDENT_NFT: Option<StudentNFT> = None;

fn process_mint(student_nft: &mut StudentNFT, user: &ActorId) -> StudentNftEvent {
    let Some(next_nft_nonce) = student_nft.nft_nonce.checked_add(1) else {
        return StudentNftEvent::Error("Math overflow.".to_owned());
    };

    if student_nft.nft_owners.contains_key(user) {
        return StudentNftEvent::Error("User already has student nft.".to_owned());
    }

    student_nft.nft_owners.insert(*user, next_nft_nonce);
    student_nft.nft_nonce = next_nft_nonce;
    student_nft.nfts.insert(next_nft_nonce, Nft::new(user));

    StudentNftEvent::Minted {
        user: *user,
        id: next_nft_nonce,
    }
}

fn process_create_course(
    student_nft: &mut StudentNFT,
    owner: &ActorId,
    name: String,
    description: String,
) -> StudentNftEvent {
    let Some(next_course_nonce) = student_nft.course_nonce.checked_add(1) else {
        return StudentNftEvent::Error("Math overflow.".to_owned());
    };

    let Some(next_emote_nonce) = student_nft.emote_nonce.checked_add(1) else {
        return StudentNftEvent::Error("Math overflow.".to_owned());
    };

    student_nft.course_nonce = next_course_nonce;
    student_nft.emote_nonce = next_emote_nonce;
    student_nft.courses.insert(
        next_course_nonce,
        Course::new(owner, name, description, next_emote_nonce),
    );
    student_nft
        .emotes
        .insert(next_emote_nonce, Emote::default());

    StudentNftEvent::CourseCreated {
        owner: *owner,
        id: next_course_nonce,
    }
}

fn process_add_course_helper(
    student_nft: &mut StudentNFT,
    owner: &ActorId,
    id: CourseId,
    helper: ActorId,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&id) {
        if owner != &course.owner {
            return StudentNftEvent::Error("Only owner can add more helpers.".to_owned());
        }

        if course.owner_helpers.contains(&helper) {
            return StudentNftEvent::Error("Helper already added.".to_owned());
        }

        course.owner_helpers.push(helper);
        StudentNftEvent::CourseHelperAdded { id, helper }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_remove_course_helper(
    student_nft: &mut StudentNFT,
    owner: &ActorId,
    id: CourseId,
    helper: ActorId,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&id) {
        if owner != &course.owner {
            return StudentNftEvent::Error("Only owner can remove helpers.".to_owned());
        }

        if !course.owner_helpers.contains(&helper) {
            return StudentNftEvent::Error("Helper not found.".to_owned());
        }

        let helpers: Vec<ActorId> = course
            .owner_helpers
            .iter()
            .filter(|&h| h != &helper)
            .copied()
            .collect();

        course.owner_helpers = helpers;
        StudentNftEvent::CourseHelperRemoved { id, helper }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_add_lesson(
    student_nft: &mut StudentNFT,
    source: &ActorId,
    id: CourseId,
    lesson: Lesson,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&id) {
        if source != &course.owner && !course.owner_helpers.contains(source) {
            return StudentNftEvent::Error(
                "Only owner or helpers can add more lessons.".to_owned(),
            );
        }

        if course.is_finished {
            return StudentNftEvent::Error("Provided course is finished.".to_owned());
        }

        let Some(next_emote_nonce) = student_nft.emote_nonce.checked_add(1) else {
            return StudentNftEvent::Error("Math overflow.".to_owned());
        };

        student_nft.emote_nonce = next_emote_nonce;
        student_nft
            .emotes
            .insert(next_emote_nonce, Emote::default());

        course.lessons.push(Lesson {
            emote_id: next_emote_nonce,
            ..lesson
        });
        StudentNftEvent::LessonAdded { course_id: id }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_start_course(
    student_nft: &mut StudentNFT,
    user: &ActorId,
    id: CourseId,
) -> StudentNftEvent {
    if let Some(nft_id) = student_nft.nft_owners.get(user) {
        if student_nft.courses.get(&id).is_none() {
            return StudentNftEvent::Error("Provided course does not exist.".to_owned());
        };

        let Some(nft) = student_nft.nfts.get_mut(nft_id) else {
            return StudentNftEvent::Error("Invalid nft id.".to_owned());
        };

        if nft.actual_courses.iter().any(|ac| ac.id == id) {
            return StudentNftEvent::Error("Course already started.".to_owned());
        }

        nft.actual_courses.push(ActualCourse::new(id));
        StudentNftEvent::CourseStarted { user: *user, id }
    } else {
        StudentNftEvent::Error("User don't have student nft.".to_owned())
    }
}

#[allow(clippy::too_many_arguments)]
fn process_approve_hw(
    student_nft: &mut StudentNFT,
    source: &ActorId,
    nft_id: NftId,
    course_id: CourseId,
    lesson_id: LessonId,
    solution_url: String,
    comment: Option<String>,
    rate: u8,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&course_id) {
        if source != &course.owner && !course.owner_helpers.contains(source) {
            return StudentNftEvent::Error("Only owner or helpers can approve hw.".to_owned());
        }

        if let Some(lesson) = course.lessons.get(lesson_id as usize) {
            if !lesson.is_provide_hw {
                return StudentNftEvent::Error("Lesson does not provide hw.".to_owned());
            }
        } else {
            return StudentNftEvent::Error("Invalid lesson id.".to_owned());
        }

        let Some(nft) = student_nft.nfts.get_mut(&nft_id) else {
            return StudentNftEvent::Error("Invalid nft id.".to_owned());
        };

        let Some(actual_course) = nft.actual_courses.iter_mut().find(|ac| ac.id == course_id) else {
            return StudentNftEvent::Error("Course is not started by nft owner.".to_owned());
        };

        if actual_course.is_completed {
            return StudentNftEvent::Error("Course is already completed by nft owner.".to_owned());
        }

        if actual_course.hws.iter().any(|hw| hw.lesson_id == lesson_id) {
            return StudentNftEvent::Error("Hw already approved for provided lesson.".to_owned());
        }

        let hw = Hw::new(
            lesson_id,
            solution_url,
            comment,
            rate,
            exec::block_timestamp() as i64,
        );

        actual_course.hws.push(hw.clone());

        StudentNftEvent::HwApproved {
            course_id,
            nft_id,
            hw,
        }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_finish_course(
    student_nft: &mut StudentNFT,
    source: &ActorId,
    course_id: CourseId,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&course_id) {
        if source != &course.owner && !course.owner_helpers.contains(source) {
            return StudentNftEvent::Error("Only owner or helpers can finish course.".to_owned());
        }

        if course.is_finished {
            return StudentNftEvent::Error("Course is already finished.".to_owned());
        }

        course.is_finished = true;
        StudentNftEvent::CourseFinished { course_id }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_complete_course(
    student_nft: &mut StudentNFT,
    user: &ActorId,
    course_id: CourseId,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&course_id) {
        if !course.is_finished {
            return StudentNftEvent::Error("Course is not finished by owner.".to_owned());
        }

        let Some(nft_id) = student_nft.nft_owners.get(user) else {
            return StudentNftEvent::Error("User does not have nft.".to_owned());
        };

        let Some(nft) = student_nft.nfts.get_mut(nft_id) else {
            return StudentNftEvent::Error("Invalid nft id.".to_owned());
        };

        let Some(actual_course) = nft.actual_courses.iter_mut().find(|ac| ac.id == course_id) else {
            return StudentNftEvent::Error("Course is not started by nft owner.".to_owned());
        };

        if actual_course.is_completed {
            return StudentNftEvent::Error("Course is already completed.".to_owned());
        }

        // List with id's of all lessons, which require hw
        let mut required_lessons_id: Vec<u64> = course
            .lessons
            .iter()
            .enumerate()
            .filter_map(|(id, lesson)| {
                if lesson.is_provide_hw {
                    Some(id as u64)
                } else {
                    None
                }
            })
            .collect();
        required_lessons_id.sort();

        // List with submited id's of lessons(which require hw)
        let mut actual_lessons_id: Vec<u64> =
            actual_course.hws.iter().map(|h| h.lesson_id).collect();
        actual_lessons_id.sort();

        if required_lessons_id == actual_lessons_id {
            actual_course.is_completed = true;
            StudentNftEvent::CourseCompleted {
                user: *user,
                course_id,
            }
        } else {
            StudentNftEvent::Error("Required hw's are not completed.".to_owned())
        }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

fn process_emote(
    student_nft: &mut StudentNFT,
    user: &ActorId,
    id: EmoteId,
    action: EmoteAction,
) -> StudentNftEvent {
    if let Some(emote) = student_nft.emotes.get_mut(&id) {
        match &action {
            EmoteAction::Upvote => {
                if emote.upvotes.contains(user) {
                    emote.upvotes.remove(user);
                } else {
                    emote.upvotes.insert(*user);
                }
            }
            EmoteAction::Reaction { emoji } => {
                if let Some(emoji) = emoji {
                    emote.reactions.insert(*user, emoji.clone());
                } else {
                    emote.reactions.remove(user);
                }
            }
        }

        StudentNftEvent::Emote {
            user: *user,
            action,
        }
    } else {
        StudentNftEvent::Error("Invalid emote id.".to_owned())
    }
}

fn process_add_lesson_review(
    student_nft: &mut StudentNFT,
    user: &ActorId,
    course_id: CourseId,
    lesson_id: LessonId,
    review: String,
) -> StudentNftEvent {
    if let Some(course) = student_nft.courses.get_mut(&course_id) {
        if review.is_empty() {
            return StudentNftEvent::Error("Review is empty.".to_owned());
        }

        let Some(lesson) = course.lessons.get_mut(lesson_id as usize) else {
            return StudentNftEvent::Error("Invalid lesson id.".to_owned());
        };

        lesson.reviews.push((*user, review.clone()));

        StudentNftEvent::LessonReviewAdded {
            user: *user,
            course_id,
            lesson_id,
            review,
        }
    } else {
        StudentNftEvent::Error("Provided course does not exist.".to_owned())
    }
}

#[no_mangle]
extern "C" fn init() {
    let _init: StudentNftInit = msg::load().expect("Unable to decode `StudentNftInit`.");

    unsafe { STUDENT_NFT = Some(StudentNFT::default()) };
}

#[no_mangle]
extern "C" fn handle() {
    let action: StudentNftAction = msg::load().expect("Could not load `StudentNftAction`.");
    let student_nft: &mut StudentNFT = unsafe { STUDENT_NFT.get_or_insert(StudentNFT::default()) };

    let user = msg::source();

    let result = match action {
        StudentNftAction::Mint => process_mint(student_nft, &user),
        StudentNftAction::CreateCourse { name, description } => {
            process_create_course(student_nft, &user, name, description)
        }
        StudentNftAction::AddCourseHelper { course_id, helper } => {
            process_add_course_helper(student_nft, &user, course_id, helper)
        }
        StudentNftAction::RemoveCourseHelper { course_id, helper } => {
            process_remove_course_helper(student_nft, &user, course_id, helper)
        }
        StudentNftAction::StartCourse { course_id } => {
            process_start_course(student_nft, &user, course_id)
        }
        StudentNftAction::AddLesson { course_id, lesson } => {
            process_add_lesson(student_nft, &user, course_id, lesson)
        }
        StudentNftAction::ApproveHw {
            nft_id,
            course_id,
            lesson_id,
            solution_url,
            comment,
            rate,
        } => process_approve_hw(
            student_nft,
            &user,
            nft_id,
            course_id,
            lesson_id,
            solution_url,
            comment,
            rate,
        ),
        StudentNftAction::Emote { id, action } => process_emote(student_nft, &user, id, action),
        StudentNftAction::AddLessonReview {
            course_id,
            lesson_id,
            review,
        } => process_add_lesson_review(student_nft, &user, course_id, lesson_id, review),
        StudentNftAction::FinishCourse { course_id } => {
            process_finish_course(student_nft, &user, course_id)
        }
        StudentNftAction::CompleteCourse { course_id } => {
            process_complete_course(student_nft, &user, course_id)
        }
    };

    reply(result).expect("Failed to encode or reply with `StudentNftEvent`.");
}

#[no_mangle]
extern "C" fn state() {
    reply(unsafe {
        let student_nft = STUDENT_NFT
            .as_ref()
            .expect("Uninitialized `StudentNFT` state.");
        let student_nft_state: StudentNftState = student_nft.into();
        student_nft_state
    })
    .expect("Failed to share state.");
}

fn reply(payload: impl Encode) -> GstdResult<MessageId> {
    msg::reply(payload, 0)
}
