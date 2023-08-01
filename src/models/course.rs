use std::collections::HashMap;

use near_sdk::{
  borsh::{self, BorshDeserialize, BorshSerialize},
  serde::{Deserialize, Serialize},
  AccountId, Balance,
};

use super::user::UserId;

/// `CourseId` is a type alias for `String`, typically representing a unique identifier for a course in the system.
pub type CourseId = String;

/// The `CourseMetadata` struct represents metadata for a course in the system.
#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CourseMetadata {
  /// Unique identifier for the course, of type `CourseId`.
  pub course_id: CourseId,

  /// Name of the course.
  pub title: String,

  /// Detailed description of the course.
  pub description: Option<String>,

  /// Thumbnail of the course
  pub media: Option<String>,

  /// Instructor's account ID.
  pub instructor_id: UserId,

  /// Date when the course was created, represented as a timestamp.
  pub created_at: u64,

  /// Price of this course, of type `U128`.
  pub price: Balance,

  /// Number of students currently studying this course.
  pub students_studying_map: HashMap<AccountId, u64>,

  /// Number of students who have completed this course. And time stamp
  pub students_completed: HashMap<AccountId, u64>,

  /// Average of all the ratings this course has received.
  pub rating: u8,

  /// Number of ratings this course has received.
  pub rating_count: u32,

  /// The Content of this course
  pub content: String,
}

pub trait CourseFeatures {
  fn create_course(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: Balance,
  ) -> CourseMetadata;
  fn payment_course(&mut self, course_id: CourseId);
  fn get_course_metadata_by_course_id(&self, course_id: CourseId) -> Option<CourseMetadata>;
  fn get_all_courses_per_instructor(
    &self,
    instructor_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata>;

  /// Get all the course per user have. Current and complete course
  fn get_all_courses_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata>;

  /// Make user completed the course
  fn make_user_finish_course(&mut self, course_id: CourseId, user_id: UserId);

  /// Check user completed course or not
  fn check_course_completed(&self, course_id: CourseId, user_id: UserId) -> bool;
}
