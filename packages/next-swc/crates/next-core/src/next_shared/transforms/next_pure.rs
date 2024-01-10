use anyhow::Result;
use async_trait::async_trait;
use turbopack_binding::{
    swc::core::{common::util::take::Take, ecma::ast::*},
    turbopack::ecmascript::{CustomTransformer, EcmascriptInputTransform, TransformContext},
};

#[derive(Debug)]
struct NextPure {}

#[async_trait]
impl CustomTransformer for NextPure {
    async fn transform(&self, program: &mut Program, _ctx: &TransformContext<'_>) -> Result<()> {
        let p = std::mem::replace(program, Program::Module(Module::dummy()));

        *program = p;
        Ok(())
    }
}
