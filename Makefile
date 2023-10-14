.PHONY: deploy-all-bb deploy-service-unit-bb

deploy-all-bb:
	scp target/release/bb-gamebot-rs bb:/home/nordborn/bb/

deploy-service-unit-bb:
	scp services/bb_gamebot_rs.service bb:/home/nordborn/bb/
