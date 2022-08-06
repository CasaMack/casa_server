#[macro_use]
extern crate rocket;
mod run;
use run::get_logger;
use tracing::instrument;

use std::sync::Arc;

use chrono::Timelike;
use influxdb::Client;
use influxdb::ReadQuery;
use rocket::response::status;
use rocket::State;
use serde::Deserialize;

trait QueryType {}

#[derive(Deserialize, Debug)]
struct QueryResults<T>
where
    T: QueryType,
{
    pub results: Vec<Statement<T>>,
}

#[derive(Deserialize, Debug)]
struct Statement<T>
where
    T: QueryType,
{
    pub statement_id: usize,
    pub series: Vec<Serie<T>>,
}

#[derive(Deserialize, Debug)]
struct Serie<T>
where
    T: QueryType,
{
    pub name: String,
    pub columns: Vec<String>,
    pub values: Vec<T>,
}

#[derive(Deserialize, Debug)]
struct Refined {
    time: chrono::DateTime<chrono::Utc>,
    date: String,
    hour: String,
    i8h_low: bool,
    in_0_6_high: bool,
    in_12_18_high: bool,
    in_18_24_high: bool,
    in_6_12_high: bool,
    in_6_l_8: bool,
    pris_forhold_24: f64,
    pris_max: u32,
    pris_min: u32,
    pris_snitt_24: f64,
    pris_time: f64,
    t0_60: bool,
    t115_140: bool,
    t140_999: bool,
    t60_90: bool,
    t90_115: bool,
}

#[derive(Deserialize, Debug)]
struct SingleValue {
    _time: String,
    pub value: f64,
}

impl QueryType for Refined {}
impl QueryType for SingleValue {}

async fn get_refined(client: &Client) -> Result<Refined, ()> {
    let now = chrono::Utc::now();
    let read_query = ReadQuery::new(format!(
        "SELECT * FROM refined WHERE date = '{}' AND hour = '{}'",
        now.date()
            .to_string()
            .split("U")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap(),
        now.hour()
    ));

    let read_result = client.query(read_query).await;
    match read_result {
        Ok(result) => {
            let mut r: QueryResults<Refined> = serde_json::from_str(&result).or(Err(()))?;
            Ok(r.results
                .swap_remove(0)
                .series
                .swap_remove(0)
                .values
                .swap_remove(0))
        }
        Err(e) => {
            eprintln!("{}", e);
            tracing::error!("{}", e);
            Err(())
        }
    }
}

async fn get_single_value(
    series: &str,
    field: &str,
    targ: &str,
    client: &Client,
) -> Result<SingleValue, ()> {
    let read_query = ReadQuery::new(format!(
        "SELECT value FROM {} WHERE {} = '{}' ORDER BY time DESC LIMIT 1",
        series, field, targ
    ));

    let read_result = client.query(read_query).await;
    match read_result {
        Ok(result) => {
            let mut r: QueryResults<SingleValue> = serde_json::from_str(&result).or(Err(()))?;
            Ok(r.results
                .swap_remove(0)
                .series
                .swap_remove(0)
                .values
                .swap_remove(0))
        }
        Err(e) => {
            eprintln!("{}", e);
            tracing::error!("{}", e);
            Err(())
        }
    }
}

struct ClientRef {
    pub client: Arc<Client>,
}

#[instrument(skip(closure, client))]
async fn extract_refined<F>(closure: F, client: &Client) -> status::Accepted<String>
where
    F: Fn(Refined) -> String,
{
    let val = get_refined(client).await.ok().map(closure);
    status::Accepted(val)
}

#[instrument(skip(state))]
async fn extract_single(
    series: &str,
    field: &str,
    targ: &str,
    state: &State<ClientRef>,
) -> status::Accepted<String> {
    let val = get_single_value(series, field, targ, state.client.as_ref())
        .await
        .ok()
        .map(|v| v.value.to_string());
    status::Accepted(val)
}

#[instrument(skip(state))]
#[get("/carChargerUsage")]
async fn car_charger_usage(state: &State<ClientRef>) -> status::Accepted<String> {
    status::Accepted(
        get_single_value("EH230177", "variable", "power", state.client.as_ref())
            .await
            .ok()
            .map(|v| v.value.to_string()),
    )
}

#[instrument(skip(state))]
#[get("/easeeLadeMengde")]
async fn easee_lade_mengde(state: &State<ClientRef>) -> status::Accepted<String> {
    status::Accepted(
        get_single_value("EH230177", "variable", "session", state.client.as_ref())
            .await
            .ok()
            .map(|v| v.value.to_string()),
    )
}

#[instrument(skip(state))]
#[get("/easeeEnergyPerHour")]
async fn easee_energy_per_hour(state: &State<ClientRef>) -> status::Accepted<String> {
    status::Accepted(
        get_single_value(
            "EH230177",
            "variable",
            "energy_per_hour",
            state.client.as_ref(),
        )
        .await
        .ok()
        .map(|v| v.value.to_string()),
    )
}

#[instrument(skip(state))]
#[get("/APIpower")]
async fn api_power(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single("liveMeasurement", "field_name", "power", state).await
}

#[instrument(skip(state))]
#[get("/APIlastMeterConsumption")]
async fn api_last_meter_consumption(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single(
        "liveMeasurement",
        "field_name",
        "lastMeterConsumption",
        state,
    )
    .await
}

#[instrument(skip(state))]
#[get("/APIaccumulatedConsumption")]
async fn api_accumulated_consumption(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single(
        "liveMeasurement",
        "field_name",
        "accumulatedConsumption",
        state,
    )
    .await
}

#[instrument(skip(state))]
#[get("/APIaccumulatedConsumptionLastHour")]
async fn api_accumulated_consumption_last_hour(
    state: &State<ClientRef>,
) -> status::Accepted<String> {
    extract_single(
        "liveMeasurement",
        "field_name",
        "accumulatedConsumptionLastHour",
        state,
    )
    .await
}

#[instrument(skip(state))]
#[get("/APIminPower")]
async fn api_min_power(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single("liveMeasurement", "field_name", "minPower", state).await
}

#[instrument(skip(state))]
#[get("/APIaveragePower")]
async fn api_average_power(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single("liveMeasurement", "field_name", "averagePower", state).await
}

#[instrument(skip(state))]
#[get("/APImaxPower")]
async fn api_max_power(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_single("liveMeasurement", "field_name", "maxPower", state).await
}

#[instrument(skip(state))]
#[get("/PrisSnitt24")]
async fn pris_snitt_24(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.pris_snitt_24.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/in6Low8")]
async fn in_6_l_8(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.in_6_l_8.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/0_6High")]
async fn in_0_6_high(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.in_0_6_high.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/6_12High")]
async fn in_6_12_high(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.in_6_12_high.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/12_18High")]
async fn in_12_18_high(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.in_12_18_high.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/18_24High")]
async fn in_18_24_high(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.in_18_24_high.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/90_115")]
async fn t90_115(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.t90_115.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/60_90")]
async fn t60_90(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.t60_90.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/0_60")]
async fn t0_60(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.t0_60.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/115_140")]
async fn t115_140(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.t115_140.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/140_999")]
async fn t140_999(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.t140_999.to_string(), state.client.as_ref()).await
}

#[instrument(skip(_state))]
#[get("/pricePage")]
async fn price_page(_state: &State<ClientRef>) -> status::Accepted<String> {
    todo!();
}

#[instrument(skip(state))]
#[get("/PrisTime")]
async fn pris_time(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.pris_time.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/PrisForhold24")]
async fn pris_forhold_24(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.pris_forhold_24.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/PrisMax")]
async fn pris_max(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.pris_max.to_string(), state.client.as_ref()).await
}

#[instrument(skip(state))]
#[get("/PrisMin")]
async fn pris_min(state: &State<ClientRef>) -> status::Accepted<String> {
    extract_refined(|r| r.pris_min.to_string(), state.client.as_ref()).await
}

#[launch]
fn rocket() -> _ {
    let (subscriber, _guard) = get_logger();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");
    tracing::info!("Log setup complete");
    let client = Client::new("http://192.168.10.102:8086", "Fibaro");
    rocket::build()
        .manage(ClientRef {
            client: Arc::new(client),
        })
        .mount(
            "/",
            routes![
                car_charger_usage,
                easee_lade_mengde,
                easee_energy_per_hour,
                api_power,
                api_last_meter_consumption,
                api_accumulated_consumption,
                api_accumulated_consumption_last_hour,
                api_min_power,
                api_average_power,
                api_max_power,
                pris_snitt_24,
                in_6_l_8,
                in_0_6_high,
                in_6_12_high,
                in_12_18_high,
                in_18_24_high,
                t90_115,
                t60_90,
                t0_60,
                t115_140,
                t140_999,
                price_page,
                pris_time,
                pris_forhold_24,
                pris_max,
                pris_min,
            ],
        )
}
