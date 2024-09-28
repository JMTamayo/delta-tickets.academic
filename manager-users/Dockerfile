# From official Docker Rust Image:
	FROM rust:latest as builder

	# Setting working directory in builder:
	WORKDIR /app
	
	# Copying the application source code to the constructor:
	COPY ./manager-users/src ./src
	COPY ./manager-users/Cargo.toml ./Cargo.toml
	COPY ./lib-protos ./../lib-protos

	# Build the proyect:
	RUN cargo build --release
	
	# Deploying the application:
	FROM rust:latest

	# Copying app binary from builder:
	COPY --from=builder /app/target/release/delta-tickets-users-manager .
	COPY --from=builder /app/src ./src
	
	# Run the application:
	CMD ["./delta-tickets-users-manager"]