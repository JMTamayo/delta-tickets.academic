# From official Docker Rust Image:
	FROM rust:latest as builder

	# Setting working directory in builder:
	WORKDIR /app
	
	# Copying the application source code to the constructor:
	COPY ./back-users/src ./src
	COPY ./back-users/Cargo.toml ./Cargo.toml
	COPY ./lib-protos ./../lib-protos

	# Build the proyect:
	RUN cargo build --release
	
	# Deploying the application:
	FROM rust:latest

	# Copying app binary from builder:
	COPY --from=builder /app/target/release/back-users .
	
	# Run the application:
	CMD ["./back-users"]