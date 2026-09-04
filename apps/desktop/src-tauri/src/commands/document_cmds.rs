use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::document::{
    upload_document::UploadDocumentUseCase,
    delete_document::DeleteDocumentUseCase,
};
use orbis_app::context::RequestContext;
use uuid::Uuid;

#[tauri::command]
pub async fn upload_document(
    org_id: String,
    uploaded_by: Option<String>,
    entity_type: Option<String>,
    entity_id: Option<String>,
    name: String,
    description: Option<String>,
    category: String,
    asset_type: String,
    school_year: Option<String>,
    term: Option<String>,
    tags: Option<String>,
    visibility: String,
    size: Option<i32>,
    mime_type: Option<String>,
    extension: Option<String>,
    storage_provider: String,
    path: String,
    url: Option<String>,
    thumbnail_path: Option<String>,
    preview_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = Uuid::parse_str(&org_id).map_err(|_| "Invalid org ID")?;
    let parsed_uploaded_by = match uploaded_by {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid uploaded_by ID")?),
        None => None,
    };
    let parsed_entity_id = match entity_id {
        Some(id) => Some(Uuid::parse_str(&id).map_err(|_| "Invalid entity ID")?),
        None => None,
    };

    let ctx = RequestContext { tenant_id: parsed_org_id, actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    
    let usecase = UploadDocumentUseCase { 
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    
    match usecase.execute(
        &ctx, parsed_org_id, parsed_uploaded_by, entity_type, parsed_entity_id, 
        name, description, category, asset_type, school_year, term, tags, 
        visibility, size, mime_type, extension, storage_provider, path, url, 
        thumbnail_path, preview_path
    ).await {
        Ok(doc) => Ok(doc.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn delete_document(
    document_id: String,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    let parsed_document_id = Uuid::parse_str(&document_id).map_err(|_| "Invalid document ID")?;
    
    let ctx = RequestContext { tenant_id: Uuid::new_v4(), actor: orbis_domain::iam::actor::Actor::System, request_id: Uuid::new_v4().to_string() };
    
    let usecase = DeleteDocumentUseCase { 
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone()
    };
    
    match usecase.execute(&ctx, parsed_document_id).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
