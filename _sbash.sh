#!/bin/bash
 
# Request resources:
#SBATCH -c 128          # 1 CPU core
#SBATCH --mem=50G       # memory required, up to 250G on standard nodes.
#SBATCH --time=5:10:0   # time limit for job (format:  days-hours:minutes:seconds)
#SBATCH --gres=tmp:100G  # temporary disk space required on the compute node ($TMPDIR),
#                        up to 400G
#SBATCH --mail-user=jhtb65@durham.ac.uk
#SBATCH --mail-type=ALL
#SBATCH -p shared
 
# Commands to be run:
./target/release/examples/phase_diagram_test