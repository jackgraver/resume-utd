# Rust + Debian Bookworm
FROM rust:bookworm

# Install LaTeX engine + fonts + utilities
RUN apt-get update && apt-get install -y \
    texlive-xetex \
    texlive-latex-recommended \
    texlive-latex-extra \
    texlive-fonts-recommended \
    texlive-fonts-extra \
    fonts-texgyre \
    libfontconfig1 \
    zlib1g \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY templates/ ./templates/
COPY data/ ./data/

# Build Rust program
RUN cargo build --release

# Create output directory
RUN mkdir -p /app/output

# Default command: run Rust generator and compile PDF
CMD ["bash", "-c", "./target/release/resume-gen && xelatex -output-directory=output output/resume.tex"]
