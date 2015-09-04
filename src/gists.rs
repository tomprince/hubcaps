
use self::super::Github;
use rustc_serialize::json;
use std::io::Result;
use rep::{Gist, GistFork, GistReq};

/// reference to gists associated with a github user
pub struct UserGists<'a> {
  github: &'a Github<'a>,
  owner: &'static str,
}

impl<'a> UserGists<'a> {

  pub fn new(github: &'a Github<'a>, owner: &'static str) -> UserGists<'a> {
    UserGists { github: github, owner: owner }
  }

  pub fn list(&self) -> Result<Vec<Gist>> {
    let body = try!(
      self.github.get(
        &format!("/users/{}/gists", self.owner)
      )
    );
    Ok(json::decode::<Vec<Gist>>(&body).unwrap())
  }
}

pub struct Gists<'a> {
  github: &'a Github<'a>
}

impl<'a> Gists<'a> {
  pub fn new(github: &'a Github<'a>) -> Gists<'a> {
    Gists { github: github }
  }

  fn path(&self, more: &str) -> String {
    format!("/gists{}", more)
  }

  pub fn star(&self, id: &'static str) -> Result<()> {
    self.github.put(
      &self.path(
        &format!("/{}/star", id)
      ),
      &[]
    ).map(|_| ())
  }

  pub fn unstar(&self, id: &'static str) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/{}/star", id)
      )
    ).map(|_| ())
  }

  pub fn fork(&self, id: &'static str) -> Result<Gist> {
    let body = try!(
      self.github.post(
        &self.path(
          &format!("/{}/forks", id)
        ),
        &[]
      )
    );
    Ok(json::decode::<Gist>(&body).unwrap())
  }

  pub fn forks(&self, id: &'static str) -> Result<Vec<GistFork>> {
    let body = try!(
      self.github.get(
        &self.path(
          &format!("/{}/forks", id)
        )
      )
    );
    Ok(json::decode::<Vec<GistFork>>(&body).unwrap())
  }

  pub fn delete(&self, id: &'static str) -> Result<()> {
    self.github.delete(
      &self.path(
        &format!("/{}", id)
      )
    ).map(|_| ())
  }

  pub fn get(&self, id: &'static str) -> Result<Gist> {
    let body = try!(
      self.github.get(
        &self.path(
          &format!("/{}", id)
        )
      )
    );
    Ok(json::decode::<Gist>(&body).unwrap())
  }

  pub fn getrev(&self, id: &'static str, sha: &'static str) -> Result<Gist> {
    let body = try!(
      self.github.get(
        &self.path(
          &format!("/{}/{}", id, sha)
        )
      )
    );
    Ok(json::decode::<Gist>(&body).unwrap())
  }

  pub fn list(&self) -> Result<Vec<Gist>> {
    let body = try!(
      self.github.get(
        &self.path("")
      )
    );
    Ok(json::decode::<Vec<Gist>>(&body).unwrap())
  }

  pub fn public(&self) -> Result<Vec<Gist>> {
    let body = try!(
      self.github.get(
        &self.path("/public")
      )
    );
    Ok(json::decode::<Vec<Gist>>(&body).unwrap())
    }

  pub fn starred(&self) -> Result<Vec<Gist>> {
    let body = try!(
      self.github.get(
        &self.path("/starred")
      )
    );
    Ok(json::decode::<Vec<Gist>>(&body).unwrap())
  }

  pub fn create(&self, gist: &GistReq) -> Result<Gist> {
    let data = json::encode(&gist).unwrap();
    let body = try!(
      self.github.post(
        &self.path(""),
        data.as_bytes()
      )
    );
    Ok(json::decode::<Gist>(&body).unwrap())
  }
}