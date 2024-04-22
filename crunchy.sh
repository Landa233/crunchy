#!/bin/bash

# Request resources:
#SBATCH -c 48
#SBATCH --time=0-0:14:0
#SBATCH --mem=50G
#SBATCH --tmp=100G
#SBATCH --mail-user=jhtb65@durham.ac.uk
#SBATCH --mail-type=ALL
#SBATCH -p test


#Commands to be run:
./target/release/examples/run_sweep configs/phase_diagram.json