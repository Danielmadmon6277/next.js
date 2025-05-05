/// Defines code generation for compiler supported values that are relative to
/// the output directory.
///
/// Currently this just supports the `__dirname` constant, though `__filename` could be added
/// as well.
///
/// `import.meta` has similar semantics but doesn't share this solution since there is a need
/// to maintain a module level singleton value.
use anyhow::Result;
use serde::{Deserialize, Serialize};
use turbo_tasks::{debug::ValueDebugFormat, trace::TraceRawVcs, NonLocalValue, Vc};
use turbopack_core::{
    chunk::ChunkingContext, compile_time_info::OutputRelativeConstant, module_graph::ModuleGraph,
};

use crate::{
    code_gen::{CodeGen, CodeGeneration},
    create_visitor,
    references::AstPath,
};

#[derive(PartialEq, Eq, Serialize, Deserialize, TraceRawVcs, ValueDebugFormat, NonLocalValue)]
pub struct OutputRelative {
    path: AstPath,
    kind: OutputRelativeConstant,
}

impl OutputRelative {
    pub fn new(path: AstPath, kind: OutputRelativeConstant) -> Self {
        Self { path, kind }
    }

    pub async fn code_generation(
        &self,
        _module_graph: Vc<ModuleGraph>,
        chunking_context: Vc<Box<dyn ChunkingContext>>,
    ) -> Result<CodeGeneration> {
        let kind = self.kind;
        // All chunks are placed in a flat output directory
        let path = chunking_context.chunk_root_path().await?.path.clone();
        let path = format!("/ROOT/{path}");
        let visitor = create_visitor!(self.path, visit_mut_expr(expr: &mut Expr) {
            *expr = match kind {
                OutputRelativeConstant::DirName => path.to_string().into(),
            }
        });

        Ok(CodeGeneration::visitors(vec![visitor]))
    }
}

impl From<OutputRelative> for CodeGen {
    fn from(val: OutputRelative) -> Self {
        CodeGen::OutputRelativeCodeGen(val)
    }
}
