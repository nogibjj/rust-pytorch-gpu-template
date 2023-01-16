FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld lshw \
   && apt-get autoremove -y && apt-get clean -y

#manully install cuda 11.7
RUN wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/cuda-ubuntu1804.pin \
&& mv cuda-ubuntu1804.pin /etc/apt/preferences.d/cuda-repository-pin-600 \
&& wget https://developer.download.nvidia.com/compute/cuda/11.7.0/local_installers/cuda-repo-ubuntu1804-11-7-local_11.7.0-515.43.04-1_amd64.deb \
&& dpkg -i cuda-repo-ubuntu1804-11-7-local_11.7.0-515.43.04-1_amd64.deb \
&& cp /var/cuda-repo-ubuntu1804-11-7-local/cuda-*-keyring.gpg /usr/share/keyrings/ \

RUN apt-get update && apt-get -y install --no-install-recommends \
   cuda \
   ffmpeg \ 
   gcc \
   pciutils \
   && apt-get autoremove -y && apt-get clean -y 