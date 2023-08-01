use near_sdk::{borsh::BorshSerialize, collections::UnorderedSet};

use crate::models::{
  contract::{ContractStorageKey, ELearningContract},
  course::CourseId,
  user::UserId,
};

use super::hash_account_id;

impl ELearningContract {
  pub(crate) fn internal_add_course_to_instructor(&mut self, account_id: &UserId, course_id: &CourseId) {
    let mut courses_set = self.courses_per_instructor.get(account_id).unwrap_or_else(|| {
      UnorderedSet::new(
        ContractStorageKey::CoursesPerInstructorInner { instructor_id_hash: hash_account_id(account_id) }
          .try_to_vec()
          .unwrap(),
      )
    });

    //we insert the token ID into the set
    courses_set.insert(course_id);

    self.courses_per_instructor.insert(account_id, &courses_set);
  }
}
