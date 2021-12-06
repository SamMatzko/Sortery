#! /bin/bash
# Install Sortery for Linux

# The current directory that we're in
ROOT_DIR=`dirname $0`

# Use Cargo to build Sortery
echo "Builduing Sortery"
cd $ROOT_DIR
cargo build

# Create the bash script that will run Sortery
EXECUTABLE="${HOME}/.local/bin/sortery"
touch EXECUTABLE
echo "Creating shell script at ${EXECUTABLE}"
echo "${ROOT_DIR}/target/debug/sortery \"\$@\"" > ${EXECUTABLE}
chmod +x $EXECUTABLE
