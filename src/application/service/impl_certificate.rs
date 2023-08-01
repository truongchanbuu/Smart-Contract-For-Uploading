#![allow(clippy::too_many_arguments)]
use super::super::repository::convert_to_certificate_id;
use crate::models::{
  certificate::{CertificateFeatures, CertificateId, CertificateMetadata},
  contract::{ELearningContract, ELearningContractExt},
  course::CourseId,
  skill::SkillId,
  user::UserId,
};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
/// Implement function for certificate
impl CertificateFeatures for ELearningContract {
  fn mint_certificate(
    &mut self,
    course_id: CourseId,
    student_id: UserId,
    skill_id: SkillId,
    media: String,
    credit: u32,
    description: Option<String>,
  ) {
    // this function only for course owner
    let check_owner = env::signer_account_id();
    let course = self.course_metadata_by_id.get(&course_id).unwrap();
    assert!(check_owner == course.instructor_id, "You are not the course owner");
    assert!(course.students_studying_map.contains_key(&student_id), "This user is not a student in course");
    assert!(course.students_completed.contains_key(&student_id), "Student are not completed the course");

    let certificate_id = convert_to_certificate_id(&course_id, &student_id);
    assert!(
      !self.user_metadata_by_id.get(&student_id).unwrap().certificate.contains(&certificate_id),
      "This certificate already exist"
    );

    // New certificate data
    let certificate_metadata = CertificateMetadata {
      certificate_id: certificate_id.clone(),
      student: student_id.clone(),
      media,
      skill_id,
      credit,
      certificate_used: false,
      description,
    };

    // Storage certificate in system contract
    self.certificate_metadata_by_id.insert(&certificate_id, &certificate_metadata);

    // Storage certificate in student's data
    //self.certificate_per_user.insert(&student, &certificate_id);
    self.internal_add_certificate_to_user(&student_id, &certificate_id);
  }

  /// Get all certicicate by user id
  fn get_all_certificate_by_user_id(
    &self,
    user_id: UserId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<CertificateMetadata> {
    assert!(self.user_metadata_by_id.contains_key(&user_id), "This is not a user");
    // Take user's certificate id
    let certificate_id = self.user_metadata_by_id.get(&user_id).unwrap().certificate;
    certificate_id
      .iter()
      .skip(start.unwrap_or(0) as usize)
      .take(limit.unwrap_or(20) as usize)
      .map(|x| self.certificate_metadata_by_id.get(x).unwrap())
      .collect()
  }

  /// Get certificate metadata by certificate id
  fn get_certificate_metadata_by_certificate_id(&self, certificate_id: CertificateId) -> Option<CertificateMetadata> {
    /* uncomment this code when use event */
    //assert!(self.certificate_metadata_by_id.contains_key(&certificate_id), "Certificate is not exsist");
    if let Some(data) = self.certificate_metadata_by_id.get(&certificate_id) {
      Some(data)
    } else {
      None
    }
  }
}
