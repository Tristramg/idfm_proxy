/*

struct AppState {
    uri: String,
    apikey: String,
    siri: Option<siri_lite::siri::Siri>,
}

impl AppState {
    async fn update(&mut self) -> Result<(DateTime<Utc>, chrono::Duration)> {
        let start = Utc::now();
        let client = reqwest::Client::new();
        let resp = client
            .get(&self.uri)
            .header("apikey", &self.apikey)
            .query(&[("LineRef", "ALL")])
            .send()
            .await?
            .text()
            .await?;
        self.siri = Some(serde_json::from_str(&resp)?);

        let end = Utc::now();
        Ok((end, end - start))
    }
}

let state = Arc::new(Mutex::new(AppState {
    uri: "https://prim.iledefrance-mobilites.fr/marketplace/estimated-timetable".to_string(),
    apikey: "".to_string(),
    siri: None,
}));

let _state2 = state.clone();
  tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(60));
    loop {
        interval.tick().await;

        println!("Lancement mise-à-jour {}", Utc::now());
        let mut data = state2.lock().await;
        match data.update().await {
            Ok((end, duration)) => println!("...Mise-à-jour finie à {:?} ({}s) — nombre deliveries {}", end, duration.num_seconds(), data.siri.as_ref().unwrap().service_delivery.as_ref().unwrap().estimated_timetable_delivery.len()),
            err => println!("Erreur… {:?}", err)
        }
    }
});*/
