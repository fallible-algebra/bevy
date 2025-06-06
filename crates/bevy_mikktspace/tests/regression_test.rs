#![expect(missing_docs, reason = "Not all docs are written yet, see #3492.")]
#![expect(
    clippy::bool_assert_comparison,
    clippy::semicolon_if_nothing_returned,
    clippy::useless_conversion,
    reason = "Crate auto-generated with many non-idiomatic decisions. See #7372 for details."
)]

use bevy_mikktspace::{generate_tangents, Geometry};
use glam::{Vec2, Vec3};

pub type Face = [u32; 3];

#[derive(Debug)]
struct Vertex {
    position: Vec3,
    normal: Vec3,
    tex_coord: Vec2,
}

#[derive(Debug, PartialEq)]
struct Result {
    tangent: [f32; 3],
    bi_tangent: [f32; 3],
    mag_s: f32,
    mag_t: f32,
    bi_tangent_preserves_orientation: bool,
    face: usize,
    vert: usize,
}

impl Result {
    fn new(
        tangent: [f32; 3],
        bi_tangent: [f32; 3],
        mag_s: f32,
        mag_t: f32,
        bi_tangent_preserves_orientation: bool,
        face: usize,
        vert: usize,
    ) -> Self {
        Self {
            tangent,
            bi_tangent,
            mag_s,
            mag_t,
            bi_tangent_preserves_orientation,
            face,
            vert,
        }
    }
}

struct Mesh {
    faces: Vec<Face>,
    vertices: Vec<Vertex>,
}

struct Context {
    mesh: Mesh,
    results: Vec<Result>,
}

fn vertex(mesh: &Mesh, face: usize, vert: usize) -> &Vertex {
    let vs: &[u32; 3] = &mesh.faces[face];
    &mesh.vertices[vs[vert] as usize]
}

impl Geometry for Context {
    fn num_faces(&self) -> usize {
        self.mesh.faces.len()
    }

    fn num_vertices_of_face(&self, _face: usize) -> usize {
        3
    }

    fn position(&self, face: usize, vert: usize) -> [f32; 3] {
        vertex(&self.mesh, face, vert).position.into()
    }

    fn normal(&self, face: usize, vert: usize) -> [f32; 3] {
        vertex(&self.mesh, face, vert).normal.into()
    }

    fn tex_coord(&self, face: usize, vert: usize) -> [f32; 2] {
        vertex(&self.mesh, face, vert).tex_coord.into()
    }

    fn set_tangent(
        &mut self,
        tangent: [f32; 3],
        bi_tangent: [f32; 3],
        mag_s: f32,
        mag_t: f32,
        bi_tangent_preserves_orientation: bool,
        face: usize,
        vert: usize,
    ) {
        self.results.push(Result {
            tangent,
            bi_tangent,
            mag_s,
            mag_t,
            bi_tangent_preserves_orientation,
            face,
            vert,
        })
    }
}

struct ControlPoint {
    uv: [f32; 2],
    dir: [f32; 3],
}

impl ControlPoint {
    fn new(uv: [f32; 2], dir: [f32; 3]) -> Self {
        Self { uv, dir }
    }
}

fn make_cube() -> Mesh {
    let mut faces = Vec::new();
    let mut ctl_pts = Vec::new();
    let mut vertices = Vec::new();

    // +x plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 1.0], [1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 0.0], [1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.5, 0.5], [1.0, 0.0, 0.0]));
    }

    // -x plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([1.0, 0.0], [-1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 1.0], [-1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [-1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [-1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.5, 0.5], [-1.0, 0.0, 0.0]));
    }

    // +y plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [-1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [-1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.5], [0.0, 1.0, 0.0]));
    }

    // -y plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [-1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [-1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.5], [0.0, -1.0, 0.0]));
    }

    // +z plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [-1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [-1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 1.0], [1.0, -1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 0.0], [1.0, 1.0, 1.0]));
        ctl_pts.push(ControlPoint::new([0.5, 0.5], [0.0, 0.0, 1.0]));
    }

    // -z plane
    {
        let base = ctl_pts.len() as u32;
        faces.push([base, base + 1, base + 4]);
        faces.push([base + 1, base + 2, base + 4]);
        faces.push([base + 2, base + 3, base + 4]);
        faces.push([base + 3, base, base + 4]);
        ctl_pts.push(ControlPoint::new([1.0, 0.0], [1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([1.0, 1.0], [1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 1.0], [-1.0, -1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.0, 0.0], [-1.0, 1.0, -1.0]));
        ctl_pts.push(ControlPoint::new([0.5, 0.5], [0.0, 0.0, -1.0]));
    }

    for pt in ctl_pts {
        let p: Vec3 = pt.dir.into();
        let n: Vec3 = p.normalize();
        let t: Vec2 = pt.uv.into();
        vertices.push(Vertex {
            position: (p / 2.0).into(),
            normal: n.into(),
            tex_coord: t.into(),
        });
    }

    Mesh { faces, vertices }
}

#[test]
fn cube_tangents_should_equal_reference_values() {
    let mut context = Context {
        mesh: make_cube(),
        results: Vec::new(),
    };
    let ret = generate_tangents(&mut context);
    assert_eq!(true, ret);

    let expected_results: Vec<Result> = vec![
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            0,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            0,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            false,
            0,
            2,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            1,
            0,
        ),
        Result::new(
            [-0.40824825, 0.81649655, 0.40824825],
            [-0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            1,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            false,
            1,
            2,
        ),
        Result::new(
            [-0.40824825, 0.81649655, 0.40824825],
            [-0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            2,
            0,
        ),
        Result::new(
            [-0.40824825, 0.81649655, -0.40824825],
            [0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            2,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            false,
            2,
            2,
        ),
        Result::new(
            [-0.40824825, 0.81649655, -0.40824825],
            [0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            3,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            3,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            false,
            3,
            2,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            4,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            4,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            true,
            4,
            2,
        ),
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            5,
            0,
        ),
        Result::new(
            [-0.40824825, 0.81649655, -0.40824825],
            [0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            5,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            true,
            5,
            2,
        ),
        Result::new(
            [-0.40824825, 0.81649655, -0.40824825],
            [0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            6,
            0,
        ),
        Result::new(
            [-0.40824825, 0.81649655, 0.40824825],
            [-0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            6,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            true,
            6,
            2,
        ),
        Result::new(
            [-0.40824825, 0.81649655, 0.40824825],
            [-0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            7,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            7,
            1,
        ),
        Result::new(
            [0.00000000, 1.00000000, 0.00000000],
            [0.00000000, 0.00000000, -1.00000000],
            1.00000000,
            1.00000000,
            true,
            7,
            2,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            8,
            0,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            8,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            8,
            2,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            9,
            0,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            9,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            9,
            2,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            10,
            0,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            10,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            10,
            2,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            11,
            0,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            11,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            11,
            2,
        ),
        Result::new(
            [-0.40824825, 0.81649655, 0.40824825],
            [-0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            12,
            0,
        ),
        Result::new(
            [-0.40824825, 0.81649655, -0.40824825],
            [0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            true,
            12,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            12,
            2,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            13,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            13,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            13,
            2,
        ),
        Result::new(
            [0.40824825, 0.81649655, -0.40824825],
            [-0.40824825, 0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            14,
            0,
        ),
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            14,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            14,
            2,
        ),
        Result::new(
            [0.40824825, 0.81649655, 0.40824825],
            [0.40824825, -0.40824825, -0.81649655],
            1.00000000,
            1.00000000,
            false,
            15,
            0,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            15,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, 1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            15,
            2,
        ),
        Result::new(
            [0.81649655, 0.40824825, 0.40824825],
            [-0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            false,
            16,
            0,
        ),
        Result::new(
            [0.81649655, -0.40824825, 0.40824825],
            [0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            false,
            16,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            16,
            2,
        ),
        Result::new(
            [0.81649655, -0.40824825, 0.40824825],
            [0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            false,
            17,
            0,
        ),
        Result::new(
            [0.81649655, 0.40824825, -0.40824825],
            [-0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            false,
            17,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            17,
            2,
        ),
        Result::new(
            [0.81649655, 0.40824825, -0.40824825],
            [-0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            false,
            18,
            0,
        ),
        Result::new(
            [0.81649655, -0.40824825, -0.40824825],
            [0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            false,
            18,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            18,
            2,
        ),
        Result::new(
            [0.81649655, -0.40824825, -0.40824825],
            [0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            false,
            19,
            0,
        ),
        Result::new(
            [0.81649655, 0.40824825, 0.40824825],
            [-0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            false,
            19,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            false,
            19,
            2,
        ),
        Result::new(
            [0.81649655, -0.40824825, 0.40824825],
            [0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            true,
            20,
            0,
        ),
        Result::new(
            [0.81649655, 0.40824825, 0.40824825],
            [-0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            true,
            20,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            true,
            20,
            2,
        ),
        Result::new(
            [0.81649655, 0.40824825, 0.40824825],
            [-0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            true,
            21,
            0,
        ),
        Result::new(
            [0.81649655, -0.40824825, -0.40824825],
            [0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            true,
            21,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            true,
            21,
            2,
        ),
        Result::new(
            [0.81649655, -0.40824825, -0.40824825],
            [0.40824825, -0.81649655, 0.40824825],
            1.00000000,
            1.00000000,
            true,
            22,
            0,
        ),
        Result::new(
            [0.81649655, 0.40824825, -0.40824825],
            [-0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            true,
            22,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            true,
            22,
            2,
        ),
        Result::new(
            [0.81649655, 0.40824825, -0.40824825],
            [-0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            true,
            23,
            0,
        ),
        Result::new(
            [0.81649655, -0.40824825, 0.40824825],
            [0.40824825, -0.81649655, -0.40824825],
            1.00000000,
            1.00000000,
            true,
            23,
            1,
        ),
        Result::new(
            [1.00000000, 0.00000000, 0.00000000],
            [0.00000000, -1.00000000, 0.00000000],
            1.00000000,
            1.00000000,
            true,
            23,
            2,
        ),
    ];

    assert_eq!(expected_results, context.results);
}
