use super::super::repository::convert_to_certificate_id;
use crate::{
  application::repository::convert_coure_title_to_cousrse_id,
  models::{
    contract::{ELearningContract, ELearningContractExt},
    course::{CourseFeatures, CourseId, CourseMetadata},
    user::{ImplUser, Roles, UserId},
  },
};
use near_sdk::{env, near_bindgen, Balance};
use std::collections::HashMap;

#[near_bindgen]
impl CourseFeatures for ELearningContract {
  fn create_course(
    &mut self,
    title: String,
    description: Option<String>,
    media: Option<String>,
    price: Balance,
  ) -> CourseMetadata {
    let instructor_id = env::signer_account_id();
    let course_id = convert_coure_title_to_cousrse_id(&title, instructor_id.to_string());
    assert!(
      self.user_metadata_by_id.get(&instructor_id).unwrap().metadata.role == Roles::Instructor,
      "You aren't an instructor, You need register & upload your resume to become a instructor!"
    );
    assert!(
      !self.course_metadata_by_id.contains_key(&course_id),
      "Please! Change your title course, it already exists"
    );

    let course_metadata = CourseMetadata {
      course_id: course_id.clone(),
      title,
      price,
      media,
      description,
      instructor_id: instructor_id.clone(),
      created_at: env::block_timestamp_ms(),
      students_completed: HashMap::new(),
      students_studying_map: HashMap::new(),
      rating: 0,
      rating_count: 0,
      content: "".to_string(),
    };
    self.course_metadata_by_id.insert(&course_id, &course_metadata);
    let mut user = self.user_metadata_by_id.get(&instructor_id).unwrap();
    user.metadata.courses_owned += 1;
    self.user_metadata_by_id.insert(&instructor_id, &user);
    self.internal_add_course_to_instructor(&instructor_id, &course_id);
    course_metadata
  }

  fn get_all_courses_per_instructor(
    &self,
    instructor_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata> {
    assert!(
      self.user_metadata_by_id.get(&instructor_id).unwrap().metadata.role == Roles::Instructor,
      "This user are not Instructor. Please change the Instructor id"
    );
    let courses_per_owner_set = self.courses_per_instructor.get(&instructor_id);
    let courses = if let Some(courses_per_owner) = courses_per_owner_set {
      courses_per_owner
    } else {
      return vec![];
    };

    courses
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(10) as usize)
      .map(|value| self.get_course_metadata_by_course_id(value).unwrap())
      .collect()
  }

  #[payable]
  fn payment_course(&mut self, course_id: CourseId) {
    // Check course has exists
    let mut course = self.course_metadata_by_id.get(&course_id);
    let user_id = env::signer_account_id();
    assert!(course.is_some(), "The course doesn't exists");
    assert!(self.user_metadata_by_id.contains_key(&user_id), "You need registration to use platform!");
    assert!(user_id != course.clone().unwrap().instructor_id, "You own the course");
    assert!(!course.clone().unwrap().students_studying_map.contains_key(&user_id), "You already have this course!");

    // Plus 1 student to course owner
    let mut coure_owner = self.user_metadata_by_id.get(&course.clone().unwrap().instructor_id).unwrap();
    coure_owner.metadata.students += 1;
    self.user_metadata_by_id.insert(&coure_owner.user_id, &coure_owner);

    // Storage new course data
    course.as_mut().unwrap().students_studying_map.insert(user_id.clone(), env::block_timestamp_ms());
    let mut user = self.get_user_metadata_by_user_id(&user_id).unwrap();
    user.courses.push(course_id.clone());
    self.user_metadata_by_id.insert(&user_id, &user);
    self.course_metadata_by_id.insert(&course_id, &course.unwrap());
  }

  /// Get all the course per user have. Current and complete course
  fn get_all_courses_per_user_own(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CourseMetadata> {
    // Check the user is exists or not
    assert!(self.user_metadata_by_id.contains_key(&user_id), "User does not exist. Please change the user id");
    let mut all_courses = Vec::new();

    // Get course id per user
    let user_course = self.user_metadata_by_id.get(&user_id).unwrap().courses;

    // Get course metadata
    for course in user_course.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize) {
      all_courses.push(self.get_course_metadata_by_course_id(course.clone()).unwrap())
    }

    // return
    all_courses
  }

  fn check_course_completed(&self, course_id: CourseId, user_id: UserId) -> bool {
    // Check course exist or not. User is student or not
    assert!(self.course_metadata_by_id.contains_key(&course_id), "This course is not exist");
    let course_set = self.course_metadata_by_id.get(&course_id).unwrap();
    assert!(course_set.students_studying_map.contains_key(&user_id), "This user is not a student of this course");
    // Return
    course_set.students_completed.contains_key(&user_id)
  }

  // TODO: More Requirement to check
  fn make_user_finish_course(&mut self, course_id: CourseId, user_id: UserId) {
    let check_user = env::signer_account_id();
    let mut course = self.course_metadata_by_id.get(&course_id).unwrap();

    // Check the courser owner
    assert!(check_user == course.instructor_id, "You are not the course owner");
    // Check user are student in this course or not
    assert!(course.students_studying_map.contains_key(&user_id), "This user is not a student in this course");
    // Check: has student complete the course yet
    let certificate_id = convert_to_certificate_id(&course_id, &user_id);
    assert!(
      !self.user_metadata_by_id.get(&user_id).unwrap().certificate.contains(&certificate_id),
      "This student already completed the course"
    );
    // Update new data
    course.students_completed.insert(user_id, env::block_timestamp_ms());
    self.course_metadata_by_id.insert(&course_id, &course);
  }

  /// Get all the course per user have. Current and complete course
  fn get_course_metadata_by_course_id(&self, course_id: CourseId) -> Option<CourseMetadata> {
    /* uncomment this code when use event*/
    //assert!(self.course_metadata_by_id.contains_key(&course_id), "This course is not exist");
    if let Some(course) = self.course_metadata_by_id.get(&course_id) {
      Some(course)
    } else {
      None
    }
  }
}
