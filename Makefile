.PHONY: deploy-all-bb

deploy-all-bb:
	scp target/release/bb-gamebot-rs bb:/home/nordborn/bb/
