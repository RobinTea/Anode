use docx::{Document, Paragraph, Section, Table, TableCell, TableRow, Text, Run};
use docx::shared::{Inches, Pt, AlignmentType};
use serde_json::Value;
use std::path::Path;
use uuid::Uuid;

use crate::{Result, AnodeError};
use crate::paths::book_dir;
use crate::BookService;

pub fn compile_to_docx(
    library: &Path,
    book_id: Uuid,
    include_snapshots: bool,
    output_path: &Path,
) -> Result<()> {
    // Load book metadata
    let meta = BookService::load_meta(library, book_id)?;
    
    // Load compile order and pages
    let book_path = book_dir(library, book_id);
    let compile_order = crate::PageService::compile_order(library, book_id)?;
    let pages = crate::PageService::list(library, book_id)?;
    
    // Create document
    let mut doc = Document::new();
    
    // Add title page
    doc = doc.push(
        Section::new()
            .push(Paragraph::new(Text::new(meta.title.clone()).bold()))
            .push(Paragraph::new(Text::new(meta.author.clone())))
            .push(Paragraph::new(Text::new(meta.genre.clone())))
            .push(Paragraph::new(""))
            .push(Paragraph::new(Text::new(meta.synopsis.clone()).italic()))
    );
    
    // Add pages in compile order
    for entry in compile_order {
        if !entry.included {
            continue;
        }
        
        // Find the page metadata
        if let Some(page) = pages.iter().find(|p| p.id == entry.page_id) {
            // Load page body
            let body = crate::PageService::load_body(library, book_id, page.id)?;
            
            // Add page break
            doc = doc.push(Paragraph::new("").page_break());
            
            // Add title
            doc = doc.push(
                Paragraph::new(Text::new(page.title.clone()).bold())
                    .with_alignment(AlignmentType::Center)
            );
            
            // Add content
            doc = add_content_to_doc(doc, &body.doc)?;
        }
    }
    
    // Write document
    doc.build().pack(output_path)
        .map_err(|e| AnodeError::msg(format!("Failed to build DOCX: {}", e)))?;
    
    Ok(())
}

fn add_content_to_doc(mut doc: Document, node: &Value) -> Result<Document> {
    match node {
        Value::Object(obj) => {
            if let Some(node_type) = obj.get("type").and_then(|v| v.as_str()) {
                match node_type {
                    "doc" => {
                        if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
                            for child in content {
                                doc = add_content_to_doc(doc, child)?;
                            }
                        }
                    }
                    "paragraph" => {
                        let text = extract_text(node);
                        doc = doc.push(Paragraph::new(text));
                    }
                    "heading" => {
                        let level = obj.get("attrs")
                            .and_then(|a| a.get("level"))
                            .and_then(|l| l.as_u64())
                            .unwrap_or(1) as u32;
                        let text = extract_text(node);
                        let mut para = Paragraph::new(Text::new(text).bold());
                        if level > 0 {
                            para = para;
                        }
                        doc = doc.push(para);
                    }
                    "codeBlock" => {
                        let text = extract_text(node);
                        doc = doc.push(Paragraph::new(text).style("code"));
                    }
                    "orderedList" | "bulletList" => {
                        if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
                            for item in content {
                                doc = add_list_item_to_doc(doc, item, node_type == "orderedList")?;
                            }
                        }
                    }
                    "blockquote" => {
                        if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
                            for child in content {
                                let text = extract_text(child);
                                doc = doc.push(Paragraph::new(format!("> {}", text)).style("Quote"));
                            }
                        }
                    }
                    "table" => {
                        if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
                            let mut rows = Vec::new();
                            for row_node in content {
                                if let Some(row_obj) = row_node.as_object() {
                                    if let Some(cells_arr) = row_obj.get("content").and_then(|v| v.as_array()) {
                                        let mut cells = Vec::new();
                                        for cell_node in cells_arr {
                                            let cell_text = extract_text(cell_node);
                                            cells.push(TableCell::new().add_paragraph(Paragraph::new(cell_text)));
                                        }
                                        rows.push(TableRow::new(cells));
                                    }
                                }
                            }
                            if !rows.is_empty() {
                                doc = doc.push(Table::new(rows));
                            }
                        }
                    }
                    "hardBreak" => {
                        doc = doc.push(Paragraph::new(""));
                    }
                    "horizontalRule" => {
                        doc = doc.push(Paragraph::new("―――――――――"));
                    }
                    _ => {}
                }
            }
        }
        Value::Array(arr) => {
            for item in arr {
                doc = add_content_to_doc(doc, item)?;
            }
        }
        _ => {}
    }
    Ok(doc)
}

fn add_list_item_to_doc(mut doc: Document, node: &Value, ordered: bool) -> Result<Document> {
    if let Some(obj) = node.as_object() {
        if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
            let mut text = String::new();
            for child in content {
                text.push_str(&extract_text(child));
            }
            let prefix = if ordered { "• " } else { "- " };
            doc = doc.push(Paragraph::new(format!("{}{}", prefix, text)));
        }
    }
    Ok(doc)
}

fn extract_text(node: &Value) -> String {
    match node {
        Value::Object(obj) => {
            let mut text = String::new();
            
            // Get direct text
            if let Some(t) = obj.get("text").and_then(|v| v.as_str()) {
                text.push_str(t);
            }
            
            // Recursively get text from children
            if let Some(content) = obj.get("content").and_then(|v| v.as_array()) {
                for child in content {
                    text.push_str(&extract_text(child));
                }
            }
            
            text
        }
        Value::Array(arr) => {
            let mut text = String::new();
            for item in arr {
                text.push_str(&extract_text(item));
            }
            text
        }
        Value::String(s) => s.clone(),
        _ => String::new(),
    }
}
