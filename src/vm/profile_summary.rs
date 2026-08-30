pub mod slot_view_model {
    use crate::save::common::user_data_10::ProfileSummary;


    #[derive(Clone)]
    pub struct ProfileSummaryViewModel {
        pub active: bool,
        pub character_name: String,
        pub seconds_played: u32,
    }

    impl Default for ProfileSummaryViewModel {
        fn default() -> Self {
            Self {
                active: Default::default(),
                character_name: Default::default(),
                seconds_played: Default::default(),
            }
        }
    }

    impl ProfileSummaryViewModel {
        pub fn from_save(profile_summary: &ProfileSummary) -> Self {
            let active = true;

            // Character Name
            let character_name = profile_summary.character_name;
            let character_name = String::from_utf16(&character_name).expect("");

            // Seconds played
            let seconds_played = profile_summary.seconds_played;

            Self {
                active,
                character_name,
                seconds_played,
            }
        }

        /// Format seconds played as "XXh XXm XXs"
        pub fn play_time_formatted(&self) -> String {
            let hours = self.seconds_played / 3600;
            let minutes = (self.seconds_played % 3600) / 60;
            let seconds = self.seconds_played % 60;
            format!("{}h {}m {}s", hours, minutes, seconds)
        }
    }
}