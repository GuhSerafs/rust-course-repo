#[cfg(test)]
mod tests {

    mod state_pattern_tests {
        use obj_oriented_rust::state_pattern::Post;

        #[test]
        fn check_insert_empty_content_in_post() {
            let post = Post::new();
            assert_eq!("", post.get_content());
        }

        #[test]
        fn check_insert_with_content_in_post() {
            let post = Post::from("Hello people!");
            assert_eq!("Hello people!", post.get_content());
        }

        #[test]
        fn check_recently_created_post_state() {
            let mut post = Post::new();
            post.add_text("New informations arrived!");
            assert_eq!("New informations arrived!", post.get_content());
            assert_eq!("", post.get_published_content());
        }

        #[test]
        fn check_request_review_message() {
            let mut post = Post::new();
            
            post.add_text("Breaking News!");
            assert_eq!("", post.get_published_content());
            assert_eq!("Breaking News!", post.get_content());
            
            post.request_review();
            assert_eq!("Breaking News!", post.get_content());
            assert_eq!("[Waiting for Review]", post.get_published_content());
        }

        #[test]
        fn check_approved_post() {
            let mut post = Post::new();
            post.add_text("A great day to be alive!");
            assert_eq!("A great day to be alive!", post.get_content());
            assert_eq!("", post.get_published_content());

            post.request_review();
            assert_eq!("A great day to be alive!", post.get_content());
            assert_eq!("[Waiting for Review]", post.get_published_content());

            post.approve();
            assert_eq!("A great day to be alive!", post.get_content());
            assert_eq!("A great day to be alive!", post.get_published_content());
        }
    }
    
    mod typed_state_pattern_tests {
        use obj_oriented_rust::typed_state_pattern::*;

        #[test]
        fn check_typed_change_of_states() {
            let mut post = Post::new();
            post.add_text("What a nice implementation, doesn't it?");
            assert_eq!("", post.published_content());

            let post = post.request_review();
            assert_eq!("[Waiting Review]", post.published_content());

            let post = post.approve();
            assert_eq!("What a nice implementation, doesn't it?", post.content());
        }
    }
}
