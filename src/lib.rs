use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, Promise, PanicOnDefault, AccountId, Timestamp, env};
use near_sdk::serde::{Deserialize, Serialize};

type CID = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner_id: AccountId,
    authors: UnorderedMap<AccountId, Author>,
    works: UnorderedMap<AccountId, Vec<(CID, Work)>>,
    number_of_authors: u128,
    number_of_all_works: u128
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Author {
  id: AccountId,
  name: String,
  age: u32,
  rated_works: Vec<(CID, u32)>
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Work {
    id: CID,
    name: String,
    content: String,
    author: Author,
    fee: u128,
    ratings: Vec<(AccountId, u32)>,
    average_rating: Option<f64>,
    published_date: Timestamp,
    updated_date: Timestamp,
    collaborators: Vec<AccountId>,
    reported_infringements: Option<Vec<Report>>,
    ratios: Option<Vec<(AccountId, u32)>>,
    authorized_users: Vec<AccountId>,
    votes: Option<Vec<Vote>>
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Report {
  reporter: AccountId,
  reason: String,
  timestamp: Timestamp,
}

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Vote {
  voter: AccountId,
  decision: bool,
  timestamp: Timestamp
}

impl Default for Contract {
  fn default() -> Self {
    Self {
      owner_id: env::signer_account_id(),
      authors: UnorderedMap::new(b"authors".try_to_vec().unwrap()),
      works: UnorderedMap::new(b"works".try_to_vec().unwrap()),
      number_of_authors: 0,
      number_of_all_works: 0
    }
  }
}

impl Work {
  pub fn calculate_avg_ratings(&self) -> Option<f64> {
    let total_ratings: u32 = self.ratings.iter().map(|(_, r)| *r).sum();
    let total_count = self.ratings.len() as u32;

    if total_count > 0 {
        let avg_rating = f64::from(total_ratings) / f64::from(total_count);
        Some(avg_rating)
    } else {
        None
    }
  }
}

pub trait Copyright {
  fn create_author(&mut self, name: String, age: u32) -> Author;
  fn create_work(&mut self, 
    name: String, 
    content: String, 
    collaborators: Option<Vec<AccountId>>,
    fee: Option<u128>,
    ratios: Option<Vec<(AccountId, u32)>>
  ) -> Work;
  fn update_author(&mut self, name: Option<String>, age: Option<u32>) -> Author;
  fn update_work(
      &mut self, 
      work_id: CID, 
      name: Option<String>, 
      content: Option<String>, 
      fee: Option<u128>,
      average_rating: Option<f64>,
      ratios: Option<Vec<(AccountId, u32)>>
  ) -> Work;
  fn delete_work(&mut self, work_id: CID) -> bool;
  fn delete_author(&mut self, author_id: AccountId) -> bool;
  fn update_work_list(&mut self, author_id: AccountId, work_obj: &Work);
  fn get_work_by_id(&self, work_id: CID) -> Option<Work>;
  fn get_all_authors(&self) -> Vec<Author>;
  fn get_all_works_of_author(&self, author_id: AccountId) -> Vec<Work>;
  fn get_all_works(&self) -> Vec<Work>;
  fn get_author_by_id(&self, author_id: AccountId) -> Option<Author>;
  fn rate_work(&mut self, work_id: CID, rating: u32) -> bool;
  fn add_collaborator(&mut self, work_id: CID, collaborators: Vec<AccountId>) -> bool;
  fn report_infringement(&mut self, work_id: CID, reason: String) -> bool;
  fn distribute_funds(
    &mut self, 
    total_amount: u128, 
    work_id: CID, 
    ratios: Option<Vec<(AccountId, u32)>>
  );
  fn get_access(&mut self, work_id: CID) -> bool;
  fn vote(&mut self, work_id: CID, decision: bool) -> bool;
}

#[near_bindgen]
impl Copyright for Contract {
  fn create_author(&mut self, name: String, age: u32) -> Author {
      let author = Author {
          id: env::signer_account_id(),
          name, age,
          rated_works: Vec::new(),
      };

      self.authors.insert(&author.id, &author);
      self.number_of_authors += 1;
      author
  }

  fn create_work(
    &mut self, 
    name: String, 
    content: String, 
    collaborators: Option<Vec<AccountId>>, 
    fee: Option<u128>,
    ratios: Option<Vec<(AccountId, u32)>>
  ) -> Work {
      let id = generate_id(name.clone(), env::block_timestamp_ms());
      let author = self.authors.get(&env::signer_account_id()).expect("There is no author");
      
      let author_id = author.id.clone();
      
      let mut collaborators_list = Vec::new();

      if let Some(collaborators) = collaborators {
          for collaborator_id in collaborators {
              collaborators_list.push(collaborator_id);
          }
      }
      
      let fee = fee.unwrap_or(0);

      let work_obj = Work {
          id, name, content, fee,
          author: author.clone(),
          ratings: Vec::new(),
          average_rating: None,
          collaborators: collaborators_list,
          published_date: env::block_timestamp_ms(), 
          updated_date: env::block_timestamp_ms(),
          reported_infringements: None,
          ratios,
          authorized_users: Vec::new(),
          votes: None
      };
      
      self.number_of_all_works += 1;

      self.authors.insert(&author_id, &author);
      
      self.update_work_list(author_id, &work_obj);
      
      work_obj
  }

  fn update_author(&mut self, name: Option<String>, age: Option<u32>) -> Author {
    let mut author = self.authors.get(&env::signer_account_id())
                    .expect("There is no author");
      
    if let Some(name) = name {
      author.name = name;
    }

    if let Some(age) = age {
      author.age = age;
    }

    self.authors.insert(&author.id.clone(), &author);
    
    author
  }

  fn update_work(
    &mut self, 
    work_id: CID, 
    name: Option<String>, 
    content: Option<String>, 
    fee: Option<u128>,
    average_rating: Option<f64>,
    ratios: Option<Vec<(AccountId, u32)>>,
  ) -> Work {
    let mut work = self.get_work_by_id(work_id).expect("There is no work");
    assert_eq!(work.author.id.clone(), env::signer_account_id());

    if let Some(new_name) = name {
      work.name = new_name;
    }

    if let Some(new_content) = content {
      work.content = new_content;
    }

    if let Some(new_avg_rating) = average_rating {
      work.average_rating = Some(new_avg_rating);
    }

    if let Some(new_ratios) = ratios {
      let total_ratio: u32 = new_ratios.iter().map(|(_, ratio)| *ratio).sum();
      assert_eq!(100, total_ratio, "Total ratio must be 100%");

      work.ratios = Some(new_ratios);
    }

    if let Some(fee) = fee {
      work.fee = fee;
    }

    work.updated_date = env::block_timestamp();

    self.update_work_list(work.author.id.clone(), &work.clone());
    work
  }

  fn delete_work(&mut self, work_id: CID) -> bool {
    let author_id = env::signer_account_id();

    if let Some(mut work) = self.get_work_by_id(work_id.clone()) {
        assert_eq!(work.author.id, author_id, "Unauthorized");

        let total_votes = work.votes.as_ref()
                              .expect("There is no vote. Please proceed voting by call `vote` function")
                              .len() as u32;
        let total_people = 1 + work.collaborators.len();
        let caller = work.votes.as_deref().unwrap().iter().find(|v| v.voter == author_id);
        
        if (total_people <= 2 && caller.is_some()) || (total_votes >= ((total_people as f64 - 1.0) * 0.75) as u32 && caller.is_some()) {
          let total_agree = work.votes
                                                .unwrap()
                                                .iter()
                                                .filter(|v| v.decision == true).count() as u32;
          let total_disagree = total_votes - total_agree;

          if total_agree > total_disagree {
            let mut work_vec = self
                                                    .works
                                                    .get(&author_id)
                                                    .expect("Author does not exist");

            if let Some(index) = work_vec.iter().position(|(_, w)| w.id == work_id) {
                work_vec.remove(index);

                work.votes = None;
                self.works.insert(&author_id, &work_vec);
                self.number_of_all_works -= 1;
                
                let author = self.get_author_by_id(author_id.clone()).unwrap();
                self.authors.insert(&author_id, &author);

                return true;
            }
          } else {
              panic!("The number of disagreement is more than");
          }
      } else {
          panic!("Do not get enough conditions");
      }
    }

    false
  }
  
  fn delete_author(&mut self, author_id: AccountId) -> bool {
    if let Some(author) = self.authors.get(&author_id) {
        assert_eq!(author.id, env::signer_account_id(), "Unauthorized");
        self.authors.remove(&author_id);

        return true;
    }

    false
  }

  fn update_work_list(&mut self, author_id: AccountId, work_obj: &Work) {
    if let Some(mut work_list) = self.works.get(&author_id) {
        if let Some(index) = work_list.iter().position(|(id, _)| *id == work_obj.id) {
            work_list[index] = (work_obj.id.clone(), work_obj.clone());
        } else {
            work_list.push((work_obj.id.clone(), work_obj.clone()));
        }
        self.works.insert(&author_id, &work_list);
    } else {
        let work_list = vec![(work_obj.id.clone(), work_obj.clone())];
        self.works.insert(&author_id, &work_list);
    }
  }

  fn get_work_by_id(&self, work_id: CID) -> Option<Work> {
    for work_vec in self.works.values() {
        if let Some((_, work)) = work_vec.iter().find(|(_, w)| *w.id == work_id) {
            if env::signer_account_id() == work.author.id.clone() || work.collaborators.contains(&env::signer_account_id()) {
                return Some(work.clone());
            } else if work.authorized_users.contains(&env::signer_account_id()) {
                let mut work_clone = work.clone();
                work_clone.reported_infringements = None;
                work_clone.ratios = None;
                work_clone.votes = None;
                work_clone.authorized_users = Vec::new();
                return Some(work_clone);
            } else {
                let mut work_clone = work.clone();
                work_clone.content = "Invisible content".to_string();
                work_clone.reported_infringements = None;
                work_clone.ratios = None;
                work_clone.votes = None;
                work_clone.authorized_users = Vec::new();
                return Some(work_clone);
            }
        }
    }
  
    None
  }


  fn get_all_authors(&self) -> Vec<Author> {
    self.authors.values().collect()
  }

  fn get_all_works_of_author(&self, author_id: AccountId) -> Vec<Work> {
    let works = self.works.get(&author_id).unwrap_or_else(Vec::new);

    works
        .iter()
        .map(|(_, work)| {
          if env::signer_account_id() == work.author.id || work.collaborators.contains(&env::signer_account_id()) {
              work.clone()
          } else if work.authorized_users.contains(&env::signer_account_id()) {
              let mut work_clone = work.clone();
              work_clone.reported_infringements = None;
              work_clone.votes = None;
              work_clone
          } else {
              let mut work_clone = work.clone();
              work_clone.content = "Invisible content".to_string();
              work_clone.reported_infringements = None;
              work_clone.authorized_users = Vec::new();
              work_clone.votes = None;
              work_clone
          }
        })
        .collect()
  }


  fn get_all_works(&self) -> Vec<Work> {
    self.works
        .values()
        .flatten()
        .map(|(_, work)| {
          if env::signer_account_id() != work.author.id || !work.collaborators.contains(&env::signer_account_id()) || !work.authorized_users.contains(&env::signer_account_id()) {
              Work {
                reported_infringements: None,
                ratios: None,
                authorized_users: Vec::new(),
                votes: None,
                ..work
              }
          } else {
              work
          }
        })
        .collect()
  }

  fn get_author_by_id(&self, author_id: AccountId) -> Option<Author> {
    self.authors.get(&author_id)
  }

  fn rate_work(&mut self, work_id: CID, rating: u32) -> bool {
    let author_id = env::signer_account_id();

    if let Some(mut work) = self.get_work_by_id(work_id.clone()) {
      if rating <= 5 {
        if let Some(index) = work.ratings.iter().position(|(user, _)| *user == author_id) {
          if rating > 0 {
            work.ratings[index] = (author_id.clone(), rating);
          } else {
            work.ratings.remove(index);
          }
        } else {
          if rating > 0 {
            work.ratings.push((author_id.clone(), rating));
          }
        }

        work.average_rating = work.calculate_avg_ratings();
        let mut author = self.get_author_by_id(author_id.clone()).expect("There's no author");
        author.rated_works.push((work.id.clone(), rating));
        self.authors.insert(&author_id, &author);

        self.update_work_list(work.author.id.clone(), &work.clone());
        return true;
      }
    }

    false
  }

  fn add_collaborator(&mut self, work_id: CID, collaborators: Vec<AccountId>) -> bool {
    if let Some(mut work) = self.get_work_by_id(work_id.clone()) {
        let caller_id = env::signer_account_id();

        if work.author.id == caller_id || work.collaborators.contains(&caller_id) {
            let existing_collaborators = work.collaborators.clone();
            if collaborators.iter().any(|colab| existing_collaborators.contains(colab)) {
                return false;
            }

            let total_votes = work.votes.as_ref().map(|v| v.len()).expect("There is no vote. Please proceed voting by call `vote` function")
            as u32;
            let total_people = 1 + work.collaborators.len();
            let caller = work.votes.as_ref().and_then(|v| v.iter().find(|vote| vote.voter == caller_id));

            if (total_people <= 2 && caller.is_some())
                || (total_votes >= ((total_people as f64 - 1.0) * 0.75) as u32 && caller.is_some())
            {
                let total_agree = work.votes.as_ref()
                    .map(|votes| votes.iter().filter(|v| v.decision).count() as u32)
                    .unwrap_or_default();
                let total_disagree = total_votes - total_agree;

                if total_agree > total_disagree {
                    work.collaborators.extend(collaborators);
                    self.update_work_list(work.author.id.clone(), &work);

                    work.votes = None;
                    return true;
                }
            }
        }
    }
    false
  }

  fn report_infringement(&mut self, work_id: CID, reason: String) -> bool {
    let caller_id = env::signer_account_id();

    if let Some(mut work) = self.get_work_by_id(work_id.clone()) {
        if work.reported_infringements.is_none() {
            work.reported_infringements = Some(Vec::new());
        }

        if let Some(reported_infringements) = work.reported_infringements.as_mut() {
            if let Some(report) = reported_infringements.iter_mut().find(|report| report.reporter == caller_id) {
                report.reason = reason;
                report.timestamp = env::block_timestamp();
            } else {
                let new_report = Report {
                    reporter: caller_id.clone(),
                    reason,
                    timestamp: env::block_timestamp(),
                };
                reported_infringements.push(new_report);
            }
            self.update_work_list(work.author.id.clone(), &work);
            return true;
        }
    }

    false
  }

  #[payable]
  fn distribute_funds(&mut self, total_amount: u128, work_id: CID, ratios: Option<Vec<(AccountId, u32)>>) {
      let ratios = ratios.unwrap_or_else(|| {
        let work = self.get_work_by_id(work_id).expect("There is no work");
        
        let number_of_accounts = work.collaborators.len() + 1;

        let equal_ratio = 100 / number_of_accounts as u32;
        
        let author_id = work.author.id;
        let mut ratios = vec![(author_id, equal_ratio)];
        
        for collaborator in work.collaborators {
          ratios.push((collaborator, equal_ratio));
        }

        ratios
      });

      let total_ratio: u32 = ratios.iter()
                            .map(|(_, ratio)| *ratio).sum();

      assert_eq!(100, total_ratio, "Ratio must be 100%");

      for (account, ratio) in ratios {
          let amount = (total_amount * u128::from(ratio)) / 100;
          Promise::new(account).transfer(amount);
      }
  }

  #[payable]
  fn get_access(&mut self, work_id: CID) -> bool {
    let mut work = self.get_work_by_id(work_id.clone()).expect("There is no work");
    let fee = work.fee;
    let attached_deposit = env::attached_deposit() / 10u128.pow(24);

    if env::signer_account_id() != work.author.id.clone() || !work.collaborators.contains(&env::signer_account_id()) {
      assert_eq!(fee, attached_deposit, "Not correct money amount");
      work.authorized_users.push(env::signer_account_id());

      let work_clone = work.clone();
      self.update_work_list(work.author.id.clone(), &work_clone);
      
      if let Some(ratios) = work.ratios {
          self.distribute_funds(work.fee, work_id.clone(), Some(ratios));
      } else {
          self.distribute_funds(work.fee, work_id.clone(), None);
      }

      return true;
    }

    false
  }

  #[payable]
  fn vote(&mut self, work_id: CID, decision: bool) -> bool {
      let caller_id = env::signer_account_id();
      let mut work = self.get_work_by_id(work_id.clone()).expect("There is no work");

      assert_eq!(caller_id, work.author.id, "Unauthorized");
      
      if let Some(votes) = work.votes.clone() {
        if votes.iter().any(|v| v.voter == caller_id) {
          panic!("Already voted");
        }
      }

      let vote_fee: u128 = 1;
      let attached_deposit = env::attached_deposit() / 10u128.pow(24);

      assert_eq!(vote_fee, attached_deposit, "Not correct money amount");
      
      if work.votes.is_none() {
          work.votes = Some(Vec::new());
      }

      let vote = Vote {
          voter: caller_id.clone(),
          decision,
          timestamp: env::block_timestamp(),
      };

      work.votes.as_mut().unwrap().push(vote);
      self.update_work_list(work.author.id.clone(), &work);

      true
  }
}

fn generate_id(name: String, timestamp: u64) -> CID {
  let words: Vec<&str> = name.split_whitespace().collect();

  let acronym: String = words
      .iter()
      .map(|word| word.chars().next().unwrap().to_ascii_uppercase())
      .collect();

  let id = format!("{}{}", acronym, timestamp);
  id
}