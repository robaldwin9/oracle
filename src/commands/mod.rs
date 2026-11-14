use crate::Error;
use rand::seq::IndexedRandom;
mod decide;
mod compliment;
mod insult;
mod consult;
mod roll;

pub fn all() -> Vec<poise::Command<crate::Data, crate::Error>> {
    vec![decide::decide(), compliment::compliment(), insult::insult(), consult::consult(), roll::roll()]
}


pub fn pick_random_response(entries: &[String]) -> Result<String, Error> {
     entries
        .choose(&mut rand::rng())
        .cloned()
        .ok_or_else(|| "Response list is empty".into())
}
