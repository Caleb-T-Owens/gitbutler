use anyhow::Context;
use tauri::{AppHandle, Manager};
use timed::timed;

use crate::{error::Error, project_repository::branch};

use super::controller::Controller;

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn commit_virtual_branch(
    handle: AppHandle,
    project_id: &str,
    branch: &str,
    message: &str,
) -> Result<(), Error> {
    handle
        .state::<Controller>()
        .create_commit(project_id, branch, message)
        .await
        .context("failed to create commit")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn list_virtual_branches(
    handle: AppHandle,
    project_id: &str,
) -> Result<Vec<super::VirtualBranch>, Error> {
    let branches = handle
        .state::<Controller>()
        .list_virtual_branches(project_id)
        .await
        .context("failed to list virtual branches")?;
    Ok(branches)
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn create_virtual_branch(
    handle: AppHandle,
    project_id: &str,
    branch: super::branch::BranchCreateRequest,
) -> Result<(), Error> {
    handle
        .state::<Controller>()
        .create_virtual_branch(project_id, &branch)
        .await
        .context("failed to create virtual branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn create_virtual_branch_from_branch(
    handle: AppHandle,
    project_id: &str,
    branch: branch::Name,
) -> Result<String, Error> {
    let branch_id = handle
        .state::<Controller>()
        .create_virtual_branch_from_branch(project_id, &branch)
        .await
        .context("failed to create virtual branch from branch")?;
    Ok(branch_id)
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn get_base_branch_data(
    handle: AppHandle,
    project_id: &str,
) -> Result<Option<super::BaseBranch>, Error> {
    let target = handle
        .state::<Controller>()
        .get_base_branch_data(project_id)
        .await
        .context("failed to get base branch data")?;

    Ok(target)
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn set_base_branch(
    handle: AppHandle,
    project_id: &str,
    branch: &str,
) -> Result<super::BaseBranch, Error> {
    let controller = handle.state::<Controller>();
    let target = controller
        .set_base_branch(project_id, branch)
        .await
        .context("failed to get target data")?;
    Ok(target)
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn update_base_branch(handle: AppHandle, project_id: &str) -> Result<(), Error> {
    let controller = handle.state::<Controller>();
    controller
        .update_base_branch(project_id)
        .await
        .context("failed to update base branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn update_virtual_branch(
    handle: AppHandle,
    project_id: &str,
    branch: super::branch::BranchUpdateRequest,
) -> Result<(), Error> {
    let controller = handle.state::<Controller>();
    controller
        .update_virtual_branch(project_id, branch)
        .await
        .context("failed to update virtual branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn delete_virtual_branch(
    handle: AppHandle,
    project_id: &str,
    branch_id: &str,
) -> Result<(), Error> {
    let controller = handle.state::<Controller>();
    controller
        .delete_virtual_branch(project_id, branch_id)
        .await
        .context("failed to update virtual branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn apply_branch(handle: AppHandle, project_id: &str, branch: &str) -> Result<(), Error> {
    let controller = handle.state::<Controller>();
    controller
        .apply_virtual_branch(project_id, branch)
        .await
        .context("failed to apply branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn unapply_branch(
    handle: AppHandle,
    project_id: &str,
    branch: &str,
) -> Result<(), Error> {
    let controller = handle.state::<Controller>();
    controller
        .unapply_virtual_branch(project_id, branch)
        .await
        .context("failed to unapply branch")?;
    Ok(())
}

#[timed(duration(printer = "debug!"))]
#[tauri::command(async)]
pub async fn push_virtual_branch(
    handle: AppHandle,
    project_id: &str,
    branch_id: &str,
) -> Result<(), Error> {
    handle
        .state::<Controller>()
        .push_virtual_branch(project_id, branch_id)
        .await?;
    Ok(())
}