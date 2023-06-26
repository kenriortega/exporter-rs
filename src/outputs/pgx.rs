use crate::outputs::{LogType, Output};
use sqlx::types::Json;

pub struct Postgres;

impl Output<Postgres> {
    pub async fn send_data(&self) {
        match &self.data_received {
            LogType::LogEntryApache(data) => {
                let _ =sqlx::query!(
                    r#"
                INSERT INTO exporter_rs_logs_apache ( log_raw )
                VALUES ( $1 )
                RETURNING  log_raw
                        "#,
                    Json(data) as _
                )
                .fetch_one(&self.cfg.pgx_opts.pool)
                .await;
            }
            LogType::LogEntryIIS(data) => {
                let _ =sqlx::query!(
                    r#"
                INSERT INTO exporter_rs_logs_iis ( log_raw )
                VALUES ( $1 )
                RETURNING  log_raw
                        "#,
                    Json(data) as _
                )
                .fetch_one(&self.cfg.pgx_opts.pool)
                .await;
            }
            LogType::LogEntryNginx(data) => {
                let _ =sqlx::query!(
                    r#"
                INSERT INTO exporter_rs_logs_nginx ( log_raw )
                VALUES ( $1 )
                RETURNING  log_raw
                        "#,
                    Json(data) as _
                )
                .fetch_one(&self.cfg.pgx_opts.pool)
                .await;
            }
        }
    }
}
