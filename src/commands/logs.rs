use kube::{api::{ListParams, LogParams}, Api, Client};
use k8s_openapi::api::core::v1::Pod;
use regex::Regex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures::stream::{self, StreamExt};

pub async fn fetch_logs(
    deployment_name: &str,
    keyword: Option<&str>,
    output_file: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::default_namespaced(client);

    println!("Fetching logs for deployment: {}", deployment_name);
    let list_params = ListParams::default().labels(&format!("app={}", deployment_name));
    let pod_list = pods.list(&list_params).await?;

    let pod_names: Vec<String> = pod_list
        .items
        .into_iter()
        .filter_map(|pod| pod.metadata.name)
        .collect();

    if pod_names.is_empty() {
        eprintln!("No pods found for deployment: {}", deployment_name);
        return Ok(());
    }

    let log_params = LogParams {
        follow: false,
        ..Default::default()
    };

    let regex = keyword.map(|kw| Regex::new(kw)).transpose()?;

    let combined_logs = stream::iter(pod_names)
        .then(|pod_name| {
            // Clone `pod_name` to avoid borrowing issues.
            let pod_name_cloned = pod_name.clone();
            fetch_and_filter_logs(&pods, pod_name_cloned, &log_params, regex.as_ref())
        })
        .filter_map(|logs| async { logs })
        .collect::<Vec<String>>()
        .await
        .join("\n");

    if combined_logs.is_empty() {
        eprintln!("No matching logs found.");
        return Ok(());
    }

    if output_file {
        let file_name = format!("{}_logs.txt", deployment_name);
        let mut file = File::create(&file_name).await?;
        file.write_all(combined_logs.as_bytes()).await?;
        println!("Logs saved to file: {}", file_name);
    } else {
        println!("{}", combined_logs);
    }

    Ok(())
}

async fn fetch_and_filter_logs(
    pods: &Api<Pod>,
    pod_name: String,
    log_params: &LogParams,
    regex: Option<&Regex>,
) -> Option<String> {
    match pods.logs(&pod_name, log_params).await {
        Ok(logs) => filter_logs(&pod_name, &logs, regex),
        Err(err) => {
            eprintln!("Failed to fetch logs for pod {}: {}", pod_name, err);
            None
        }
    }
}

fn filter_logs(pod_name: &str, logs: &str, regex: Option<&Regex>) -> Option<String> {
    let filtered_logs: String = logs
        .lines()
        .filter_map(|line| {
            if regex.map_or(true, |re| re.is_match(line)) {
                Some(format!("[{}]: {}", pod_name, line))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if filtered_logs.is_empty() {
        None
    } else {
        Some(filtered_logs)
    }
}