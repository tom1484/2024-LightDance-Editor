use std::time::Instant;
use dotenv::var;
use sqlx::MySqlPool;

#[derive(Debug)]
struct LED {
    color_id: i32,
    alpha: i32,
}

#[tokio::test]
pub async fn setup_benchmark() -> Result<(), sqlx::Error> {

    let host = var("DATABASE_URL").expect("MYSQL_HOST is not set");
    let pool = MySqlPool::connect(host.as_str())
        .await
        .expect("Failed to create mysql pool");

    let start = Instant::now();
    for i in 1..=1000 {
        let mut leds_json_array: Vec<(i32, i32)> = Vec::new();
        let mut leds: Vec<(i32, i32, i32, i32)> = Vec::new();
        for j in 1..=20 {
            leds_json_array.push((0, j));
            leds.push((i, j, 0, j));
        }
        let leds_json = serde_json::to_string(&leds_json_array).unwrap();

        let _ = sqlx::query!(
            r#"
                INSERT INTO `led_effect` (`name`, `part_name`, `repeat`, `frames`)
                VALUES (?, ?, ?, ?)
            "#,
            i.to_string(),
            "test",
            1,
            leds_json
        )
        .execute(&pool)
        .await?;

        let _ = sqlx::query!(
            r#"
                INSERT INTO `led_effect_test` (`name`, `part_name`, `repeat`)
                VALUES (?, ?, ?)
            "#,
            i.to_string(),
            "test",
            1
        )
        .execute(&pool)
        .await?;

        for (effect_id, position, color_id, alpha) in leds {
            let _ = sqlx::query!(
                r#"
                    INSERT INTO `led` (`effect_id`, `position`, `color_id`, `alpha`)
                    VALUES (?, ?, ?, ?)
                "#,
                effect_id,
                position,
                color_id,
                alpha
            )
            .execute(&pool)
            .await?;
        }
    }
    println!("Inserting 100 effects took {:?}", start.elapsed());

    Ok(())
}

// #[sqlx::test(migrations = "tests/migrations")]
// pub async fn run(pool: sqlx::MySqlPool) -> sqlx::Result<()> {
#[tokio::test]
pub async fn run_benchmark() -> Result<(), sqlx::Error> {

    let host = var("DATABASE_URL").expect("MYSQL_HOST is not set");
    let pool = MySqlPool::connect(host.as_str())
        .await
        .expect("Failed to create mysql pool");

    let start = Instant::now();
    let led_data: Vec::<LED> = sqlx::query_as!(
        LED,
        r#"
            SELECT color_id, alpha FROM led
            WHERE effect_id = 1
            ORDER BY position ASC;
        "#
    )
    .fetch_all(&pool)
    .await?;
    println!("Fetched {:?} rows", led_data.len());
    println!("Fetching LEDs by table took {:?}", start.elapsed());

    let start = Instant::now();
    let mut led_data_from_json = Vec::<LED>::new();
    let frames: sqlx::types::JsonValue = sqlx::query_scalar!(
        r#"
            SELECT frames FROM led_effect WHERE id = 1 LIMIT 1;
        "#
    )
    .fetch_one(&pool)
    .await?;
    if let Some(frames) = frames.as_array() {
        for frame in frames {
            if let Some(leds) = frame.as_array() {
                led_data_from_json.push(LED {
                    color_id: leds[0].as_i64().unwrap() as i32,
                    alpha: leds[1].as_i64().unwrap() as i32,
                })
            }
        }
    }
    println!("Fetched {:?} rows", led_data_from_json.len());
    println!("Fetching LEDs by JSON took {:?}", start.elapsed());

    // println!("Effect data: {:?}", led_data);
    println!("Effect data: {:?}", led_data_from_json);

    Ok(())
}
