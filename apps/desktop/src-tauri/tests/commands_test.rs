use app_lib::commands::{
    create_milestone, create_project, create_task, delete_milestone, delete_project, delete_task,
    get_milestones, get_projects, get_tasks, get_tasks_by_project, update_milestone_status,
    update_project, update_task_status,
};
use app_lib::persistence::Database;
use tauri::Manager;
use tempfile::tempdir;

fn setup_test_app() -> (tauri::App<tauri::test::MockRuntime>, Database) {
    let tmp = tempdir().unwrap();
    let db_path = tmp.keep();
    let db = Database::new(&db_path).unwrap();
    let app = tauri::test::mock_app();
    app.manage(Database {
        pool: db.pool.clone(),
    });
    let db_clone = Database {
        pool: db.pool.clone(),
    };
    (app, db_clone)
}

#[test]
fn test_database_initialization_and_migrations() {
    let tmp = tempdir().unwrap();
    let db = Database::new(tmp.path());
    assert!(
        db.is_ok(),
        "Database should initialize successfully in a temporary directory"
    );
}

#[test]
fn test_project_crud_and_cascade() {
    let (app, db) = setup_test_app();
    let db_state = app.state::<Database>();

    // 1. Create a project
    let proj = create_project(
        "Test Project".to_string(),
        "/path/to/test".to_string(),
        Some("A description of the test project".to_string()),
        db_state.clone(),
    )
    .expect("Failed to create project");

    assert_eq!(proj.name, "Test Project");
    assert_eq!(proj.path, "/path/to/test");
    assert_eq!(
        proj.description,
        Some("A description of the test project".to_string())
    );

    // 2. Read projects
    let projects = get_projects(db_state.clone()).expect("Failed to get projects");
    assert!(!projects.is_empty(), "Projects list should not be empty");
    assert_eq!(projects[0].id, proj.id);

    // 3. Update project
    update_project(
        proj.id.clone(),
        Some("Updated Project Name".to_string()),
        None,
        Some("Updated Description".to_string()),
        db_state.clone(),
    )
    .expect("Failed to update project");

    let projects_updated = get_projects(db_state.clone()).expect("Failed to get projects");
    assert_eq!(projects_updated[0].name, "Updated Project Name");
    assert_eq!(
        projects_updated[0].description,
        Some("Updated Description".to_string())
    );

    // 4. Create a task under this project
    let task = create_task(
        "Test Cascade Task".to_string(),
        None,
        None,
        Some(proj.id.clone()),
        None,
        db_state.clone(),
    )
    .expect("Failed to create task");

    let tasks_before = get_tasks_by_project(proj.id.clone(), None, None, db_state.clone())
        .expect("Failed to get tasks");
    assert_eq!(tasks_before.len(), 1);
    assert_eq!(tasks_before[0].id, task.id);

    // 5. Delete project (checks cascade delete of task under it)
    delete_project(proj.id.clone(), db_state.clone()).expect("Failed to delete project");

    let projects_after = get_projects(db_state.clone()).expect("Failed to get projects");
    assert!(
        projects_after.iter().all(|p| p.id != proj.id),
        "Project should be deleted"
    );

    // Tasks under project should be cascade deleted
    let tasks_after = get_tasks(None, None, db_state.clone()).expect("Failed to get all tasks");
    assert!(
        tasks_after
            .iter()
            .all(|t| t.project_id.as_ref() != Some(&proj.id)),
        "Cascaded tasks should be deleted"
    );
}

#[test]
fn test_task_operations_and_pagination() {
    let (app, db) = setup_test_app();
    let db_state = app.state::<Database>();

    // Create a series of tasks to test pagination
    for i in 1..=5 {
        create_task(
            format!("Task {}", i),
            Some(format!("Desc {}", i)),
            Some("high".to_string()),
            None,
            None,
            db_state.clone(),
        )
        .expect("Failed to create task");
    }

    // Test default list and pagination limits
    let tasks_all = get_tasks(None, None, db_state.clone()).expect("Failed to get tasks");
    assert_eq!(tasks_all.len(), 5);

    // Page 1 (limit 2)
    let tasks_page1 = get_tasks(Some(2), Some(0), db_state.clone()).expect("Failed to get page 1");
    assert_eq!(tasks_page1.len(), 2);

    // Page 2 (limit 2, offset 2)
    let tasks_page2 = get_tasks(Some(2), Some(2), db_state.clone()).expect("Failed to get page 2");
    assert_eq!(tasks_page2.len(), 2);
    assert_ne!(tasks_page1[0].id, tasks_page2[0].id);

    // Test updating task status
    let target_task = &tasks_all[0];
    update_task_status(target_task.id.clone(), "done".to_string(), db_state.clone())
        .expect("Failed to update status");

    let tasks_updated = get_tasks(None, None, db_state.clone()).expect("Failed to get tasks");
    let updated = tasks_updated
        .iter()
        .find(|t| t.id == target_task.id)
        .unwrap();
    assert_eq!(updated.status, app_lib::models::TaskStatus::Done);

    // Test delete task
    delete_task(target_task.id.clone(), db_state.clone()).expect("Failed to delete task");
    let tasks_final = get_tasks(None, None, db_state.clone()).expect("Failed to get tasks");
    assert_eq!(tasks_final.len(), 4);
    assert!(tasks_final.iter().all(|t| t.id != target_task.id));
}

#[test]
fn test_milestone_crud_and_status() {
    let (app, db) = setup_test_app();
    let db_state = app.state::<Database>();

    // 1. Create a milestone
    let ms = create_milestone(
        "Sprint 1".to_string(),
        Some("First sprint goals".to_string()),
        Some("2026-06-01".to_string()),
        None,
        db_state.clone(),
    )
    .expect("Failed to create milestone");

    assert_eq!(ms.title, "Sprint 1");
    assert_eq!(ms.description, Some("First sprint goals".to_string()));
    assert_eq!(ms.due_date, Some("2026-06-01".to_string()));
    assert_eq!(ms.status, app_lib::models::MilestoneStatus::Open);

    // 2. Read milestones
    let milestones =
        get_milestones(None, None, db_state.clone()).expect("Failed to get milestones");
    assert_eq!(milestones.len(), 1);
    assert_eq!(milestones[0].id, ms.id);

    // 3. Toggle/Update status
    update_milestone_status(ms.id.clone(), "closed".to_string(), db_state.clone())
        .expect("Failed to update status");

    let milestones_updated =
        get_milestones(None, None, db_state.clone()).expect("Failed to get milestones");
    assert_eq!(
        milestones_updated[0].status,
        app_lib::models::MilestoneStatus::Closed
    );

    // 4. Delete milestone
    delete_milestone(ms.id.clone(), db_state.clone()).expect("Failed to delete milestone");
    let milestones_final =
        get_milestones(None, None, db_state.clone()).expect("Failed to get milestones");
    assert!(milestones_final.is_empty());
}
