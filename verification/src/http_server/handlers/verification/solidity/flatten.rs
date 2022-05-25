use super::types::{FlattenedSource, VerificationRequest, VerificationResponse};
use crate::solidity::compiler_cache::CompilerCache;
use actix_web::{
    error,
    web::{self, Json},
    Error,
};
use ethers_solc::{CompilerInput, CompilerOutput};
use semver::Version;

pub async fn compile(
    cache: &CompilerCache,
    compiler_version: &str,
    input: &CompilerInput,
) -> Result<CompilerOutput, Error> {
    let ver = Version::parse(compiler_version).map_err(error::ErrorBadRequest)?;
    let solc = cache
        .get(&ver)
        .await
        .map_err(error::ErrorInternalServerError)?;
    solc.compile(&input)
        .map_err(error::ErrorInternalServerError)
}

pub async fn verify(
    cache: web::Data<CompilerCache>,
    params: Json<VerificationRequest<FlattenedSource>>,
) -> Result<Json<VerificationResponse>, Error> {
    let params = params.into_inner();

    let input = CompilerInput::from(params.content);
    let output = compile(&cache, &params.compiler_version, &input).await?;
    // TODO: verify output
    let _ = output;

    Ok(Json(VerificationResponse { verified: true }))
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn compile() {}
}
