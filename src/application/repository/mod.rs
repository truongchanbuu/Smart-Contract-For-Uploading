#![allow(unused)]

use near_sdk::{env, AccountId, CryptoHash};
use unidecode::unidecode;

use crate::models::{course::CourseId, skill::SkillId, user::UserId};

pub mod internal_certificate;
pub mod internal_course;
pub mod internal_skill;
pub mod internal_user;

/*

Function for course

*/

pub(crate) fn convert_coure_title_to_cousrse_id(title: &str, account_id: String) -> String {
  let unaccented = unidecode(title);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = lowercased + " " + &account_id;
  result.replace(' ', "_")
}

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
  //get the default hash
  let mut hash = CryptoHash::default();
  //we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
  hash
}

//used to make sure the user attached exactly 1 yoctoNEAR
pub(crate) fn assert_one_yocto() {
  assert_eq!(env::attached_deposit(), 1, "Requires attached deposit of exactly 1 yoctoNEAR",)
}

//Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage)
pub(crate) fn assert_at_least_one_yocto() {
  assert!(env::attached_deposit() >= 1, "Requires attached deposit of at least 1 yoctoNEAR",)
}

/*

Function for skill

*/

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_skill_id(skill_id: &SkillId) -> CryptoHash {
  //get the default hash
  let mut hash = CryptoHash::default();
  //we hash the account ID and return it
  hash.copy_from_slice(&env::sha256(skill_id.as_bytes()));
  hash
}

/*

Function for certificate

*/
pub(crate) fn convert_to_certificate_id(course_id: &CourseId, student: &UserId) -> String {
  let cert = "cert ".to_ascii_lowercase();
  let student_convert = student.to_string().to_ascii_lowercase();
  let unaccented = unidecode(course_id);
  let lowercased = unaccented.to_ascii_lowercase();
  let result = cert + &lowercased + " " + &student_convert;
  result.replace(' ', "_")
}
