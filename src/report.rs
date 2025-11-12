use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::models::*;

pub struct ReportGenerator;

impl ReportGenerator {
pub fn print_analysis(analysis: &BuildAnalysis) {
        println!("\n{}", "📊 Gradle Build Profile Analysis".bold().cyan());
        println!("{}", "=".repeat(60).dimmed());
        
        Self::print_summary(analysis);
        Self::print_slowest_tasks(&analysis.slowest_tasks);
        Self::print_project_summary(&analysis.project_summary);
        Self::print_optimization_tips(&analysis.optimization_tips);
    }

    fn print_summary(analysis: &BuildAnalysis) {
        println!("\n{}", "Build Summary".bold());
        println!("Total build time: {}", Self::format_duration(analysis.total_time).green());
        println!();
    }

    fn print_slowest_tasks(tasks: &[TaskSummary]) {
        println!("{}", "🐌 Top 10 Slowest Tasks".bold());
        
        #[derive(Tabled)]
        struct TaskRow {
            #[tabled(rename = "Task")]
            task: String,
            #[tabled(rename = "Duration")]
            duration: String,
            #[tabled(rename = "% of Total")]
            percentage: String,
        }

        let rows: Vec<TaskRow> = tasks
            .iter()
            .map(|t| TaskRow {
                task: t.name.clone(),
                duration: Self::format_duration(t.duration),
                percentage: format!("{:.1}%", t.percentage),
            })
            .collect();

        let table = Table::new(rows)
            .with(Style::rounded())
            .to_string();
        
        println!("{}\n", table);
    }

    fn print_project_summary(projects: &[ProjectSummary]) {
        println!("{}", "📦 Project Summary".bold());
        
        for project in projects {
            println!(
                "  {} - {} ({} tasks)",
                project.name.yellow(),
                Self::format_duration(project.total_duration),
                project.task_count
            );
        }
        println!();
    }

    fn print_optimization_tips(tips: &[String]) {
        println!("{}", "💡 Optimization Tips".bold());
        
        for tip in tips {
            println!("  {} {}", "•".green(), tip);
        }
        println!();
    }

    fn format_duration(millis: u64) -> String {
        let seconds = millis / 1000;
        let remaining_millis = millis % 1000;
        
        if seconds > 60 {
            let minutes = seconds / 60;
            let remaining_seconds = seconds % 60;
            format!("{}m {}s", minutes, remaining_seconds)
        } else {
            format!("{}.{}s", seconds, remaining_millis / 100)
        }
    }
}