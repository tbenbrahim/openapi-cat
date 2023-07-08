pub mod config {
    use regex::Regex;
    use std::fs;

    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Application {
        pub name: Option<String>,
        pub prefix: String,
        pub path: String,
        pub spec: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub applications: Vec<Application>,
    }

    impl Config {
        pub fn read_from(file_path: &str) -> Config {
            let contents =
                fs::read_to_string(file_path).expect("Should have been able to read the file");
            serde_yaml::from_str(contents.as_str())
                .expect(&format!("Could not deserialize file {}", file_path))
        }
        pub fn validate(&self) -> Result<(), String> {
            let mut result = Ok(());
            result = result.and(self.check_unique(
                &self.applications,
                |app| app.prefix.clone(),
                "Prefixes",
            ));
            result =
                result.and(self.check_unique(&self.applications, |app| app.path.clone(), "Paths"));
            result =
                result.and(self.check_unique(&self.applications, |app| app.spec.clone(), "Specs"));
            let name_re = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap();
            let path_re = Regex::new(r"^/[^/]+(/[^/]+)*$").unwrap();

            for app in &self.applications {
                if !name_re.is_match(&app.prefix) {
                    let err = format!("Application prefix {} is not valid", app.prefix);
                    result = result.and(Err(err));
                }

                if !path_re.is_match(&app.path) {
                    let err = format!("Application path {} is not valid", app.path);
                    result = result.and(Err(err));
                }
            }
            result
        }

        fn check_unique(
            &self,
            application: &Vec<Application>,
            extractor: fn(&Application) -> String,
            name: &str,
        ) -> Result<(), String> {
            let mut data: Vec<String> = application.iter().map(|app| extractor(app)).collect();
            let original_len = data.len();
            data.sort();
            data.dedup();
            if data.len() != original_len {
                let err = format!("{} are not unique", name);
                return Err(err);
            }
            Ok(())
        }
    }
}
