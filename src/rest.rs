use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
};

mod parser;
mod restfile;
use restfile::RestFile;

mod method;
use method::RestMethod;

mod request;
use request::RestRequest;

mod response;
use response::RestResponse;

use tera::Context;

pub(crate) async fn index(
    selected: Option<Path<usize>>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Html<String>, StatusCode> {
    tracing::info!("rest index, selected: {:?}", selected);
    let rest_file = parse_rest_file(&state.opt.file).await?;

    let mut ctx = Context::new();
    ctx.insert("requests", &rest_file.requests);

    if let Some(Path(selected)) = selected {
        ctx.insert("selected", &selected);
    }
    let html = state.tera.render("index.html", &ctx).map_err(|e| {
        tracing::error!("failed to render index.html: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(html))
}

pub(crate) async fn response(
    Path(selected): Path<usize>,
    State(state): State<Arc<crate::AppState>>,
) -> Result<Html<String>, StatusCode> {
    // parse rest file and fetch response
    let rest_file = parse_rest_file(&state.opt.file).await?;
    let response = &rest_file.requests[selected]
        .get_response()
        .await
        .map_err(|e| {
            tracing::error!("failed to render index.html: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // render response
    tracing::info!("rendering response");

    let mut ctx = Context::new();
    ctx.insert("selected", &selected);
    ctx.insert("requests", &rest_file.requests);
    ctx.insert("response", &response);
    let html = state.tera.render("index.html", &ctx).map_err(|e| {
        tracing::error!("failed to render index.html: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(html))
}

async fn parse_rest_file(file_name: &str) -> Result<RestFile, StatusCode> {
    tracing::info!("parsing rest-file '{}'", file_name);
    let content =
        std::fs::read_to_string(file_name).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let rest_file = RestFile::try_from(content.as_str()).map_err(|e| {
        tracing::error!("cannot parse file '{}': {}", file_name, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(rest_file)
}
