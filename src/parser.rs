use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::*;

pub struct ProfileParser;

impl ProfileParser {
    pub fn find_latest_profile(project_path: &Path) -> Result<String> {
        let profile_dir = project_path.join("build/reports/profile");
        
        if !profile_dir.exists() {
            anyhow::bail!(
                "Profile directory not found at {:?}. Run './gradlew build --profile' first",
                profile_dir
            );
        }

        let mut profiles: Vec<_> = WalkDir::new(&profile_dir)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.path().is_file() && 
                e.path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "html")
                    .unwrap_or(false)
            })
            .collect();

        if profiles.is_empty() {
            anyhow::bail!("No profile files (*.html) found in {:?}", profile_dir);
        }

        // 수정된 부분: .ok()를 추가해서 Result를 Option으로 변환
        profiles.sort_by_cached_key(|entry| {
            entry
                .metadata()
                .ok()  // io::Result<Metadata> -> Option<Metadata>
                .and_then(|m| m.modified().ok())  // io::Result<SystemTime> -> Option<SystemTime>
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });

        profiles
            .last()
            .map(|e| e.path().display().to_string())
            .context("Failed to get latest profile file")
    }

    /// HTML 프로파일 파싱
    pub fn parse_html(file_path: &str) -> Result<BuildProfile> {
        let content = fs::read_to_string(file_path)
            .context(format!("Failed to read file: {}", file_path))?;
        
        // 총 빌드 시간 추출
        let total_time = Self::extract_total_time(&content)?;
        
        // 태스크 실행 데이터 추출
        let projects = Self::extract_tasks(&content)?;
        
        Ok(BuildProfile {
            build_profile: BuildData {
                elapsed_total: total_time,
                projects,
            },
        })
    }

    fn extract_total_time(html: &str) -> Result<u64> {
        // "Total Build Time" 다음의 duration 찾기
        let re = Regex::new(r#"Total Build Time</td>\s*<td class="numeric">([^<]+)</td>"#)
            .unwrap();
        
        if let Some(caps) = re.captures(html) {
            let duration_str = caps.get(1).unwrap().as_str();
            Ok(Self::parse_duration(duration_str))
        } else {
            anyhow::bail!("Could not find Total Build Time in HTML")
        }
    }

    fn extract_tasks(html: &str) -> Result<Vec<Project>> {
        let mut projects: std::collections::HashMap<String, Vec<Task>> = std::collections::HashMap::new();
        
        // Task Execution 섹션 찾기
        let task_section_start = html.find(r#"<h2>Task Execution</h2>"#)
            .context("Could not find Task Execution section")?;
        
        let task_section = &html[task_section_start..];
        
        // 태스크 라인 파싱: <td>프로젝트</td> 또는 <td class="indentPath">:프로젝트:태스크</td>
        let task_re = Regex::new(
            r#"<td(?:\s+class="indentPath")?>([^<]+)</td>\s*<td class="numeric">([^<]+)</td>\s*<td>([^<]*)</td>"#
        ).unwrap();
        
        for caps in task_re.captures_iter(task_section) {
            let task_path = caps.get(1).unwrap().as_str().trim();
            let duration_str = caps.get(2).unwrap().as_str().trim();
            let result = caps.get(3).unwrap().as_str().trim();
            
            // 프로젝트 총계는 무시 (예: ":app" without task name)
            if !task_path.contains(':') || task_path.matches(':').count() < 2 {
                continue;
            }
            
            // Duration이 "0s"나 "(total)"인 경우 무시
            if duration_str == "0s" || duration_str.contains("(total)") {
                continue;
            }
            
            let duration = Self::parse_duration(duration_str);
            
            // 프로젝트 이름 추출 (예: ":app:compileDebugKotlin" -> ":app")
            let parts: Vec<&str> = task_path.split(':').collect();
            let project_name = if parts.len() >= 2 {
                format!(":{}",  parts[1])
            } else {
                ":app".to_string()
            };
            
            let task = Task {
                path: task_path.to_string(),
                duration,
                result: if result.is_empty() { "SUCCESS".to_string() } else { result.to_string() },
                task_type: None,
            };
            
            projects.entry(project_name.clone())
                .or_insert_with(Vec::new)
                .push(task);
        }
        
        // HashMap을 Vec<Project>로 변환
        let mut project_list: Vec<Project> = projects
            .into_iter()
            .map(|(path, tasks)| Project { path, tasks })
            .collect();
        
        // 프로젝트 이름으로 정렬
        project_list.sort_by(|a, b| a.path.cmp(&b.path));
        
        if project_list.is_empty() {
            anyhow::bail!("Could not parse any tasks from HTML profile");
        }
        
        Ok(project_list)
    }
    
    fn parse_duration(duration_str: &str) -> u64 {
        let mut total_ms = 0u64;
        
        // "1m13.54s" 형식 파싱
        let min_re = Regex::new(r"(\d+)m").unwrap();
        let sec_re = Regex::new(r"(\d+(?:\.\d+)?)s").unwrap();
        
        // 분 추출
        if let Some(caps) = min_re.captures(duration_str) {
            let minutes: u64 = caps.get(1).unwrap().as_str().parse().unwrap_or(0);
            total_ms += minutes * 60 * 1000;
        }
        
        // 초 추출
        if let Some(caps) = sec_re.captures(duration_str) {
            let seconds: f64 = caps.get(1).unwrap().as_str().parse().unwrap_or(0.0);
            total_ms += (seconds * 1000.0) as u64;
        }
        
        total_ms
    }
}