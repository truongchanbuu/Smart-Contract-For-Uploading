use std::collections::HashMap;

use crate::models::contract::{ELearningContract, ELearningContractExt};
use crate::models::user::{ImplUser, JsonUser, Roles, UserId, UserMetadata};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
/// Implement function for user
impl ImplUser for ELearningContract {
  /// Create a user
  fn create_user(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
  ) {
    // Check User has exist
    let user_id = env::signer_account_id();
    assert!(!self.user_metadata_by_id.contains_key(&user_id), "User has already exists");
    let new_nickname = if let Some(value) = nickname { value } else { user_id.to_string() };

    // Create the basic information of user
    let user_metadata = UserMetadata {
      user_id: user_id.clone(),
      role: Roles::Subscriber,
      nickname: new_nickname,
      total_credit: 0,
      first_name,
      last_name,
      bio,
      avatar: None,
      resume: None,
      created_at: env::block_timestamp_ms(),
      updated_at: env::block_timestamp_ms(),
      courses_owned: 0,
      students: 0,
    };

    // Create a Json of user
    let json_user = JsonUser {
      user_id: user_id.clone(),
      metadata: user_metadata,
      skill: HashMap::new(),
      certificate: Vec::new(),
      courses: Vec::new(),
    };

    // Storage json user in system contract
    self.user_metadata_by_id.insert(&user_id, &json_user);

    // Storage user_id in system contract
    self.subscriber_users.insert(&user_id);
  }

  /// Update the role
  fn update_role(&mut self) -> JsonUser {
    // Only Owned has access
    let user_id = env::signer_account_id();
    assert!(self.user_metadata_by_id.contains_key(&user_id), "You don't have access!");

    // Check user had the resume
    let mut user = self.user_metadata_by_id.get(&user_id).unwrap();
    assert!(user.metadata.role == Roles::Subscriber, "You already is Instructor");
    assert!(user.metadata.resume.is_some(), "You must upload your resume!");

    // Change user's role then storage
    user.metadata.role = Roles::Instructor;
    self.user_metadata_by_id.insert(&user_id, &user);
    self.intructor_users.insert(&user_id);

    // return
    user
  }

  /// Update user information
  fn update_user_information(
    &mut self,
    nickname: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    bio: Option<String>,
    avatar: Option<String>,
    resume: Option<String>,
  ) -> JsonUser {
    // Check access
    assert!(self.user_metadata_by_id.contains_key(&env::signer_account_id()), "You don't have access");

    let mut user = self.user_metadata_by_id.get(&env::signer_account_id()).unwrap();

    // Check attribute. If it have some -> update
    if let Some(n) = nickname {
      user.metadata.nickname = n
    };
    if let Some(f) = first_name {
      user.metadata.first_name = Some(f)
    }

    if let Some(l) = last_name {
      user.metadata.last_name = Some(l)
    }

    if let Some(b) = bio {
      user.metadata.bio = Some(b)
    }

    if let Some(a) = avatar {
      user.metadata.avatar = Some(a)
    }

    if let Some(r) = resume {
      user.metadata.resume = Some(r)
    }

    // Storage time information when user update
    user.metadata.updated_at = env::block_timestamp_ms();

    // Storage the change
    self.user_metadata_by_id.insert(&env::signer_account_id(), &user);

    // Return
    user
  }

  /// Get user information. From 'index' to 'index + limit'
  fn get_all_user_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser> {
    let mut all_user = Vec::new();
    for user_id in
      self.subscriber_users.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize)
    {
      all_user.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }
    all_user
  }

  /// Get Instructor information. From 'index' to 'index + limit'
  fn get_all_instructor_metadata(&self, from_index: Option<u32>, limit: Option<u32>) -> Vec<JsonUser> {
    let mut all_instructor = Vec::new();
    for user_id in self.intructor_users.iter().skip(from_index.unwrap_or(0) as usize).take(limit.unwrap_or(20) as usize)
    {
      all_instructor.push(self.user_metadata_by_id.get(&user_id).unwrap());
    }
    all_instructor
  }

  /// Check user role
  fn check_user_role(&self, user_id: UserId) -> Roles {
    // Chek user exist or not
    assert!(self.user_metadata_by_id.contains_key(&user_id), "User does not exist");
    // Return
    self.user_metadata_by_id.get(&user_id).unwrap().metadata.role
  }

  /// Get information of user
  fn get_user_metadata_by_user_id(&self, user_id: &UserId) -> Option<JsonUser> {
    if let Some(metadata) = self.user_metadata_by_id.get(user_id) {
      Some(metadata)
    } else {
      None
    }
  }

  fn create_pool() {}
}
