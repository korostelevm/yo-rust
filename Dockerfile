FROM calavera/cargo-lambda
# FROM rust:1.58.1

# Install system dependencies
RUN apt-get update && \
    apt-get install -y libfontconfig1-dev libfreetype6-dev pkg-config cmake

# Create a new user
RUN useradd -m user
USER user

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files
COPY --chown=user:user . .

# Build the project
CMD cargo lambda build --release