/// [355] Design Twitter
/// Difficulty: Medium
/// Topics: Hash Table, Linked List, Design, Heap (Priority Queue)
/// Tags: NeetCode150
///
/// Design a simplified version of Twitter where users can post tweets, follow/unfollow another user, and is able to see the 10 most recent tweets in the user's news feed.
/// Implement the Twitter class:
/// - Twitter() Initializes your twitter object.
/// - void postTweet(int userId, int tweetId) Composes a new tweet with ID tweetId by the user userId. Each call to this function will be made with a unique tweetId.
/// - List<Integer> getNewsFeed(int userId) Retrieves the 10 most recent tweet IDs in the user's news feed. Each item in the news feed must be posted by users who the user followed or by the user themself. Tweets must be ordered from most recent to least recent.
/// - void follow(int followerId, int followeeId) The user with ID followerId started following the user with ID followeeId.
/// - void unfollow(int followerId, int followeeId) The user with ID followerId started unfollowing the user with ID followeeId.
///
/// Link: https://leetcode.com/problems/design-twitter/

struct Twitter {
    todo: ()
}

impl Twitter {
    pub fn new() -> Self {
        todo!()
    }
    
    pub fn post_tweet(&self, user_id: i32, tweet_id: i32) {
        todo!()
    }
    
    pub fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        todo!()
    }
    
    pub fn follow(&self, follower_id: i32, followee_id: i32) {
        todo!()
    }
    
    pub fn unfollow(&self, follower_id: i32, followee_id: i32) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // let twitter = Twitter::new();
        // twitter.post_tweet(1, 5);
        // assert_eq!(twitter.get_news_feed(1), vec![5]);
        // twitter.follow(1, 2);
        // twitter.post_tweet(2, 6);
        // assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        // twitter.unfollow(1, 2);
        // assert_eq!(twitter.get_news_feed(1), vec![5]);
        todo!();
    }
}
