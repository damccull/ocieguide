use bigdecimal::{BigDecimal, FromPrimitive};
use ocieguide::persistence::{
    model::{LineItemNumber, NationalStockNumber, OcieItem},
    repository::PostgresOcieItemRepository,
};
use uuid::Uuid;

use crate::helpers::TestApp;

#[actix_rt::test]
async fn add_persists_item_into_respository() {
    // Arrange
    let _app = TestApp::<PostgresOcieItemRepository>::spawn().await;
    let _item = OcieItem {
        id: Uuid::new_v4(),
        nsn: NationalStockNumber::parse("010-0000-00000-0000".into()).unwrap(),
        lin: LineItemNumber::parse("N12345".into()).unwrap(),
        nomenclature: "TEST ITEM".into(),
        size: Some("LARGE".into()),
        unit_of_issue: Some("EACH".into()),
        price: BigDecimal::from_f64(1.32f64),
    };

    // Act

    // Assert
}

#[actix_rt::test]
async fn get_all_returns_all_records() {
    // Arrange
    let test_app = TestApp::<PostgresOcieItemRepository>::spawn().await;

    // Act
    let response = reqwest::Client::new()
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
