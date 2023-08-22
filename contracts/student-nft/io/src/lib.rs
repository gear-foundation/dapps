#![no_std]

use gmeta::{In, InOut, Metadata};
use gstd::{prelude::*, ActorId};

pub type NftId = u128;
pub type CourseId = u128;
pub type EmoteId = u128;
pub type LessonId = u64;

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<StudentNftInit>;
    type Handle = InOut<StudentNftAction, StudentNftEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = StudentNftState;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StudentNftAction {
    Mint,
    CreateCourse {
        name: String,
        description: String,
    },
    AddCourseHelper {
        course_id: CourseId,
        helper: ActorId,
    },
    RemoveCourseHelper {
        course_id: CourseId,
        helper: ActorId,
    },
    StartCourse {
        course_id: CourseId,
    },
    AddLesson {
        course_id: CourseId,
        lesson: Lesson,
    },
    ApproveHw {
        nft_id: NftId,
        course_id: CourseId,
        lesson_id: LessonId,
        solution_url: String,
        comment: Option<String>,
        rate: u8,
    },
    Emote {
        id: EmoteId,
        action: EmoteAction,
    },
    AddLessonReview {
        course_id: CourseId,
        lesson_id: LessonId,
        review: String,
    },
    FinishCourse {
        course_id: CourseId,
    },
    CompleteCourse {
        course_id: CourseId,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StudentNftEvent {
    Minted {
        user: ActorId,
        id: NftId,
    },
    CourseCreated {
        owner: ActorId,
        id: CourseId,
    },
    CourseHelperAdded {
        id: CourseId,
        helper: ActorId,
    },
    CourseHelperRemoved {
        id: CourseId,
        helper: ActorId,
    },
    CourseStarted {
        user: ActorId,
        id: CourseId,
    },
    LessonAdded {
        course_id: CourseId,
    },
    HwApproved {
        course_id: CourseId,
        nft_id: NftId,
        hw: Hw,
    },
    Emote {
        user: ActorId,
        action: EmoteAction,
    },
    LessonReviewAdded {
        user: ActorId,
        course_id: CourseId,
        lesson_id: LessonId,
        review: String,
    },
    CourseFinished {
        course_id: CourseId,
    },
    CourseCompleted {
        user: ActorId,
        course_id: CourseId,
    },
    Error(String),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StudentNftInit {}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct EmoteState {
    pub upvotes: Vec<ActorId>,
    pub reactions: Vec<(ActorId, String)>,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct StudentNftState {
    pub nfts: Vec<(NftId, Nft)>,
    pub nft_owners: Vec<(ActorId, NftId)>,
    pub courses: Vec<(CourseId, Course)>,
    pub emotes: Vec<(EmoteId, EmoteState)>,
    pub nft_nonce: NftId,
    pub course_nonce: CourseId,
    pub emote_nonce: EmoteId,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Nft {
    pub owner: ActorId,
    pub actual_courses: Vec<ActualCourse>,
}

impl Nft {
    pub fn new(owner: &ActorId) -> Self {
        Nft {
            owner: *owner,
            actual_courses: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct ActualCourse {
    pub id: CourseId,
    pub hws: Vec<Hw>,
    pub is_completed: bool,
}

impl ActualCourse {
    pub fn new(id: CourseId) -> Self {
        ActualCourse {
            id,
            hws: Vec::new(),
            is_completed: false,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Hw {
    pub lesson_id: LessonId,
    pub solution_url: String,
    pub comment: Option<String>,
    pub rate: u8,
    pub check_date: i64,
}

impl Hw {
    pub fn new(
        lesson_id: LessonId,
        solution_url: String,
        comment: Option<String>,
        rate: u8,
        check_date: i64,
    ) -> Self {
        Hw {
            lesson_id,
            solution_url,
            comment,
            rate,
            check_date,
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Course {
    pub owner: ActorId,
    pub owner_helpers: Vec<ActorId>,
    pub name: String,
    pub description: String,
    pub lessons: Vec<Lesson>,
    /// Identifier of associated `Emote` struct.
    pub emote_id: EmoteId,
    pub is_finished: bool,
}

impl Course {
    pub fn new(owner: &ActorId, name: String, description: String, emote_id: EmoteId) -> Self {
        Course {
            owner: *owner,
            owner_helpers: Vec::new(),
            name,
            description,
            lessons: Vec::new(),
            emote_id,
            is_finished: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct Lesson {
    pub name: String,
    pub description: String,
    pub media_url: String,
    pub thumb_url: String,
    pub reviews: Vec<(ActorId, String)>,
    /// Identifier of associated `Emote` struct.
    pub emote_id: EmoteId,
    pub is_provide_hw: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub enum EmoteAction {
    Upvote,
    Reaction { emoji: Option<String> },
}
