use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build test app");
        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_drop)]
        let _ = tokio::spawn(app.run());
        let http_client = reqwest::Client::new();
        Self { address, http_client }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn post_signup(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .send()
            .await
            .expect("Failed to execute signup")
    }

    pub async fn get_login(&self) -> reqwest::Response {
        self.http_client
        .get(format!("{}/login", &self.address))
        .send()
        .await
        .expect("Failed to execute login")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
        .post(format!("{}/logout", &self.address))
        .send()
        .await
        .expect("Failed to execute logout")
    }

    pub async fn get_verify_token(&self) -> reqwest::Response {
        self.http_client
        .get(format!("{}/verify-token", &self.address))
        .send()
        .await
        .expect("Failed to execute verify-token")
    }

    pub async fn get_verify_f2a(&self) -> reqwest::Response {
        self.http_client
        .get(format!("{}/verify-f2a", &self.address))
        .send()
        .await
        .expect("Failed to execute verify-f2a")
    }
}