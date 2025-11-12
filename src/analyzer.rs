use crate::models::*;

pub struct BuildAnalyzer;

impl BuildAnalyzer {
    pub fn analyze(profile: &BuildProfile) -> BuildAnalysis {
        let data = &profile.build_profile;
        let total_time = data.elapsed_total;

        let mut all_tasks: Vec<Task> = data
            .projects
            .iter()
            .flat_map(|p| p.tasks.clone())
            .collect();
        
        // 모든 task 수집
        all_tasks.sort_by(|a, b| b.duration.cmp(&a.duration));

        // 가장 느린 task top 10
        let slowest_tasks: Vec<TaskSummary> = all_tasks
            .iter()
            .take(10)
            .map(|task| TaskSummary {
                name: task.path.clone(),
                duration: task.duration,
                percentage: (task.duration as f64 / total_time as f64) * 100.0,
            })
            .collect();

        // 프로젝트 별 요약
        let project_summary: Vec<ProjectSummary> = data
            .projects
            .iter()
            .map(|project| {
                let total_duration: u64 = project.tasks.iter().map(|t| t.duration).sum();
                ProjectSummary {
                    name: project.path.clone(),
                    total_duration,
                    task_count: project.tasks.len(),
                }
            })
            .collect();

        // 최적화 팁 생성
        let optimization_tips = Self::generate_tips(&all_tasks, total_time);
        
        BuildAnalysis { total_time, slowest_tasks, project_summary, optimization_tips }
    }

    fn generate_tips(tasks: &[Task], total_time: u64) -> Vec<String> {
        let mut tips = Vec::new();

        // Kapt 사용 체크
        let kapt_tasks: Vec<&Task> = tasks
            .iter()
            .filter(|t| t.path.contains("kapt"))
            .collect();

        if !kapt_tasks.is_empty() {
            let kapt_time: u64 = kapt_tasks.iter().map(|t| t.duration).sum();
            let kapt_percentage = (kapt_time as f64 / total_time as f64) * 100.0;

            if kapt_percentage > 20.0 {
                tips.push(format!(
                    "Kapt takes {:.1}% of build time. Consider migrating to KSP for ~40% improvement",
                    kapt_percentage
                ));
            }
        }

        // Kotlin 컴파일 시간 체크
        let kotlin_compile: Vec<&Task> = tasks
            .iter()
            .filter(|t| t.path.contains("compileKotlin"))
            .collect();

        if !kotlin_compile.is_empty() {
            let longest_compile = kotlin_compile.iter().max_by_key(|t| t.duration);
            if let Some(task) = longest_compile {
                if task.duration > 30000 {
                    tips.push(format!(
                        "Module '{}' has slow Kotlin compilation. Consider splitting into smaller modules",
                        task.path.split(':').nth(1).unwrap_or("unknown")
                    ));
                }
            }
        }

        // 병렬화 가능성 체크
        tips.push("Enable Gradle parallel execution: org.gradle.parallel=true".to_string());
        tips.push("Enable configuration cache: org.gradle.configuration-cache=true".to_string());

        tips
    }
}