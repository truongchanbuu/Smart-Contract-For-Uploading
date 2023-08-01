use crate::models::{
  certificate::CertificateId,
  contract::{ELearningContract, ELearningContractExt},
  skill::{SkillFeatures, SkillId, SkillMetadata},
  user::UserId,
};

use near_sdk::{env, near_bindgen};
use std::collections::HashMap;

#[near_bindgen]
impl SkillFeatures for ELearningContract {
  /// Add new new skill by user
  fn add_skill(&mut self, student: UserId, skill_id: SkillId, credit: u32) {
    let mut user = self.user_metadata_by_id.get(&student).unwrap();
    user.skill.entry(skill_id).and_modify(|x| *x += credit).or_insert(credit);
    self.user_metadata_by_id.insert(&student, &user);
  }

  /// Mint a skill from certificate credit
  fn mint_skill_by_certificate(&mut self, certificate_id: CertificateId) {
    // Check certificate
    assert!(self.certificate_metadata_by_id.contains_key(&certificate_id), "This Certificate is not exist");

    // Check certificatw owner
    let mut certificate = self.certificate_metadata_by_id.get(&certificate_id).unwrap();
    assert!(certificate.student == env::signer_account_id(), "This certificate is not belong to you");
    assert!(!certificate.certificate_used, "This Certificate has been used");

    // Create new skillmetadata by skill id in system contract
    let skill_to_add = SkillMetadata {
      skill_id: certificate.skill_id.clone(),
      credit: certificate.credit,
      credit_from: certificate_id.clone(),
      use_skill: true,
      description: certificate.description.clone(),
    };
    //internal skill to add new skill metadata to skill id
    self.internal_add_skill_metadata_to_skill_id(&certificate.skill_id, &skill_to_add);

    // Storage new certificate data
    certificate.certificate_used = true;
    self.certificate_metadata_by_id.insert(&certificate_id, &certificate);
    // Add new skill credit for user
    self.add_skill(certificate.student, certificate.skill_id, certificate.credit);
  }

  /// Get all skills per user
  fn get_all_skills_per_user(&self, user_id: UserId) -> HashMap<SkillId, u32> {
    assert!(self.user_metadata_by_id.contains_key(&user_id), "User is not exists");
    self.user_metadata_by_id.get(&user_id).unwrap().skill
  }

  /// Get skill metadata by skill id
  fn get_all_skill_metadata_by_skill_id(
    &self,
    skill_id: SkillId,
    start: Option<u32>,
    limit: Option<u32>,
  ) -> Vec<SkillMetadata> {
    let skill_per_skill_id = self.skill_metadata_by_skill_id.get(&skill_id);
    let data = if let Some(skill_id_set) = skill_per_skill_id { skill_id_set } else { return vec![] };
    data.iter().skip(start.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize).collect()
  }
}
