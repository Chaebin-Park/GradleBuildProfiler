use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildProfile {
    #[serde(rename = "buildProfile")]
    pub build_profile: BuildData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BuildData {
    #[serde(rename = "elapsedTotal")]
    pub elapsed_total: u64,
    pub projects: Vec<Project>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    pub path: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub path: String,
    pub duration: u64,
    pub result: String,
    #[serde(rename = "type")]
    pub task_type: Option<String>,
}

// 분석 결과 구조체
#[derive(Debug)]
pub struct BuildAnalysis {
    pub total_time: u64,
    pub slowest_tasks: Vec<TaskSummary>,
    pub project_summary: Vec<ProjectSummary>,
    pub optimization_tips: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TaskSummary {
    pub name: String,
    pub duration: u64,
    pub percentage: f64,
}

#[derive(Debug)]
pub struct ProjectSummary {
    pub name: String,
    pub total_duration: u64,
    pub task_count: usize,
}