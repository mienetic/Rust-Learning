//! 🏗️ Infrastructure as Code - การจัดการโครงสร้างพื้นฐานด้วยโค้ด
//! 
//! โมดูลนี้สาธิตการสร้างและจัดการ infrastructure ด้วยโค้ด
//! รวมถึง Terraform, Kubernetes, Docker Compose, และ Cloud Formation

use std::collections::HashMap;
use std::fmt;

/// 🏗️ ประเภทของ Infrastructure Provider
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfraProvider {
    AWS,
    Azure,
    GCP,
    DigitalOcean,
    Kubernetes,
    Docker,
}

impl fmt::Display for InfraProvider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfraProvider::AWS => write!(f, "AWS"),
            InfraProvider::Azure => write!(f, "Azure"),
            InfraProvider::GCP => write!(f, "Google Cloud Platform"),
            InfraProvider::DigitalOcean => write!(f, "DigitalOcean"),
            InfraProvider::Kubernetes => write!(f, "Kubernetes"),
            InfraProvider::Docker => write!(f, "Docker"),
        }
    }
}

/// 🏗️ Infrastructure Resource
#[derive(Debug, Clone)]
pub struct InfraResource {
    pub name: String,
    pub resource_type: String,
    pub provider: InfraProvider,
    pub properties: HashMap<String, String>,
    pub dependencies: Vec<String>,
    pub tags: HashMap<String, String>,
}

impl InfraResource {
    /// สร้าง InfraResource ใหม่
    pub fn new(name: &str, resource_type: &str, provider: InfraProvider) -> Self {
        Self {
            name: name.to_string(),
            resource_type: resource_type.to_string(),
            provider,
            properties: HashMap::new(),
            dependencies: Vec::new(),
            tags: HashMap::new(),
        }
    }
    
    /// เพิ่ม property
    pub fn with_property(mut self, key: &str, value: &str) -> Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
    
    /// เพิ่ม dependency
    pub fn with_dependency(mut self, dependency: &str) -> Self {
        self.dependencies.push(dependency.to_string());
        self
    }
    
    /// เพิ่ม tag
    pub fn with_tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }
}

/// 🏗️ Terraform Configuration Generator
#[derive(Debug)]
pub struct TerraformConfig {
    pub provider_configs: HashMap<InfraProvider, HashMap<String, String>>,
    pub resources: Vec<InfraResource>,
    pub variables: HashMap<String, String>,
    pub outputs: HashMap<String, String>,
}

impl TerraformConfig {
    /// สร้าง TerraformConfig ใหม่
    pub fn new() -> Self {
        Self {
            provider_configs: HashMap::new(),
            resources: Vec::new(),
            variables: HashMap::new(),
            outputs: HashMap::new(),
        }
    }
    
    /// เพิ่ม provider configuration
    pub fn add_provider(&mut self, provider: InfraProvider, config: HashMap<String, String>) {
        self.provider_configs.insert(provider, config);
    }
    
    /// เพิ่ม resource
    pub fn add_resource(&mut self, resource: InfraResource) {
        self.resources.push(resource);
    }
    
    /// เพิ่ม variable
    pub fn add_variable(&mut self, name: &str, description: &str) {
        self.variables.insert(name.to_string(), description.to_string());
    }
    
    /// เพิ่ม output
    pub fn add_output(&mut self, name: &str, value: &str) {
        self.outputs.insert(name.to_string(), value.to_string());
    }
    
    /// สร้าง Terraform configuration file
    pub fn generate(&self) -> String {
        let mut tf = String::new();
        
        // Terraform version
        tf.push_str("terraform {\n");
        tf.push_str("  required_version = \">= 1.0\"\n");
        tf.push_str("  required_providers {\n");
        
        for provider in self.provider_configs.keys() {
            match provider {
                InfraProvider::AWS => {
                    tf.push_str("    aws = {\n");
                    tf.push_str("      source  = \"hashicorp/aws\"\n");
                    tf.push_str("      version = \"~> 5.0\"\n");
                    tf.push_str("    }\n");
                }
                InfraProvider::Azure => {
                    tf.push_str("    azurerm = {\n");
                    tf.push_str("      source  = \"hashicorp/azurerm\"\n");
                    tf.push_str("      version = \"~> 3.0\"\n");
                    tf.push_str("    }\n");
                }
                InfraProvider::GCP => {
                    tf.push_str("    google = {\n");
                    tf.push_str("      source  = \"hashicorp/google\"\n");
                    tf.push_str("      version = \"~> 4.0\"\n");
                    tf.push_str("    }\n");
                }
                _ => {}
            }
        }
        
        tf.push_str("  }\n");
        tf.push_str("}\n\n");
        
        // Provider configurations
        for (provider, config) in &self.provider_configs {
            match provider {
                InfraProvider::AWS => tf.push_str("provider \"aws\" {\n"),
                InfraProvider::Azure => tf.push_str("provider \"azurerm\" {\n"),
                InfraProvider::GCP => tf.push_str("provider \"google\" {\n"),
                _ => continue,
            }
            
            for (key, value) in config {
                tf.push_str(&format!("  {} = \"{}\"\n", key, value));
            }
            
            tf.push_str("}\n\n");
        }
        
        // Variables
        if !self.variables.is_empty() {
            for (name, description) in &self.variables {
                tf.push_str(&format!("variable \"{}\" {{\n", name));
                tf.push_str(&format!("  description = \"{}\"\n", description));
                tf.push_str("  type        = string\n");
                tf.push_str("}\n\n");
            }
        }
        
        // Resources
        for resource in &self.resources {
            let provider_prefix = match resource.provider {
                InfraProvider::AWS => "aws_",
                InfraProvider::Azure => "azurerm_",
                InfraProvider::GCP => "google_",
                _ => "",
            };
            
            tf.push_str(&format!("resource \"{}{}\", \"{}\" {{\n", 
                provider_prefix, resource.resource_type, resource.name));
            
            for (key, value) in &resource.properties {
                if value.starts_with("var.") || value.starts_with("${{") {
                    tf.push_str(&format!("  {} = {}\n", key, value));
                } else {
                    tf.push_str(&format!("  {} = \"{}\"\n", key, value));
                }
            }
            
            // Tags
            if !resource.tags.is_empty() {
                tf.push_str("  tags = {\n");
                for (key, value) in &resource.tags {
                    tf.push_str(&format!("    {} = \"{}\"\n", key, value));
                }
                tf.push_str("  }\n");
            }
            
            tf.push_str("}\n\n");
        }
        
        // Outputs
        if !self.outputs.is_empty() {
            for (name, value) in &self.outputs {
                tf.push_str(&format!("output \"{}\" {{\n", name));
                tf.push_str(&format!("  value = {}\n", value));
                tf.push_str("}\n\n");
            }
        }
        
        tf
    }
}

impl Default for TerraformConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// 🐳 Docker Compose Configuration
#[derive(Debug, Clone)]
pub struct DockerService {
    pub name: String,
    pub image: String,
    pub ports: Vec<String>,
    pub environment: HashMap<String, String>,
    pub volumes: Vec<String>,
    pub depends_on: Vec<String>,
    pub networks: Vec<String>,
    pub restart_policy: String,
}

impl DockerService {
    /// สร้าง DockerService ใหม่
    pub fn new(name: &str, image: &str) -> Self {
        Self {
            name: name.to_string(),
            image: image.to_string(),
            ports: Vec::new(),
            environment: HashMap::new(),
            volumes: Vec::new(),
            depends_on: Vec::new(),
            networks: Vec::new(),
            restart_policy: "unless-stopped".to_string(),
        }
    }
    
    /// เพิ่ม port mapping
    pub fn with_port(mut self, port_mapping: &str) -> Self {
        self.ports.push(port_mapping.to_string());
        self
    }
    
    /// เพิ่ม environment variable
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.environment.insert(key.to_string(), value.to_string());
        self
    }
    
    /// เพิ่ม volume
    pub fn with_volume(mut self, volume: &str) -> Self {
        self.volumes.push(volume.to_string());
        self
    }
    
    /// เพิ่ม dependency
    pub fn with_dependency(mut self, service: &str) -> Self {
        self.depends_on.push(service.to_string());
        self
    }
    
    /// เพิ่ม network
    pub fn with_network(mut self, network: &str) -> Self {
        self.networks.push(network.to_string());
        self
    }
}

/// 🐳 Docker Compose Generator
#[derive(Debug)]
pub struct DockerCompose {
    pub version: String,
    pub services: Vec<DockerService>,
    pub networks: HashMap<String, HashMap<String, String>>,
    pub volumes: HashMap<String, HashMap<String, String>>,
}

impl DockerCompose {
    /// สร้าง DockerCompose ใหม่
    pub fn new() -> Self {
        Self {
            version: "3.8".to_string(),
            services: Vec::new(),
            networks: HashMap::new(),
            volumes: HashMap::new(),
        }
    }
    
    /// เพิ่ม service
    pub fn add_service(&mut self, service: DockerService) {
        self.services.push(service);
    }
    
    /// เพิ่ม network
    pub fn add_network(&mut self, name: &str, config: HashMap<String, String>) {
        self.networks.insert(name.to_string(), config);
    }
    
    /// เพิ่ม volume
    pub fn add_volume(&mut self, name: &str, config: HashMap<String, String>) {
        self.volumes.insert(name.to_string(), config);
    }
    
    /// สร้าง docker-compose.yml
    pub fn generate(&self) -> String {
        let mut yaml = String::new();
        
        yaml.push_str(&format!("version: '{}'\n\n", self.version));
        
        // Services
        yaml.push_str("services:\n");
        for service in &self.services {
            yaml.push_str(&format!("  {}:\n", service.name));
            yaml.push_str(&format!("    image: {}\n", service.image));
            
            if !service.ports.is_empty() {
                yaml.push_str("    ports:\n");
                for port in &service.ports {
                    yaml.push_str(&format!("      - \"{}\"\n", port));
                }
            }
            
            if !service.environment.is_empty() {
                yaml.push_str("    environment:\n");
                for (key, value) in &service.environment {
                    yaml.push_str(&format!("      {}: {}\n", key, value));
                }
            }
            
            if !service.volumes.is_empty() {
                yaml.push_str("    volumes:\n");
                for volume in &service.volumes {
                    yaml.push_str(&format!("      - {}\n", volume));
                }
            }
            
            if !service.depends_on.is_empty() {
                yaml.push_str("    depends_on:\n");
                for dep in &service.depends_on {
                    yaml.push_str(&format!("      - {}\n", dep));
                }
            }
            
            if !service.networks.is_empty() {
                yaml.push_str("    networks:\n");
                for network in &service.networks {
                    yaml.push_str(&format!("      - {}\n", network));
                }
            }
            
            yaml.push_str(&format!("    restart: {}\n\n", service.restart_policy));
        }
        
        // Networks
        if !self.networks.is_empty() {
            yaml.push_str("networks:\n");
            for (name, config) in &self.networks {
                yaml.push_str(&format!("  {}:\n", name));
                if config.is_empty() {
                    yaml.push_str("    driver: bridge\n");
                } else {
                    for (key, value) in config {
                        yaml.push_str(&format!("    {}: {}\n", key, value));
                    }
                }
            }
            yaml.push_str("\n");
        }
        
        // Volumes
        if !self.volumes.is_empty() {
            yaml.push_str("volumes:\n");
            for (name, config) in &self.volumes {
                yaml.push_str(&format!("  {}:\n", name));
                if config.is_empty() {
                    yaml.push_str("    driver: local\n");
                } else {
                    for (key, value) in config {
                        yaml.push_str(&format!("    {}: {}\n", key, value));
                    }
                }
            }
        }
        
        yaml
    }
}

impl Default for DockerCompose {
    fn default() -> Self {
        Self::new()
    }
}

/// ☸️ Kubernetes Resource
#[derive(Debug, Clone)]
pub struct KubernetesResource {
    pub api_version: String,
    pub kind: String,
    pub metadata: HashMap<String, String>,
    pub spec: HashMap<String, String>,
}

impl KubernetesResource {
    /// สร้าง KubernetesResource ใหม่
    pub fn new(api_version: &str, kind: &str, name: &str) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("name".to_string(), name.to_string());
        
        Self {
            api_version: api_version.to_string(),
            kind: kind.to_string(),
            metadata,
            spec: HashMap::new(),
        }
    }
    
    /// เพิ่ม metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
    
    /// เพิ่ม spec
    pub fn with_spec(mut self, key: &str, value: &str) -> Self {
        self.spec.insert(key.to_string(), value.to_string());
        self
    }
}

/// ☸️ Kubernetes Manifest Generator
#[derive(Debug)]
pub struct KubernetesManifest {
    pub resources: Vec<KubernetesResource>,
}

impl KubernetesManifest {
    /// สร้าง KubernetesManifest ใหม่
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
        }
    }
    
    /// เพิ่ม resource
    pub fn add_resource(&mut self, resource: KubernetesResource) {
        self.resources.push(resource);
    }
    
    /// สร้าง Deployment
    pub fn add_deployment(&mut self, name: &str, image: &str, replicas: u32) {
        let deployment = KubernetesResource::new("apps/v1", "Deployment", name)
            .with_metadata("labels.app", name)
            .with_spec("replicas", &replicas.to_string())
            .with_spec("selector.matchLabels.app", name)
            .with_spec("template.metadata.labels.app", name)
            .with_spec("template.spec.containers[0].name", name)
            .with_spec("template.spec.containers[0].image", image)
            .with_spec("template.spec.containers[0].ports[0].containerPort", "8080");
        
        self.add_resource(deployment);
    }
    
    /// สร้าง Service
    pub fn add_service(&mut self, name: &str, target_port: u16, service_port: u16) {
        let service = KubernetesResource::new("v1", "Service", name)
            .with_metadata("labels.app", name)
            .with_spec("selector.app", name)
            .with_spec("ports[0].port", &service_port.to_string())
            .with_spec("ports[0].targetPort", &target_port.to_string())
            .with_spec("type", "ClusterIP");
        
        self.add_resource(service);
    }
    
    /// สร้าง YAML manifest
    pub fn generate(&self) -> String {
        let mut yaml = String::new();
        
        for (i, resource) in self.resources.iter().enumerate() {
            if i > 0 {
                yaml.push_str("---\n");
            }
            
            yaml.push_str(&format!("apiVersion: {}\n", resource.api_version));
            yaml.push_str(&format!("kind: {}\n", resource.kind));
            
            yaml.push_str("metadata:\n");
            for (key, value) in &resource.metadata {
                if key.contains('.') {
                    let parts: Vec<&str> = key.split('.').collect();
                    yaml.push_str(&format!("  {}:\n", parts[0]));
                    yaml.push_str(&format!("    {}: {}\n", parts[1], value));
                } else {
                    yaml.push_str(&format!("  {}: {}\n", key, value));
                }
            }
            
            if !resource.spec.is_empty() {
                yaml.push_str("spec:\n");
                for (key, value) in &resource.spec {
                    if key.contains('.') {
                        // Simplified nested key handling
                        let parts: Vec<&str> = key.split('.').collect();
                        if parts.len() == 2 {
                            yaml.push_str(&format!("  {}:\n", parts[0]));
                            yaml.push_str(&format!("    {}: {}\n", parts[1], value));
                        } else {
                            yaml.push_str(&format!("  {}: {}\n", key, value));
                        }
                    } else {
                        yaml.push_str(&format!("  {}: {}\n", key, value));
                    }
                }
            }
            
            yaml.push_str("\n");
        }
        
        yaml
    }
}

impl Default for KubernetesManifest {
    fn default() -> Self {
        Self::new()
    }
}

/// 🏗️ Infrastructure Template
#[derive(Debug)]
pub struct InfrastructureTemplate {
    pub name: String,
    pub description: String,
    pub provider: InfraProvider,
    pub terraform_config: Option<TerraformConfig>,
    pub docker_compose: Option<DockerCompose>,
    pub kubernetes_manifest: Option<KubernetesManifest>,
}

impl InfrastructureTemplate {
    /// สร้าง InfrastructureTemplate ใหม่
    pub fn new(name: &str, description: &str, provider: InfraProvider) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            provider,
            terraform_config: None,
            docker_compose: None,
            kubernetes_manifest: None,
        }
    }
    
    /// เพิ่ม Terraform configuration
    pub fn with_terraform(mut self, config: TerraformConfig) -> Self {
        self.terraform_config = Some(config);
        self
    }
    
    /// เพิ่ม Docker Compose configuration
    pub fn with_docker_compose(mut self, compose: DockerCompose) -> Self {
        self.docker_compose = Some(compose);
        self
    }
    
    /// เพิ่ม Kubernetes manifest
    pub fn with_kubernetes(mut self, manifest: KubernetesManifest) -> Self {
        self.kubernetes_manifest = Some(manifest);
        self
    }
    
    /// สร้างไฟล์ทั้งหมด
    pub fn generate_all(&self) -> HashMap<String, String> {
        let mut files = HashMap::new();
        
        if let Some(ref tf_config) = self.terraform_config {
            files.insert("main.tf".to_string(), tf_config.generate());
        }
        
        if let Some(ref compose) = self.docker_compose {
            files.insert("docker-compose.yml".to_string(), compose.generate());
        }
        
        if let Some(ref k8s) = self.kubernetes_manifest {
            files.insert("kubernetes.yaml".to_string(), k8s.generate());
        }
        
        files
    }
}

/// 🎯 สาธิตการทำงานกับ Infrastructure as Code
pub fn demonstrate_infrastructure_as_code() {
    println!("\n🏗️ === Infrastructure as Code Demo ===");
    
    // 1. Terraform Configuration
    println!("\n1️⃣ Terraform Configuration:");
    demonstrate_terraform();
    
    // 2. Docker Compose
    println!("\n2️⃣ Docker Compose:");
    demonstrate_docker_compose();
    
    // 3. Kubernetes Manifests
    println!("\n3️⃣ Kubernetes Manifests:");
    demonstrate_kubernetes();
    
    // 4. Infrastructure Templates
    println!("\n4️⃣ Infrastructure Templates:");
    demonstrate_infrastructure_templates();
    
    // 5. IaC Best Practices
    println!("\n5️⃣ IaC Best Practices:");
    show_iac_best_practices();
    
    println!("\n✅ จบการสาธิต Infrastructure as Code!");
}

/// 🏗️ สาธิต Terraform
fn demonstrate_terraform() {
    println!("🏗️ การสร้าง Terraform Configuration:");
    
    let mut tf_config = TerraformConfig::new();
    
    // Provider configuration
    let mut aws_config = HashMap::new();
    aws_config.insert("region".to_string(), "us-west-2".to_string());
    tf_config.add_provider(InfraProvider::AWS, aws_config);
    
    // Variables
    tf_config.add_variable("instance_type", "EC2 instance type");
    tf_config.add_variable("key_name", "EC2 Key Pair name");
    
    // Resources
    let vpc = InfraResource::new("main_vpc", "vpc", InfraProvider::AWS)
        .with_property("cidr_block", "10.0.0.0/16")
        .with_property("enable_dns_hostnames", "true")
        .with_tag("Name", "Main VPC")
        .with_tag("Environment", "production");
    
    let subnet = InfraResource::new("main_subnet", "subnet", InfraProvider::AWS)
        .with_property("vpc_id", "${aws_vpc.main_vpc.id}")
        .with_property("cidr_block", "10.0.1.0/24")
        .with_property("availability_zone", "us-west-2a")
        .with_dependency("main_vpc")
        .with_tag("Name", "Main Subnet");
    
    let instance = InfraResource::new("web_server", "instance", InfraProvider::AWS)
        .with_property("ami", "ami-0c02fb55956c7d316")
        .with_property("instance_type", "var.instance_type")
        .with_property("key_name", "var.key_name")
        .with_property("subnet_id", "${aws_subnet.main_subnet.id}")
        .with_dependency("main_subnet")
        .with_tag("Name", "Web Server")
        .with_tag("Environment", "production");
    
    tf_config.add_resource(vpc);
    tf_config.add_resource(subnet);
    tf_config.add_resource(instance);
    
    // Outputs
    tf_config.add_output("instance_ip", "${aws_instance.web_server.public_ip}");
    tf_config.add_output("vpc_id", "${aws_vpc.main_vpc.id}");
    
    let terraform_code = tf_config.generate();
    
    println!("\n📄 main.tf (ตัวอย่าง):");
    let lines: Vec<&str> = terraform_code.lines().take(15).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
    
    println!("\n🔧 การใช้งาน Terraform:");
    println!("   • terraform init - เริ่มต้น Terraform");
    println!("   • terraform plan - ดูแผนการเปลี่ยนแปลง");
    println!("   • terraform apply - ปรับใช้การเปลี่ยนแปลง");
    println!("   • terraform destroy - ลบ infrastructure");
}

/// 🐳 สาธิต Docker Compose
fn demonstrate_docker_compose() {
    println!("🐳 การสร้าง Docker Compose:");
    
    let mut compose = DockerCompose::new();
    
    // Web service
    let web_service = DockerService::new("web", "nginx:alpine")
        .with_port("80:80")
        .with_port("443:443")
        .with_volume("./nginx.conf:/etc/nginx/nginx.conf")
        .with_volume("./ssl:/etc/nginx/ssl")
        .with_dependency("app")
        .with_network("frontend");
    
    // App service
    let app_service = DockerService::new("app", "rust-app:latest")
        .with_port("8080:8080")
        .with_env("DATABASE_URL", "postgresql://postgres:password@db:5432/myapp")
        .with_env("REDIS_URL", "redis://redis:6379")
        .with_volume("./app-data:/app/data")
        .with_dependency("db")
        .with_dependency("redis")
        .with_network("frontend")
        .with_network("backend");
    
    // Database service
    let db_service = DockerService::new("db", "postgres:15")
        .with_port("5432:5432")
        .with_env("POSTGRES_DB", "myapp")
        .with_env("POSTGRES_USER", "postgres")
        .with_env("POSTGRES_PASSWORD", "password")
        .with_volume("postgres-data:/var/lib/postgresql/data")
        .with_network("backend");
    
    // Redis service
    let redis_service = DockerService::new("redis", "redis:alpine")
        .with_port("6379:6379")
        .with_volume("redis-data:/data")
        .with_network("backend");
    
    compose.add_service(web_service);
    compose.add_service(app_service);
    compose.add_service(db_service);
    compose.add_service(redis_service);
    
    // Networks
    compose.add_network("frontend", HashMap::new());
    compose.add_network("backend", HashMap::new());
    
    // Volumes
    compose.add_volume("postgres-data", HashMap::new());
    compose.add_volume("redis-data", HashMap::new());
    
    let compose_yaml = compose.generate();
    
    println!("\n📄 docker-compose.yml (ตัวอย่าง):");
    let lines: Vec<&str> = compose_yaml.lines().take(20).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
    
    println!("\n🔧 การใช้งาน Docker Compose:");
    println!("   • docker-compose up -d - เริ่มต้น services");
    println!("   • docker-compose down - หยุด services");
    println!("   • docker-compose logs - ดู logs");
    println!("   • docker-compose scale app=3 - scale service");
}

/// ☸️ สาธิต Kubernetes
fn demonstrate_kubernetes() {
    println!("☸️ การสร้าง Kubernetes Manifests:");
    
    let mut k8s = KubernetesManifest::new();
    
    // Deployment
    k8s.add_deployment("rust-app", "rust-app:latest", 3);
    
    // Service
    k8s.add_service("rust-app-service", 8080, 80);
    
    // ConfigMap
    let configmap = KubernetesResource::new("v1", "ConfigMap", "rust-app-config")
        .with_metadata("labels.app", "rust-app")
        .with_spec("data.DATABASE_URL", "postgresql://postgres:5432/myapp")
        .with_spec("data.REDIS_URL", "redis://redis:6379");
    
    k8s.add_resource(configmap);
    
    // Secret
    let secret = KubernetesResource::new("v1", "Secret", "rust-app-secret")
        .with_metadata("labels.app", "rust-app")
        .with_spec("type", "Opaque")
        .with_spec("data.DB_PASSWORD", "cGFzc3dvcmQ="); // base64 encoded "password"
    
    k8s.add_resource(secret);
    
    let k8s_yaml = k8s.generate();
    
    println!("\n📄 kubernetes.yaml (ตัวอย่าง):");
    let lines: Vec<&str> = k8s_yaml.lines().take(15).collect();
    for line in lines {
        println!("   {}", line);
    }
    println!("   ... (และอีกหลายบรรทัด)");
    
    println!("\n🔧 การใช้งาน Kubernetes:");
    println!("   • kubectl apply -f kubernetes.yaml - ปรับใช้ manifests");
    println!("   • kubectl get pods - ดู pods");
    println!("   • kubectl get services - ดู services");
    println!("   • kubectl logs deployment/rust-app - ดู logs");
    println!("   • kubectl scale deployment rust-app --replicas=5 - scale");
}

/// 🏗️ สาธิต Infrastructure Templates
fn demonstrate_infrastructure_templates() {
    println!("🏗️ การสร้าง Infrastructure Templates:");
    
    // สร้าง template สำหรับ web application
    let mut tf_config = TerraformConfig::new();
    let mut aws_config = HashMap::new();
    aws_config.insert("region".to_string(), "us-west-2".to_string());
    tf_config.add_provider(InfraProvider::AWS, aws_config);
    
    let mut compose = DockerCompose::new();
    let web_service = DockerService::new("web", "nginx:alpine")
        .with_port("80:80");
    compose.add_service(web_service);
    
    let mut k8s = KubernetesManifest::new();
    k8s.add_deployment("web-app", "web-app:latest", 2);
    k8s.add_service("web-app-service", 8080, 80);
    
    let template = InfrastructureTemplate::new(
        "web-application",
        "Complete web application infrastructure",
        InfraProvider::AWS,
    )
    .with_terraform(tf_config)
    .with_docker_compose(compose)
    .with_kubernetes(k8s);
    
    let files = template.generate_all();
    
    println!("\n📁 Generated Files:");
    for (filename, content) in &files {
        println!("   📄 {}: {} บรรทัด", filename, content.lines().count());
    }
    
    println!("\n🎯 ประโยชน์ของ Infrastructure Templates:");
    println!("   • สร้าง infrastructure ได้อย่างรวดเร็ว");
    println!("   • รับประกันความสอดคล้องกัน");
    println!("   • ลดข้อผิดพลาดจากการกำหนดค่าด้วยมือ");
    println!("   • สามารถนำกลับมาใช้ใหม่ได้");
    println!("   • รองรับ version control");
}

/// 📋 แสดง IaC Best Practices
fn show_iac_best_practices() {
    println!("📋 Infrastructure as Code Best Practices:");
    
    let practices = vec![
        ("📝", "Version Control", "เก็บ IaC code ใน Git repository"),
        ("🔄", "Immutable Infrastructure", "สร้าง infrastructure ใหม่แทนการแก้ไข"),
        ("🧪", "Testing", "ทดสอบ infrastructure code ก่อนปรับใช้"),
        ("🔒", "Security", "ใช้ secrets management และ least privilege"),
        ("📊", "Monitoring", "ติดตาม infrastructure changes และ drift"),
        ("🏷️", "Tagging", "ใช้ tags สำหรับการจัดการและ cost tracking"),
        ("🔄", "CI/CD Integration", "รวม IaC เข้ากับ CI/CD pipeline"),
        ("📚", "Documentation", "เขียน documentation สำหรับ infrastructure"),
    ];
    
    for (icon, title, description) in practices {
        println!("   {} {}: {}", icon, title, description);
    }
    
    println!("\n🎯 Terraform Best Practices:");
    println!("   • ใช้ remote state backend (S3, Terraform Cloud)");
    println!("   • แยก environments ด้วย workspaces หรือ directories");
    println!("   • ใช้ modules สำหรับ reusable components");
    println!("   • กำหนด resource naming conventions");
    println!("   • ใช้ terraform fmt และ terraform validate");
    
    println!("\n🐳 Docker/Kubernetes Best Practices:");
    println!("   • ใช้ multi-stage builds สำหรับ Docker images");
    println!("   • กำหนด resource limits และ requests");
    println!("   • ใช้ health checks และ readiness probes");
    println!("   • จัดการ secrets ด้วย Kubernetes Secrets");
    println!("   • ใช้ namespaces สำหรับ isolation");
    
    println!("\n🔧 การจัดการ Infrastructure State:");
    println!("   • ใช้ state locking เพื่อป้องกัน concurrent changes");
    println!("   • Backup state files อย่างสม่ำเสมอ");
    println!("   • ใช้ state encryption สำหรับข้อมูลที่ sensitive");
    println!("   • ตรวจสอบ state drift อย่างสม่ำเสมอ");
    println!("   • ใช้ import สำหรับ existing resources");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_infra_resource() {
        let resource = InfraResource::new("test_vpc", "vpc", InfraProvider::AWS)
            .with_property("cidr_block", "10.0.0.0/16")
            .with_tag("Environment", "test");
        
        assert_eq!(resource.name, "test_vpc");
        assert_eq!(resource.resource_type, "vpc");
        assert_eq!(resource.provider, InfraProvider::AWS);
        assert_eq!(resource.properties.get("cidr_block"), Some(&"10.0.0.0/16".to_string()));
        assert_eq!(resource.tags.get("Environment"), Some(&"test".to_string()));
    }
    
    #[test]
    fn test_terraform_config() {
        let mut tf_config = TerraformConfig::new();
        
        let mut aws_config = HashMap::new();
        aws_config.insert("region".to_string(), "us-west-2".to_string());
        tf_config.add_provider(InfraProvider::AWS, aws_config);
        
        let resource = InfraResource::new("test_instance", "instance", InfraProvider::AWS);
        tf_config.add_resource(resource);
        
        let generated = tf_config.generate();
        assert!(generated.contains("terraform {"));
        assert!(generated.contains("provider \"aws\""));
        assert!(generated.contains("resource \"aws_instance\""));
    }
    
    #[test]
    fn test_docker_service() {
        let service = DockerService::new("web", "nginx:alpine")
            .with_port("80:80")
            .with_env("ENV", "production")
            .with_volume("./data:/app/data");
        
        assert_eq!(service.name, "web");
        assert_eq!(service.image, "nginx:alpine");
        assert_eq!(service.ports.len(), 1);
        assert_eq!(service.environment.len(), 1);
        assert_eq!(service.volumes.len(), 1);
    }
    
    #[test]
    fn test_docker_compose() {
        let mut compose = DockerCompose::new();
        let service = DockerService::new("app", "app:latest");
        compose.add_service(service);
        
        let generated = compose.generate();
        assert!(generated.contains("version: '3.8'"));
        assert!(generated.contains("services:"));
        assert!(generated.contains("app:"));
        assert!(generated.contains("image: app:latest"));
    }
    
    #[test]
    fn test_kubernetes_resource() {
        let resource = KubernetesResource::new("v1", "Service", "test-service")
            .with_metadata("labels.app", "test")
            .with_spec("type", "ClusterIP");
        
        assert_eq!(resource.api_version, "v1");
        assert_eq!(resource.kind, "Service");
        assert_eq!(resource.metadata.get("name"), Some(&"test-service".to_string()));
        assert_eq!(resource.spec.get("type"), Some(&"ClusterIP".to_string()));
    }
    
    #[test]
    fn test_kubernetes_manifest() {
        let mut k8s = KubernetesManifest::new();
        k8s.add_deployment("test-app", "test:latest", 2);
        k8s.add_service("test-service", 8080, 80);
        
        let generated = k8s.generate();
        assert!(generated.contains("apiVersion: apps/v1"));
        assert!(generated.contains("kind: Deployment"));
        assert!(generated.contains("kind: Service"));
        assert!(generated.contains("test-app"));
    }
}