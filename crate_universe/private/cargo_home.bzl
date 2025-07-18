""" A repository rule to normalize the symlinking of CARGO_HOME between module_ctx and repository_ctx """

def _cargo_home_rule_impl(repository_ctx):
    cargo_home = repository_ctx.path(".cargo_home")
    if repository_ctx.attr.cargo_config:
        cargo_home_config = repository_ctx.path("{}/config.toml".format(cargo_home))
        cargo_config = repository_ctx.path(repository_ctx.attr.cargo_config)
        repository_ctx.symlink(cargo_config, cargo_home_config)

    if repository_ctx.attr.cargo_credentials:
        cargo_home_credentials = repository_ctx.path("{}/credentials.toml".format(cargo_home))
        cargo_credentials = repository_ctx.path(repository_ctx.attr.cargo_credentials, cargo_home_credentials)
        repository_ctx.symlink(cargo_config, cargo_credentials)

cargo_home = repository_rule(
    implementation = _cargo_home_rule_impl,
    attrs = {
        "cargo_config": attr.label(mandatory = False),
        "cargo_credentials": attr.label(mandatory = False),
    },
)
