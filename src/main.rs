use clap::{Parser, Subcommand};
use crate::commands::{list_pods, list_deployments, list_statefulsets,fetch_logs,setup_provider};
mod commands;

#[derive(Parser)]
#[command(name = "kberry", version = "0.1.0", about = "Kubernetes CLI tool developed by Shervin")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List Pods in the cluster
    ListPods {
        /// Specify the namespace (optional)
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },
    /// List Deployments in the cluster
    ListDeployments {
        /// Specify the namespace (optional)
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },
    /// List StatefulSets in the cluster
    ListStatefulsets {
        /// Specify the namespace (optional)
        #[arg(short, long, default_value = "default")]
        namespace: String,
    },
    FetchLogs {
        #[arg(short, long, help = "Name of the deployment")]
        deployment: String,

        #[arg(short, long, help = "Keyword to filter logs")]
        keyword: Option<String>,

        #[arg(short = 'o', long, help = "Save logs to a file")]
        output_file: bool,
    },
    Setup,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::ListPods { namespace } => list_pods(&namespace).await?,
        Commands::ListDeployments { namespace } => list_deployments(&namespace).await?,
        Commands::ListStatefulsets { namespace } => list_statefulsets(&namespace).await?,
        Commands::FetchLogs {
            deployment,
            keyword,
            output_file,
        } => {
            fetch_logs(&deployment, keyword.as_deref(), output_file).await?;
        }
        Commands::Setup => setup_provider()?,
    }

    Ok(())
}