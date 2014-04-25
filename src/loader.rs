use std::io::{BufferedReader, File};
use std::str::Words;
use gl::types::{GLfloat, GLuint};
use cgmath::vector::{Vector2, Vector3};
use collections::HashMap;

// Removed lifetimes
fn parse_vertex(mut words: Words) -> Vector3<GLfloat> {
    let w: Vec<&str> = words.collect();

    if w.len() != 3 { fail!("Invalid vertex count ({})", w); }

    let mut v = w.iter().map(|&x| from_str::<GLfloat>(x).unwrap());
    Vector3::new(v.next().unwrap(), v.next().unwrap(), v.next().unwrap())
}

fn parse_vertex2(mut words: Words) -> Vector2<GLfloat> {
    let w: Vec<&str> = words.collect();

    if w.len() != 2 { fail!("Invalid vertex count ({})", w); }

    let mut v = w.iter().map(|&x| from_str::<GLfloat>(x).unwrap());
    Vector2::new(v.next().unwrap(), 1.0f32 - v.next().unwrap())
}

fn parse_face(mut words: Words, indices: &mut Vec<Vector3<GLuint>>) {
    let w: Vec<&str> = words.collect();

    if w.len() != 3 { fail!("Non-triangular faces not support"); }

    for word in w.iter() {
        let parts: Vec<GLuint> = word.split('/').map(|x| from_str::<GLuint>(x).map(|x| x-1).unwrap_or(0)).collect();

        indices.push(Vector3::new(*parts.get(0), *parts.get(1), *parts.get(2)));
    }
}

pub fn load_obj(path: &str) -> (Vec<Vector3<GLfloat>>, Vec<Vector3<GLfloat>>, Vec<Vector2<GLfloat>>, Vec<GLuint>) {
    let file = File::open(&Path::new(path));

    debug!("Loading obj file: {}", path);

    match file {
        Err(ref err) => {
            error!("{}: {}", path, err.desc);
            return (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        }
        Ok(_) => {}
    }

    let mut reader = BufferedReader::new(file);

    let mut verts: Vec<Vector3<GLfloat>> = Vec::new();
    let mut norms: Vec<Vector3<GLfloat>> = Vec::new();
    let mut uvs: Vec<Vector2<GLfloat>> = Vec::new();
    let mut indices: Vec<Vector3<GLuint>> = Vec::new();

    let mut fverts: Vec<Vector3<GLfloat>> = Vec::new();
    let mut fnorms: Vec<Vector3<GLfloat>> = Vec::new();
    let mut fuvs: Vec<Vector2<GLfloat>> = Vec::new();
    let mut findices: Vec<GLuint> = Vec::new();
    let mut points = HashMap::<Vector3<GLuint>, GLuint>::new();

    for line in reader.lines().map(|x| x.unwrap().trim_right().to_owned()) {
        let mut words = line.words();

        let tag = words.next();
        match tag {
            None => { },
            Some(w) => {
                if w.len() != 0 && w[0] != ('#' as u8) {
                    match w {
                        "v" => { verts.push(parse_vertex(words)); },
                        "vn" => { norms.push(parse_vertex(words)); },
                        "vt" => { uvs.push(parse_vertex2(words)); },
                        "f" => { parse_face(words, &mut indices); },
                        _ => { warn!(r#"Ignoring invalid string "{}""#, line); },
                    }
                }
            }
        }
    }

    for point in indices.move_iter() {
        let idx = match points.find(&point) {
            Some(x) => {
                findices.push(*x);
                None
            }
            None => {
                let idx = fverts.len() as u32;

                fverts.push(*verts.get(point.x as uint));
                fnorms.push(*norms.get(point.z as uint));
                fuvs.push(*uvs.get(point.y as uint));

                findices.push(idx);
                Some(idx)
            }
        };
        idx.map(|x| points.insert(point, x));
    }

    (fverts, fnorms, fuvs, findices)
}
