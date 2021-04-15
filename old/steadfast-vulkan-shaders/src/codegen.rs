use proc_macro2::{Span, TokenStream};
use shaderc::{CompilationArtifact, CompileOptions, Compiler, ShaderKind};
use syn::{Error, Ident};

// Target version 1.1
const VULKAN_TARGET_VERSION: u32 = (1 << 22) | (1 << 12);

pub fn compile(code: &str, kind: ShaderKind) -> Result<CompilationArtifact, String> {
    let mut compiler = Compiler::new().ok_or("Failed to create GLSL compiler")?;
    let mut options = CompileOptions::new().ok_or("Failed to initialize GLSL compiler context")?;

    let content = compiler
        .compile_into_spirv(&code, kind, "shader.glsl", "main", Some(&options))
        .map_err(|e| e.to_string())?;

    Ok(content)
}

pub fn reflect(name: &str, spirv: &[u32]) -> Result<TokenStream, Error> {
    let struct_name = Ident::new(&name, Span::call_site());

    let ast = quote! {};

    Ok(ast)
}
