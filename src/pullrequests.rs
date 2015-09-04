use self::super::{Github, SortDirection, State};
use rep::{Pull, PullEdit, PullReq};
use rustc_serialize::json;
use std::default::Default;
use std::fmt;
use std::io::Result;

pub enum Sort {
  Created,
  Updated,
  Popularity,
  LongRunning
}

impl fmt::Display for Sort {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      Sort::Created     => "created",
      Sort::Updated     => "updated",
      Sort::Popularity  => "popularity",
      Sort::LongRunning => "long-running"
    })
  }
}

impl Default for Sort {
  fn default() -> Sort {
    Sort::Created
  }
}



pub struct PullRequest<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str,
  number: i64
}

impl<'a> PullRequest<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str, number: i64) -> PullRequest<'a> {
    PullRequest { github: github, owner: owner, repo: repo, number: number }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/pulls/{}{}", self.owner, self.repo, self.number, more)
  }

  pub fn get(&self) -> Result<Pull> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }

  /// short hand for editing state = open
  pub fn open(&self) -> Result<Pull> {
    self.edit(&PullEdit::new(None, None, Some("open")))
  }

  /// shorthand for editing state = closed
  pub fn close(&self) -> Result<Pull> {
    self.edit(&PullEdit::new(None, None, Some("closed")))
  }

  pub fn edit(&self, pr: &PullEdit) -> Result<Pull> {
    let data = json::encode(&pr).unwrap();
    let body = try!(
      self.github.patch(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }
}

pub struct PullRequests<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
  repo: &'static str
}

pub struct ListBuilder<'a> {
  pulls: &'a PullRequests<'a>,
  state: State,
  sort: Sort,
  direction: SortDirection
}

impl<'a> ListBuilder<'a> {
  pub fn new(pulls: &'a PullRequests<'a>) -> ListBuilder<'a> {
    ListBuilder {
      pulls: pulls,
      state: Default::default(),
      sort: Default::default(),
      direction: Default::default()
    }
  }

  pub fn state(&mut self, state: State) -> &mut ListBuilder<'a> {
    self.state = state;
    self
  }

  pub fn sort(&mut self, sort: Sort) -> &mut ListBuilder<'a> {
    self.sort = sort;
    self
  }

  pub fn direction(&mut self, direction: SortDirection) -> &mut ListBuilder<'a> {
    self.direction = direction;
    self
  }

  pub fn get(&self) -> Result<Vec<Pull>> {
    let body = try!(
      self.pulls.github.get(
        &self.pulls.path(
          &format!(
            "?state={}&sort={}&direction={}", self.state, self.sort, self.direction
          )[..]
        )
      )
    );
    Ok(json::decode::<Vec<Pull>>(&body).unwrap())
  }
}


impl<'a> PullRequests<'a> {
  pub fn new(github: &'a Github<'a>, owner: &'static str, repo: &'static str) -> PullRequests<'a> {
    PullRequests { github: github, owner: owner, repo: repo }
  }

  fn path(&self, more: &str) -> String {
    format!("/repos/{}/{}/pulls{}", self.owner, self.repo, more)
  }

  pub fn get(&self, number: i64) -> PullRequest {
    PullRequest::new(self.github, self.owner, self.repo, number)
  }

  pub fn create(&self, pr: &PullReq) -> Result<Pull> {
    let data = json::encode(&pr).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Pull>(&body).unwrap())
  }

  pub fn list(&self) -> ListBuilder {
    ListBuilder::new(self)
  }
}
