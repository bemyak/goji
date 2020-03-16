extern crate env_logger;
extern crate goji;

use futures::executor::block_on;
use goji::{Credentials, Jira, TransitionTriggerOptions};
use std::env;

fn main() {
    drop(env_logger::init());
    if let (Ok(host), Ok(user), Ok(pass), Ok(key)) = (
        env::var("JIRA_HOST"),
        env::var("JIRA_USER"),
        env::var("JIRA_PASS"),
        env::var("JIRA_KEY"),
    ) {
        let jira = Jira::new(host, Credentials::Basic(user, pass)).unwrap();

        println!("{:#?}", block_on(jira.issues().get(key.clone())));
        let transitions = jira.transitions(key);
        for option in block_on(transitions.list()) {
            println!("{:#?}", option);
        }
        if let Ok(transition_id) = env::var("JIRA_TRANSITION_ID") {
            block_on(transitions.trigger(TransitionTriggerOptions::new(transition_id))).unwrap()
        }
    }
}
