#! /bin/bash
# Install Coral for Linux

# The current directory that we're in
ROOT_DIR=`dirname $0`

# Use Cargo to build Coral
echo "Builduing Coral"
cd $ROOT_DIR
cargo build

# Create the bash script that will run Coral
EXECUTABLE="${HOME}/.local/bin/coral"
touch EXECUTABLE
echo "Creating shell script at ${EXECUTABLE}"
echo "${ROOT_DIR}/target/debug/coral \"\$@\"" > ${EXECUTABLE}
chmod +x $EXECUTABLE
