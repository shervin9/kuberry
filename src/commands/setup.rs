use std::process::Command;
use std::io::{self, Write};

pub fn setup_provider() -> Result<(), Box<dyn std::error::Error>> {
    println!("Select the Kubernetes provider to set up:");
    println!("1. Google Kubernetes Engine (GKE)");
    println!("2. Amazon Elastic Kubernetes Service (EKS)");
    println!("3. Azure Kubernetes Service (AKS)");
    println!("4. Local Kubernetes");

    print!("Enter the number of your choice: ");
    io::stdout().flush()?; // Ensure prompt is displayed

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim();

    match choice {
        "1" => setup_gke()?,
        "2" => setup_eks()?,
        "3" => setup_aks()?,
        "4" => setup_local_kubernetes()?,
        _ => println!("Invalid choice. Please select a valid provider."),
    }

    Ok(())
}

fn setup_gke() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up GKE...");
    Command::new("gcloud")
        .args(&["auth", "login"])
        .status()?;
    Command::new("gcloud")
        .args(&["container", "clusters", "get-credentials", "CLUSTER_NAME", "--region", "REGION"])
        .status()?;
    println!("GKE setup complete!");
    Ok(())
}

fn setup_eks() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up EKS...");
    Command::new("aws")
        .args(&["configure"])
        .status()?; // Configure AWS CLI credentials
    Command::new("aws")
        .args(&[
            "eks",
            "update-kubeconfig",
            "--name",
            "CLUSTER_NAME",
            "--region",
            "REGION",
        ])
        .status()?; // Update kubeconfig with EKS credentials
    println!("EKS setup complete!");
    Ok(())
}

fn setup_aks() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up AKS...");
    Command::new("az")
        .args(&["login"])
        .status()?; // Log in to Azure
    Command::new("az")
        .args(&[
            "aks",
            "get-credentials",
            "--resource-group",
            "RESOURCE_GROUP",
            "--name",
            "CLUSTER_NAME",
        ])
        .status()?; // Update kubeconfig with AKS credentials
    println!("AKS setup complete!");
    Ok(())
}

fn setup_local_kubernetes() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up Local Kubernetes...");
    // Ensure `kubectl` is installed and configured
    Command::new("kubectl")
        .args(&["config", "view"])
        .status()?; // Validate kubeconfig for local Kubernetes
    println!("Local Kubernetes setup complete!");
    Ok(())
}