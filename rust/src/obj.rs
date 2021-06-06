use std::fs::File;

use crate::errors::SceneIOError;
use crate::{MeshGeometry, Face, FaceElement, VertexNormal, Vertex};
use std::io::{BufReader, BufRead};

pub fn read_obj_file(path: &str) -> Result<MeshGeometry, SceneIOError> {
    let mut faces = Vec::new();
    let texture_coordinates = Vec::new();
    let mut vertex_normals = Vec::new();
    let mut vertices = Vec::new();

    let file = File::open(path).map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to open obj file: {}", err.to_string()),
    })?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(v) => v,
            Err(_) => continue,
        };

        let spl: Vec<&str> = line.split(" ").collect();
        match spl[0] {
            "#" => {
                // ignore comments
            },
            "v" => vertices.push(parse_vertex(&spl[1..])?),
            "vn" => vertex_normals.push(parse_vertex_normal(&spl[1..])?),
            "f" => faces.push(parse_face(&spl[1..])?),
            "g" => {
                // ignore
            },
            "usemtl" | "mtllib" => {
                // ignore materials at the moment ...
            },
            "o" | "s" => {
                // ignore...
            },
            other => {
                warn!("Unknown line type in obj: {}, ignoring...", other);
            }
        }
    }

    Ok(MeshGeometry {
        faces,
        texture_coordinates,
        vertex_normals,
        vertices,
    })
}

fn parse_vertex(parts: &[&str]) -> Result<Vertex, SceneIOError> {
    let x = parts[0].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex x: {}", err),
    })?;
    let y = parts[1].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex y: {}", err),
    })?;
    let z = parts[2].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex z: {}", err),
    })?;
    let w = if parts.len() >= 4 {
        parts[3].parse().map_err(|err| SceneIOError::FailedToReadObj {
            description: format!("Failed to parse vertex w: {}", err),
        })?
    } else {
        1.0
    };

    Ok(Vertex {
        x,
        y,
        z,
        w,
    })
}

fn parse_vertex_normal(parts: &[&str]) -> Result<VertexNormal, SceneIOError> {
    let x = parts[0].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex normal x: {}", err),
    })?;
    let y = parts[1].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex normal y: {}", err),
    })?;
    let z = parts[2].parse().map_err(|err| SceneIOError::FailedToReadObj {
        description: format!("Failed to parse vertex normal z: {}", err),
    })?;

    Ok(VertexNormal {
        x,
        y,
        z,
    })
}

fn parse_face(parts: &[&str]) -> Result<Face, SceneIOError> {
    let mut elements = Vec::new();

    for part in parts {
        elements.push(parse_face_element(part)?);
    }

    Ok(Face {
        elements,
    })
}

fn parse_face_element(part: &str) -> Result<FaceElement, SceneIOError> {
    let spl: Vec<&str> = part.split("/").collect();
    let vertex_index = if spl[0] == "" { 0 } else {
        match spl[0].parse() {
            Ok(v) => v,
            Err(err) => return Err(SceneIOError::FailedToReadObj {
                description: format!("Failed to parse vertex index: {}", err),
            })
        }
    };
    let texture_index = if spl[1] == "" { 0 } else {
        match spl[1].parse() {
            Ok(v) => v,
            Err(err) => return Err(SceneIOError::FailedToReadObj {
                description: format!("Failed to parse texture index: {}", err),
            })
        }
    };
    let normal_index = if spl[2] == "" { 0 } else {
        match spl[2].parse() {
            Ok(v) => v,
            Err(err) => return Err(SceneIOError::FailedToReadObj {
                description: format!("Failed to parse normal index: {}", err),
            })
        }
    };

    Ok(FaceElement {
        vertex_index,
        texture_index,
        normal_index,
    })
}