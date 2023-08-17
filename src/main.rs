use clap::{App, Arg};
use kube::{Client, api::{Api, ListParams}, ResourceExt, Resource};
use kube::api::Pod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Kubernetes Namespace Viewer")
        .version("1.0")
        .author("Your Name")
        .about("View all resources in a specific Kubernetes namespace")
        .arg(
            Arg::with_name("NAMESPACE")
                .help("The namespace to view")
                .required(true)
                .index(1),
        )
        .get_matches();

    let namespace = matches.value_of("NAMESPACE").unwrap();

    // Create a Kubernetes client
    let client = Client::try_default().await?;

    // Define the API for Pods within the specified namespace
    let pods: Api<Pod> = Api::namespaced(client, namespace);

    // Fetch the Pods
    let lp = ListParams::default();
    let pod_list = pods.list(&lp).await?;

    // Print the Pods
    println!("Pods in namespace '{}':", namespace);
    for pod in pod_list {
        println!("  - {}", pod.name());
    }

    Ok(())
}
