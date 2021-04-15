#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod codegen;

use crate::codegen::{compile, reflect};
use proc_macro::TokenStream;
use shaderc::ShaderKind;
use syn::parse::{Parse, ParseStream};
use syn::Error;
use syn::Token;
use syn::{parse_macro_input, Ident, LitStr};

enum SourceKind {
    Raw(String),
}

struct ShaderInput {
    shader_kind: ShaderKind,
    source_kind: SourceKind,
}

impl Parse for ShaderInput {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let mut shader_kind = None;
        let mut source_kind = None;

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;

            match name.to_string().as_ref() {
                "kind" => {
                    if shader_kind.is_some() {
                        panic!("A shader `kind` may only be defined once");
                    }

                    let kind: LitStr = input.parse()?;
                    let kind = match kind.value().as_ref() {
                        "vertex" => ShaderKind::Vertex,
                        "tess_control" => ShaderKind::TessControl,
                        "tess_evaluation" => ShaderKind::TessEvaluation,
                        "geometry" => ShaderKind::Geometry,
                        "fragment" => ShaderKind::Fragment,
                        "compute" => ShaderKind::Compute,
                        _ => panic!(
                            "Invalid shader `kind`. Valid values: vertex, tess_control, tess_evaluation, geometry, fragment, compute"
                        ),
                    };

                    shader_kind = Some(kind);
                }
                "src" => {
                    if source_kind.is_some() {
                        panic!("A shader `src` may only be defined once")
                    }

                    let src: LitStr = input.parse()?;

                    source_kind = Some(SourceKind::Raw(src.value()));
                }
                name => panic!(format!("Unknown field name: {}", name)),
            }

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }
        }

        let shader_kind = match shader_kind {
            Some(shader_kind) => shader_kind,
            None => panic!("No shader `kind` provided. Valid values: vertex, tess_control, tess_evaluation, geometry, fragment, compute"),
        };

        let source_kind = match source_kind {
            Some(source_kind) => source_kind,
            None => panic!("No shader source provided. Valid sources: src"),
        };

        Ok(ShaderInput { shader_kind, source_kind })
    }
}

#[proc_macro]
pub fn shader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ShaderInput);

    let source_code = match input.source_kind {
        SourceKind::Raw(source) => source,
        _ => panic!("Invalid source kind"),
    };

    let spirv = match compile(&source_code, input.shader_kind) {
        Ok(ok) => ok,
        Err(e) => panic!(e),
    };

    reflect("Shader", spirv.as_binary()).unwrap().into()
}
