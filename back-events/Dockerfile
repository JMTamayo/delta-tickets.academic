# From official Docker Rust Image:
	FROM rust:latest as builder

	# Setting working directory in builder:
	WORKDIR /app
	
	# Copying the application source code to the constructor:
	COPY ./auth/src ./src
	COPY ./auth/Cargo.toml ./Cargo.toml
	COPY ./lib-protos ./../lib-protos

	# Build the proyect:
	RUN cargo build --release
	
	# Deploying the application:
	FROM rust:latest

	# Copying app binary from builder:
	COPY --from=builder /app/target/release/auth-service .
	
	# Run the application:
	CMD ["./auth-service"]