use elefren::status_builder::{NewStatus, StatusBuilder, Visibility};
use rss::Item;
use crate::configuration::StatusField;
use elefren::Language;

use thiserror::Error;
use anyhow::{Context, Result};

#[derive(Error, Debug)]
enum FormatError {
    #[error("Feed missing {0}")]
    MissingField(String),
    #[error("Error creating status {0}")]
    StatusError(String),
}

pub fn format_item(item: &Item, fields: &Vec<StatusField>, language: Language, visibility: Visibility) -> Result<NewStatus> {
    let mut field_strings = Vec::with_capacity(fields.len());
    for field in fields {
        match field {
            StatusField::Title => {
                let title = item.title().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(title);
            },
            StatusField::Link => {
                let link = item.link().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(link);
            },
            StatusField::Description => {
                let description = item.description().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(description);
            },
            StatusField::Author => {
                let author = item.author().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(author);
            },
            StatusField::Categories => {
                //let categories: Vec<String> = item.categories().iter().map(|category| category.name().to_owned()).collect();
                //field_strings.push(&categories.join(","));
            },
            StatusField::Content => {
                let content = item.content().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(content);
            },
            StatusField::Guid => {
                let guid = item.guid().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                let value = guid.value();
                field_strings.push(value);
            },
            StatusField::PubDate => {
                let pub_date = item.pub_date().ok_or_else(|| FormatError::MissingField(field.to_string()))?;
                field_strings.push(pub_date);
            },
        };

    }

    let content = field_strings.join("\n");

    StatusBuilder::new()
        .status(content)
        .visibility(visibility)
        .language(language)
        .build()
        .map_err(|e| FormatError::StatusError(e.to_string()))
        .context("Failed to build status")
}
