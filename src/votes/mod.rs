pub mod components;

pub fn vote_red(mut vote: &Vote) {
    vote.vote_red();
}


pub fn vote_blue(mut vote: &Vote) {
    vote.vote_blue();
}