mig-gen:
	diesel migration generate --diff-schema=src/tables.rs $(name)

mig-run:
	diesel migration run


#run:
	#cargo run -- --config-path=configs/dev.toml --secret-path=secrets

run:
	cargo watch -x 'run -- --config-path=configs/dev.toml --secret-path=secrets'