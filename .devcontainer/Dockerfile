FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld lshw \
   && apt-get autoremove -y && apt-get clean -y

RUN apt-get update \
&& apt-get install -y nvidia-docker2 \
&& systemctl restart docker

RUN wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/cuda-ubuntu1804.pin \
&& mv cuda-ubuntu1804.pin /etc/apt/preferences.d/cuda-repository-pin-600 \
&& apt-key adv --fetch-keys https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/3bf863cc.pub \
&& add-apt-repository "deb https://developer.download.nvidia.com/compute/cuda/repos/ubuntu1804/x86_64/ /" \
&& apt-get update \
&& apt-get -y install cuda

RUN apt-get update && apt-get -y install --no-install-recommends \
   ffmpeg \ 
   gcc \
   pciutils \
   
   && apt-get autoremove -y && apt-get clean -y \

